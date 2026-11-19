# Testing in Rust — how working engineers actually do it

This guide is a companion to [PRACTICE.md](PRACTICE.md) and the
[roadmap](../ROADMAP.md). It starts from "what is a test" and ends at the
patterns used in real production crates, with examples drawn from *this*
workspace (`crates/dsa`, `crates/resp`).

How to use it:

1. Read sections 1–4 now, once. They are the mental model.
2. Copy section 5's patterns as you need them; do not memorise them.
3. Use section 7 while doing the Day 1 exercises.
4. Keep section 10 (checklist) open when you finish a feature.

---

## 1. What a test really is

A test is **an executable claim about behaviour**. Nothing more. It has three
jobs:

| Job | Example |
|---|---|
| Specify | "`two_sum(&[2,7,11,15], 9)` returns `Some((0,1))`" |
| Protect | A future refactor that breaks the claim turns the suite red |
| Document | A newcomer learns the API from the tests instead of guessing |

A test can only fail in three ways, which is why you must know which one you
are looking at:

```
1. Compile error   → the API contract changed (signature, type, visibility)
2. panic           → an assertion failed, or the code panicked (unwrap, index)
3. timeout/hang    → deadlock, infinite loop, or a test that waits on the network
```

Tests run in **parallel threads inside one process**, in **no guaranteed
order**, and any panic fails only that test. This single fact explains a lot of
rules later (no shared global state, no `sleep`, isolate filesystem paths).

### Arrange–Act–Assert

Every good test has the same shape:

```rust
#[test]
fn withdraw_reduces_balance() {
    // Arrange: build the world (cheap, explicit)
    let mut account = Account::new();
    account.deposit(100).unwrap();

    // Act: exactly one call that matters
    let remaining = account.withdraw(30).unwrap();

    // Assert: the observable outcome
    assert_eq!(remaining, 70);
    assert_eq!(account.balance(), 70);
}
```

If a test needs a 30-line arrangement, extract a **fixture builder** (section
5.4). If a test needs two "Act" steps, it is probably two tests.

---

## 2. The four homes for tests

Rust has no test runner config and no `describe` blocks. The *location* of the
test determines what it can see. This is the most important table in the guide.

```
crates/resp/
├── src/
│   └── lib.rs        ← unit tests live INSIDE, in `#[cfg(test)] mod tests`
├── tests/
│   └── encode.rs     ← integration tests: one file = one separate crate
└── examples/         ← runnable examples (optional, not tests)
```

| Kind | Where | Can access | Speed | Use for |
|---|---|---|---|---|
| Unit test | `src/*.rs` in `#[cfg(test)] mod tests` | Private + public items | Instant | Logic, edge cases, error paths, invariants |
| Integration test | `tests/*.rs` (or `tests/feature/mod.rs`) | **Only `pub` API** | Fast | User-visible workflows, "does the crate work as shipped" |
| Doc test | `///` comments on public items | Treated as an external user | Fast | Examples that must never go stale; API ergonomics |
| Bench / fuzz / property | `benches/`, `fuzz/`, proptest | Depends | Slow | Performance, hostile input, invariants over many inputs |

### Unit tests

```rust
// crates/dsa/src/lib.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;   // pulls in `add`, and private items too

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

- `#[cfg(test)]` means the module is **not compiled** into release builds: zero
  binary cost, and test-only helpers cannot leak.
- Put unit tests in the **same file as the code**. Working engineers do this
  because a refactor of the file moves the tests with it, and reviewers see
  both in one diff.
- Split big modules: `src/parser.rs` with its own `mod tests` is better than a
  giant `tests` module in `lib.rs`.

### Integration tests

```rust
// crates/resp/tests/encode.rs
use resp::{RespValue, encode};   // only `pub` items exist here

#[test]
fn simple_string() {
    let v = RespValue::Simple("OK".to_string());
    assert_eq!(encode(&v), b"+OK\r\n");
}
```

Why these matter: they are the **only** tests that prove your crate is usable
from outside. A unit test can pass while the type is accidentally private, a
trait bound is missing, or an error type is unusable.

Language detail worth knowing: the `tests/` directory is compiled with the
crate as an **external dependency**, not as a module. So `use crate::...` does
not work there; `use resp::...` does.

### Doc tests

```rust
/// Encodes a RESP value into its wire representation.
///
/// ```
/// use resp::{RespValue, encode};
///
/// assert_eq!(encode(&RespValue::Integer(42)), b":42\r\n");
/// assert_eq!(encode(&RespValue::Null), b"$-1\r\n");
/// ```
///
/// Arrays are recursive:
///
/// ```
/// # use resp::{RespValue, encode};
/// let v = RespValue::Array(vec![RespValue::Simple("PING".into())]);
/// assert_eq!(encode(&v), b"*1\r\n+PING\r\n");
/// ```
pub fn encode(value: &RespValue) -> Vec<u8> { /* ... */ }
```

- Doc tests run on `cargo test`, so **examples in docs cannot rot**.
- Lines starting with `# ` are hidden from rendered docs but still executed —
  use them for `use` statements and setup noise.
- Modifiers: ```` ```no_run ```` (compile but don't execute, for network/disk),
  ```` ```ignore ```` (skip entirely — a smell), ```` ```compile_fail ```` (the
  snippet must *not* compile; good for documenting ownership or trait rules).
- Doc tests are the cheapest way to test a public API from a *user's* point of
  view. For a library like `resp`, every public function deserves one.

> Note: doc tests only run for **library** crates, not for `fn main` binaries.

### Which one should I write?

Default rule used in practice:

```
logic with branches / errors  → unit test (many, fast)
does it work when consumed?   → integration test (a few, realistic)
public API example            → doc test (one per public item, cheap)
```

The common real-world ratio is roughly 70% unit, 20% integration, 10% doc.
You do not need 100% line coverage; you need **every behaviour and every
documented failure mode** covered.

---

## 3. The assertion toolbox

| Tool | Use it for | Notes |
|---|---|---|
| `assert!(cond)` | Boolean properties | Add a message when the failure is not obvious |
| `assert!(cond, "ctx: {value}")` | Any assertion | Captures variables, only formatted on failure |
| `assert_eq!(left, right)` | Exact value equality | Requires `PartialEq + Debug`; prints both sides |
| `assert_ne!(a, b)` | "these must differ" | Useful for uniqueness |
| `matches!(value, Pattern)` | Enum variants / shapes | `assert!(matches!(err, E::Frozen))` |
| `unwrap_err()` | Result error path | Panics if `Ok`, which is what you want |
| `#[should_panic(expected = "...")]` | Documented panic | Substring match; use sparingly |
| `assert!(result.is_ok())` | ⚠️ Anti-pattern | Loses the value and the error; match or unwrap |

### Assert on the strongest thing available

```rust
// Weak: tells you nothing when it fails
assert!(result.is_ok());

// Better: shows the actual error
let value = result.expect("encode should succeed for a valid value");

// Best when failure cases matter: assert the exact typed error
assert_eq!(
    account.withdraw(50).unwrap_err(),
    AccountError::InsufficientFunds { available: 30, requested: 50 }
);
```

`assert_eq!` on a typed error enum is stronger than any string check. That is
why real crates define `enum Error` instead of returning `String`.

### Tests can return `Result`

```rust
#[test]
fn parses_a_full_request() -> Result<(), Box<dyn std::error::Error>> {
    let request = parse("*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n")?;   // `?` works
    assert_eq!(request.command(), "GET");
    Ok(())
}
```

When the test returns `Err`, the harness reports it as a failure with the
`Debug` output. Use this when setup is naturally fallible; it keeps tests free
of `unwrap` chains. (Plain `unwrap`/`expect` in tests is fine — the panic
message *is* the report.)

### Panics

```rust
#[test]
#[should_panic(expected = "age out of range")]
fn rejects_impossible_age() {
    let _ = age_bucket(121);
}
```

Guideline: return `Result` for expected failures; reserve panics for
**programmer errors** (violated invariants). Test those panics so the contract
is explicit. `expected = "..."` is a substring check — keep it short and stable
instead of matching a whole message.

---

## 4. Commands you will actually type

Run from the workspace root.

```sh
cargo test -p resp                  # one package
cargo test --workspace              # everything
cargo test -p resp simple_string    # tests whose full path contains "simple_string"
cargo test -p resp --test encode    # only the tests/encode.rs target
cargo test -p resp --doc            # only doc tests
cargo test -p resp -- --nocapture   # show stdout/stderr from passing tests
cargo test -p resp -- --test-threads=1   # serial; debugging global state
cargo test -p resp -- --ignored     # run only #[ignore]d (slow/stress) tests
cargo test -p resp --no-run         # compile tests, run nothing (fast check)
cargo test --workspace --all-targets     # lib, bins, tests, examples, benches
```

Useful extras:

```sh
cargo test -p dsa -- --list          # enumerate tests without running them
cargo test -p dsa two_sum -- --exact # exact path match, no substring surprises
RUST_BACKTRACE=1 cargo test -p dsa   # backtraces on panic
```

Why `-p` matters in your workspace: the RESP exercise intentionally does not
compile yet, so `cargo test --workspace` is red by design. Day to day you run
`cargo test -p dsa`, `cargo test -p resp`, and only use `--workspace` when you
believe everything should build.

### The quality gate real teams run in CI

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

`--all-targets` is the part people forget: without it, Clippy does not lint your
test code, and tests are code that needs linting too.

---

## 5. Patterns worth copying

Each pattern below is one you will use for years. Examples are self-contained;
adapt names to your crate.

### 5.1 Table-driven tests

When many inputs share one behaviour, write a table. The failure message must
identify the row.

```rust
#[test]
fn two_sum_finds_the_expected_pair() {
    // (input, target, expected)
    let cases = [
        (vec![2, 7, 11, 15], 9, Some((0, 1))),
        (vec![3, 2, 4], 6, Some((1, 2))),
        (vec![3, 3], 6, Some((0, 1))),
        (vec![1, 2], 100, None),
        (vec![], 0, None),
        (vec![5], 10, None),
    ];

    for (nums, target, expected) in cases {
        assert_eq!(
            two_sum(&nums, target),
            expected,
            "nums={nums:?}, target={target}"
        );
    }
}
```

Rules that make this work:

- The loop body must be a *single* assertion with a **context message**.
  Without the message, a failure says "left != right" and you must guess.
- Cases should be data, not code. If a case needs custom logic, it deserves its
  own test.
- Keep the happy path in a table; keep bizarre one-off scenarios separate so
  their intent stays readable.

### 5.2 Testing errors properly

Three layers, in increasing strength:

```rust
// 1. Is it an error at all?
assert!(result.is_err());

// 2. Which variant? (good when fields don't matter or don't exist)
assert!(matches!(result, Err(AccountError::Frozen)));

// 3. Exactly which data? (best — the enum should carry context)
assert_eq!(
    result,
    Err(AccountError::InsufficientFunds { available: 30, requested: 50 })
);
```

For OS errors, never match the message text — it differs per platform:

```rust
use std::io::ErrorKind;

let err = std::fs::read_to_string("/definitely/missing").unwrap_err();
assert_eq!(err.kind(), ErrorKind::NotFound);   // portable
```

Tip: design errors so tests *can* be precise. `InsufficientFunds { available,
requested }` makes assertions readable and gives callers something to log.
`String` errors force string matching, which is flaky and refactor-hostile.

### 5.3 Testing that state is unchanged after a failure

Production bugs cluster around half-applied changes. Test it directly:

```rust
#[test]
fn a_rejected_withdrawal_changes_nothing() {
    let mut account = Account::new();
    account.deposit(10).unwrap();

    assert!(account.withdraw(11).is_err());
    assert_eq!(account.balance(), 10);   // the real assertion
}
```

This is why the pattern exists: `withdraw` might correctly return `Err` *and*
still have decremented the balance first. The error test alone would miss it.

### 5.4 Fixtures and builders

When arrangement grows past ~5 lines, name it:

```rust
fn account_with(balance: i64) -> Account {
    let mut account = Account::new();
    if balance > 0 {
        account.deposit(balance).unwrap();
    }
    account
}

#[test]
fn fixture_keeps_the_test_about_behaviour() {
    let mut account = account_with(500);
    assert_eq!(account.withdraw(200), Ok(300));
}
```

Two flavours, both common:

```rust
// Builder: many optional knobs, chainable, defaults are valid.
let request = RequestBuilder::new("GET").key("foo").timeout_ms(500).build();

// Fixture constant: large immutable data loaded once.
fn fixture_bytes() -> Vec<u8> {
    include_bytes!("../fixtures/get_request.resp").to_vec()
}
```

`include_str!` / `include_bytes!` embed files at compile time — ideal for
protocol fixtures, and they work whether or not the working directory is right.

Note: do **not** relax production APIs just to make a test convenient
(e.g. allowing `deposit(0)`). Encode the rule in the fixture instead.

### 5.5 Fakes over mocks: make time and I/O injectable

The #1 source of flaky tests is real time, real network, real randomness.
Solution: put it behind a trait, inject a fake.

```rust
pub trait Clock {
    fn now_secs(&self) -> u64;
}

pub struct SystemClock;
impl Clock for SystemClock { /* SystemTime::now() */ }

pub struct TtlCache<V> { /* ... */ }
impl<V> TtlCache<V> {
    pub fn get(&self, clock: &dyn Clock, key: &str) -> Option<&V> { /* ... */ }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeClock { now: u64 }
    impl Clock for FakeClock {
        fn now_secs(&self) -> u64 { self.now }
    }

    #[test]
    fn expires_at_the_boundary() {
        let mut clock = FakeClock { now: 1_000 };
        let mut cache = TtlCache::new(30);
        cache.insert(&clock, "k", 42);

        clock.now += 29;
        assert_eq!(cache.get(&clock, "k"), Some(&42));

        clock.now += 1;                 // elapsed == ttl
        assert_eq!(cache.get(&clock, "k"), None);
    }
}
```

Why this is the real technique:

- No `sleep`, so the test takes microseconds and never flakes on CI.
- **Boundary testing becomes possible** — you can assert at exactly `ttl - 1`
  and exactly `ttl`, which is impossible with a real clock.
- The fake lives in `#[cfg(test)]`, so it never ships.

Same idea for I/O: pass a `Read`/`Write` or a trait, test with `&[u8]` /
`Vec<u8>`; for the network, define a `Transport` trait and a fake. In async
code, Tokio's `start_paused = true` gives you the same "fake time" for free.

> Mocks (`mockall`, `mockito`) are the same idea with generated boilerplate.
> Reach for them when a trait has many methods; hand-written fakes are clearer
> for one or two.

### 5.6 Property tests and invariant loops

Example-based tests check inputs you thought of. Property tests check inputs
you didn't. No dependency needed for the simple version:

```rust
#[test]
fn encoded_length_is_never_short() {
    for size in [0usize, 1, 5, 1_000, 65_536] {
        let payload = vec![b'x'; size];
        let encoded = encode(&RespValue::Bulk(payload));

        // Invariant: every response carries at least the framing bytes.
        assert!(encoded.len() > size, "size={size}");
        assert_eq!(encoded[0], b'$', "size={size}");
    }
}
```

With `proptest` (add to `[dev-dependencies]`), the framework generates many
inputs and **shrinks** a failing case to the minimal reproduction:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn bulk_roundtrips_through_decode(bytes in proptest::collection::vec(any::<u8>(), 0..4096)) {
        let encoded = encode(&RespValue::Bulk(bytes.clone()));
        prop_assert_eq!(decode(&encoded).unwrap(), RespValue::Bulk(bytes));
    }
}
```

When proptest fails it prints a seed. **Copy that seed into a regression test**
so the bug stays fixed forever.

Guideline: property tests are for invariants (round-trip, never-panics, length
relations), not for asserting specific outputs. Keep example tests for those.

### 5.7 Snapshot / golden tests

For anything that produces a large exact artifact — protocol bytes, rendered
text, JSON, CLI help — one comparison locks the whole shape:

```rust
#[test]
fn encodes_a_get_request_exactly() {
    let value = RespValue::Array(vec![
        RespValue::Bulk(b"GET".to_vec()),
        RespValue::Bulk(b"foo".to_vec()),
    ]);

    let expected: &[u8] = b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n";
    assert_eq!(encode(&value), expected);
}
```

For big outputs, keep the expected value in a file and compare with
`include_str!` (or use the `insta` crate, which writes the snapshot and shows a
diff on failure). Benefit: you review the *entire* output in the diff instead
of ten narrow assertions. Cost: a snapshot test can bless a bug — review
snapshot changes deliberately.

This pattern is exactly right for the RESP encoder: the provided tests are
golden tests for the wire format.

### 5.8 Isolating filesystem tests

Tests run in parallel; two tests writing `./tmp.txt` will fight. Give each test
its own directory, and clean up with `Drop`.

```rust
use std::fs;
use std::path::{Path, PathBuf};

struct TempDir { path: PathBuf }

impl TempDir {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir()
            .join(format!("dsa-{label}-{}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
    fn path(&self) -> &Path { &self.path }
}

impl Drop for TempDir {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.path); }
}

#[test]
fn writes_and_reads_a_file() {
    let dir = TempDir::new("roundtrip");
    let file = dir.path().join("out.txt");

    fs::write(&file, "hello").unwrap();
    assert_eq!(fs::read_to_string(&file).unwrap(), "hello");
}
```

In real projects use the `tempfile` crate (`tempfile::tempdir()`), which also
handles unique names and cleanup-on-panic. Points that matter:

- Cleanup via `Drop` runs even when the test panics — that is why you write a
  guard type instead of a `remove_dir_all` at the end of the test.
- Make names unique even within one process: use a counter, the test name, or
  a timestamp. Two tests with the same label can collide.
- Prefer `CARGO_MANIFEST_DIR` for read-only fixture paths so tests work no
  matter where Cargo is invoked from:

```rust
let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/get.resp");
```

### 5.9 Concurrency tests

Assert the invariant, not the interleaving:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[test]
fn every_increment_is_counted() {
    let hits = AtomicUsize::new(0);

    thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                for _ in 0..1_000 {
                    hits.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    assert_eq!(hits.load(Ordering::Relaxed), 8_000);
}
```

- `thread::scope` lets threads borrow locals and joins them automatically — if
  you forget a join in a hand-rolled version, the test becomes flaky, which is
  a useful thing to understand.
- A pass does **not** prove there is no race. For that, use `cargo miri test`
  (undefined behaviour) or `loom` (exhaustive interleaving exploration).
- Stress tests are slow; mark them `#[ignore = "stress; run with --ignored"]`
  so the normal loop stays fast, and run them in CI.

### 5.10 Async tests

Async functions need a runtime. With Tokio, a dev-dependency plus a macro:

```rust
#[tokio::test]
async fn responds_to_ping() {
    let (client, server) = tokio::io::duplex(1024);
    // ...
    assert_eq!(response, b"+PONG\r\n");
}
```

Use `#[tokio::test(start_paused = true)]` for timeouts so the test does not
actually wait. Never use `sleep` to "wait for" a spawned task; use a channel,
a `Notify`, or `JoinHandle::await`.

---

## 6. How a working engineer writes a test

Not "write code then bolt on tests". The loop is:

```
1. RED    Write a failing test that states the behaviour.
          Run it. Confirm it fails for the RIGHT reason.
2. GREEN  Write the smallest code that makes it pass. Ugly is allowed.
3. REFACTOR  Clean up with the test as a safety net.
4. REPEAT  Next behaviour, next test.
```

The "right reason" step matters. A test that fails to compile is red for a
different reason than one that compiles and asserts wrongly; know which state
you are in. Your Day 1 RESP exercise starts deliberately in the
*compile-error* red state — that is the contract being handed to you.

### Choosing what to test

Do test:

- every public function's contract (inputs → outputs, documented errors);
- boundaries: empty, one element, max/min, zero, duplicates;
- error paths and "state unchanged after failure";
- anything with a bug history (each fixed bug earns a regression test);
- encoding/parsing and anything with a wire format.

Don't test:

- the standard library (`Vec::push` works);
- private implementation details that a refactor is allowed to change;
- exact log text, exact timings, or anything random without a fixed seed;
- the same behaviour at three layers (unit + integration + doc) unless the
  layers genuinely differ in what they can see.

### Naming

Test names are documentation. Use a behavioural sentence:

```rust
// Good — reads as a rule
fn withdraw_more_than_balance_is_rejected()
fn encodes_empty_bulk_differently_from_null()
fn rejects_negative_amounts()

// Bad — reads as a TODO
fn test1()
fn test_withdraw()
fn it_works()
```

A good suite output reads like a specification, which is also why failure
messages should include the input.

### Determinism checklist

Before committing a test, ask:

- Does it need the network? (→ fake it)
- Does it `sleep`? (→ fake the clock, or wait on a channel)
- Does it rely on the current date/locale/CPU count? (→ inject it)
- Does it share a path or a global with another test? (→ isolate it)
- Does it depend on iteration order of a `HashMap`? (→ sort, or compare sets)
- Would it pass on a slower machine / with `--test-threads=1`? (try it once)

A flaky test is worse than no test: it teaches the team to ignore red.

---

## 7. Applied to this workspace

### 7.1 `crates/dsa` — grow the two-sum suite

You already have five good tests. Three upgrades that teach the rest of the
patterns:

**a) Table-driven expansion** (replaces repetitive cases):

```rust
#[test]
fn two_sum_table() {
    let cases: &[(&[i32], i32, Option<(usize, usize)>)] = &[
        (&[2, 7, 11, 15], 9, Some((0, 1))),
        (&[3, 2, 4], 6, Some((1, 2))),
        (&[3, 3], 6, Some((0, 1))),
        (&[1, 2], 100, None),
        (&[], 0, None),
        (&[5], 10, None),
        (&[-3, 4, 3, 0], 0, Some((0, 2))),
        (&[i32::MIN, i32::MAX], -1, Some((0, 1))),
    ];

    for &(nums, target, expected) in cases {
        assert_eq!(two_sum(nums, target), expected, "nums={nums:?}, target={target}");
    }
}
```

**b) The invariant that makes `Option<(usize, usize)>` trustworthy** — indices
must be valid and distinct, for *any* result you get back:

```rust
#[test]
fn returned_indices_are_valid_and_distinct() {
    let nums = [4, -1, 7, 3, 0, 9];
    for target in -5..=15 {
        if let Some((i, j)) = two_sum(&nums, target) {
            assert!(i < nums.len() && j < nums.len(), "out of bounds");
            assert_ne!(i, j, "an element used twice");
            assert_eq!(
                i64::from(nums[i]) + i64::from(nums[j]),
                i64::from(target)
            );
        }
    }
}
```

**c) A property test** (with `proptest` as a dev-dependency): for any slice
longer than 1 and any two distinct indices, the sum of those two elements must
be found — though *which* pair is found may differ from the brute-force answer
when duplicates exist. That distinction (property of the *output*, not equality
to a specific output) is the subtle part of property testing.

Then, when you write the `HashMap` O(n) version, you have a golden reason for
the existing tests: they must stay green **without modification**. If you have
to edit a test to make a faster implementation pass, you changed the contract.

> Ask yourself when refactoring: "is this test asserting the *contract* or my
> current *implementation*?" A contract test survives a rewrite. A test that
> peeks at internals does not — and shouldn't.

### 7.2 `crates/resp` — drive the encoder from the spec

The provided `tests/encode.rs` is your **acceptance contract**: seven golden
tests that fail to compile until `RespValue` and `encode` exist. Real workflow:

```
1. Run `cargo test -p resp` → compile error (RED, reason = missing API)
2. Define the enum exactly as specified → still RED (missing encode)
3. Implement the simple variants → simple_string/error/integer pass
4. Implement Bulb + Null → bulk tests pass
5. Implement Array recursively → array tests pass
6. Write one doc test on `encode` → `cargo test -p resp --doc` passes
7. Add your own unit tests for what the contract does not pin down
```

What the contract does *not* pin down — write unit tests for these yourself:

| Case | Why it matters |
|---|---|
| Multi-byte UTF-8 in `Simple` | Length is bytes, not `chars` |
| Empty array `*0\r\n` | Different from null array |
| `Bulk` containing `\r\n` bytes | Framing is length-based, not delimiter-based |
| Large payload (e.g. 64 KiB) | Length prefix formatting for big numbers |
| Deeply nested arrays | Recursion works to depth > 2 |
| `Integer(i64::MIN)` | No overflow/formatting surprise |
| `Bulk` with non-UTF-8 bytes | Must not go through `String` |

Sketch of the UTF-8 unit test (inside `src/lib.rs`, in `mod tests`):

```rust
#[test]
fn simple_string_length_is_not_encoded_here() {
    // Simple strings have no length prefix — this test exists to fix the
    // expectation, not to measure bytes. Bulk strings are the length-bearing
    // variant, and that is where the bytes-vs-chars bug lives.
    let v = RespValue::Simple("héllo".to_string());
    assert_eq!(encode(&v), "+héllo\r\n".as_bytes());
}

#[test]
fn bulk_length_counts_bytes_not_characters() {
    let v = RespValue::Bulk("héllo".as_bytes().to_vec());
    assert_eq!(encode(&v), b"$6\r\nh\xc3\xa9llo\r\n");
}
```

The second test is the one that catches the classic bug. Write the failing
version first if you can: implement `String::len()` on chars and watch it fail —
that failure is the lesson.

Also write the `Display` stretch test yourself. A good shape:

```rust
#[test]
fn display_is_debug_friendly() {
    assert_eq!(RespValue::Simple("OK".into()).to_string(), r#"Simple("OK")"#);
    assert_eq!(
        RespValue::Bulk(b"GET".to_vec()).to_string(),
        "Bulk([71, 69, 84])"
    );
}
```

Finally, once encoding works, add **round-trip** tests as soon as decoding
exists (Day 2): `decode(encode(v)) == v` for every variant is the single most
valuable test in a protocol crate.

### 7.3 Where new tests go from here

| You are adding | Test goes in |
|---|---|
| A DSA function or structure | `crates/dsa/src/<module>.rs` `mod tests`, plus `crates/dsa/tests/` for cross-module flows |
| A RESP feature | `crates/resp/src/lib.rs` unit tests + `crates/resp/tests/<feature>.rs` |
| A CLI tool | unit tests on pure functions; integration tests driving the binary via `std::process::Command` |
| A server (Day 3+) | integration test over a real TCP socket on port `0` (OS-assigned, no collisions) |
| A protocol fixture | `crates/<name>/tests/fixtures/` + `include_bytes!` |

For a server, the port-`0` trick is worth remembering:

```rust
let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
let addr = listener.local_addr()?;   // ask the OS for a free port
```

Hard-coded ports produce "address already in use" flakes in CI.

---

## 8. Anti-patterns

| Anti-pattern | Why it hurts | Fix |
|---|---|---|
| `assert!(result.is_ok())` | Hides the value and the error | `expect(...)` or match |
| Asserting on error strings from the OS | Platform/version dependent | Assert `ErrorKind` or your own typed error |
| `sleep(100ms)` to wait for something | Flaky on loaded CI | Fake clock, channel, `JoinHandle::await` |
| Tests depending on execution order | Harness runs in parallel, unordered | Each test builds its own world |
| Shared temp file path | Parallel tests clobber each other | Unique dir per test + `Drop` cleanup |
| Testing private internals from `tests/` | Impossible (external crate) or brittle | Keep in `mod tests`, or test the public behaviour |
| One giant test with 20 assertions | First failure hides the rest | One behaviour per test |
| No message on a loop assertion | You cannot tell which input failed | Include the input in the message |
| `#[ignore]` on everything slow by default | Nobody ever runs them | Run ignored tests in CI explicitly |
| Editing the contract test to make new code pass | You just deleted the specification | Fix the code, or consciously version the contract |
| Random data without a printed seed | Failure cannot be reproduced | Seed it and print the seed |
| 100% coverage as a goal | Rewards testing getters, not behaviour | Cover behaviours and failure modes |

---

## 9. Tooling ladder (adopt as needed, not all at once)

| Tool | What it buys | When to add |
|---|---|---|
| `cargo test` | The harness | Day 1 — already have it |
| `cargo fmt`, `cargo clippy --all-targets -D warnings` | Style + lint bugs (including test code) | Day 1 |
| `cargo test -- --nocapture`, `RUST_BACKTRACE=1` | Debugging failing tests | As needed |
| `rstest` | Fixtures + parametrised tests with `#[case]` | When tables get verbose |
| `proptest` | Generated inputs + shrinking | Parsers, encoders, numeric code |
| `tempfile` | Isolated dirs/files | First filesystem test |
| `insta` | Snapshot tests with diffs and review workflow | Large text/JSON output |
| `mockall` / `wiremock` | Generated mocks / fake HTTP servers | Many-method traits, HTTP clients |
| `criterion` | Statistical benchmarks (not tests) | When performance is a requirement |
| `cargo nextest` | Faster runner, per-test timeouts, better output | Suite > a few hundred tests |
| `cargo llvm-cov` | Coverage report (find *untested*, not "100%") | When you want blind-spot data |
| `cargo mutants` | Mutation testing: do your tests actually catch bugs? | Before trusting a critical module |
| `cargo miri test` | Detects UB (including some data races) | Unsafe code, concurrency |
| `loom` | Exhaustive concurrency interleavings | Lock-free / custom sync primitives |

Install pattern (they are normal Cargo tools):

```sh
cargo install cargo-nextest cargo-llvm-cov   # binaries
cargo add --dev proptest tempfile            # per-crate test dependencies
```

`[dev-dependencies]` are only compiled for tests, examples, and benches — they
never appear in your shipped binary.

Maturity path that mirrors your roadmap:

```
Week 1   unit tests + assert_eq + table-driven           (dsa, resp)
Week 2   integration tests + doc tests + fixtures        (resp)
Week 3   fakes for time/IO, error-path tests             (first CLI)
Week 4   proptest + tempfile + coverage                  (library quality)
Later    nextest in CI, insta, criterion, miri/loom      (project-grade)
```

---

## 10. Definition of done (per feature)

Before calling a feature finished:

- [ ] The happy path is tested.
- [ ] Empty / zero / one-element / max-value inputs are tested where relevant.
- [ ] Every documented error variant has a test that asserts *which* variant.
- [ ] A failed operation is shown to leave state unchanged.
- [ ] Public API has at least one doc test (library crates).
- [ ] Tests are deterministic: no network, no `sleep`, no shared paths.
- [ ] Test names read as rules, and loop assertions name the input.
- [ ] `cargo test -p <crate>` passes.
- [ ] `cargo clippy -p <crate> --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] Every bug you fixed today has a regression test tomorrow.

---

## 11. Further reading

- The Rust Book, ch. 11 — *Writing Automated Tests*.
- Rust by Example — *Testing*.
- `std::assert` macro docs (read the `assert_eq!` docs once; the customization
  notes are useful).
- Rust API Guidelines — *Documentation* and *Predictability* (why typed errors,
  why doc examples).
- `cargo test` reference in the Cargo Book (all flags, target discovery).
- Tokio tutorial — testing async code; `mini-redis` for how a real protocol
  project is structured and tested.
- `proptest` book — properties, strategies, shrinking.
