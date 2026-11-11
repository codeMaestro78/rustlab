# Day 1 — RESP encoder

Timebox: 60–75 min. Goal: `cargo test -p resp` goes from "does not compile" to 7 passing.

## 1. Setup check (5 min)

```sh
cd ~/rustlab
cargo test -p resp      # expect compile errors: that is the red state
```

## 2. Spec

RESP2 has five value types. Define exactly this public API in `crates/resp/src/lib.rs`:

```rust
pub enum RespValue {
    Simple(String),      // +OK\r\n
    Error(String),       // -ERR bad\r\n
    Integer(i64),        // :42\r\n
    Bulk(Vec<u8>),       // $5\r\nhello\r\n
    Null,                // $-1\r\n   (null bulk string)
    Array(Vec<RespValue>), // *2\r\n<elem><elem>
}

pub fn encode(value: &RespValue) -> Vec<u8>;
```

Encoding rules:

| Value | Wire form |
|---|---|
| `Simple(s)` | `+` then `s` then `\r\n` |
| `Error(s)` | `-` then `s` then `\r\n` |
| `Integer(n)` | `:` then decimal `n` then `\r\n` |
| `Bulk(bytes)` | `$` then `bytes.len()` then `\r\n` then bytes then `\r\n` |
| `Null` | `$-1\r\n` |
| `Array(items)` | `*` then `items.len()` then `\r\n` then each encoded item |

Notes:

- Lengths are **byte** counts, not character counts.
- An empty bulk string is `$0\r\n\r\n`, which is different from `Null`.
- Arrays are recursive: encoding an array means encoding each element.

Useful things to look up (do not copy a solution): `Vec::extend_from_slice`,
`Vec::push`, `format!`, `write!` with `std::io::Write` is *not* required.

## 3. Do it

Write the enum and `encode`. Run:

```sh
cargo test -p resp
```

If you get a borrow error, read *which* value is moved and *where*. That error is today's lesson.

## 4. Stretch (only if tests are green)

Add `impl std::fmt::Display for RespValue` that renders a debug-friendly form
(`Simple("OK")`, `Bulk([71, 69, 84])`). No test is provided; write one.

## 5. DSA drill (25 min)

In `crates/dsa/src/lib.rs` add:

```rust
pub mod warmup {
    use std::collections::HashMap;

    /// Return the indices of the two numbers that add up to `target`.
    pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        // your code
    }
}
```

Write tests: `[2,7,11,15], 9 => (0,1)`, `[3,3], 6 => (0,1)`, `[1,2], 100 => None`.
Do the brute-force `O(n²)` version first, then the `HashMap` `O(n)` version.
Then answer: why can `two_sum` take `&[i32]` and not `Vec<i32>`?

## 6. Reflect and review

Create a separate session note using [the template](TEMPLATE.md): record what broke, what you understood, and one open question. Review the changes and commit when ready when you are ready.
