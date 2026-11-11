# RustLab 🦀

A personal learning workspace for **Rust from the beginning**, **data structures and algorithms**, and **projects built in Rust**. Learn a concept, experiment with it, solve a problem, and use it in a project. This is a work in progress: exercises are intended to be completed by the learner.

## Start here

1. Install the stable toolchain using the [official Rust installation guide](https://www.rust-lang.org/tools/install). The workspace uses the Rust 2024 edition; use Rust 1.85 or newer.
2. From this folder, check your tools and run the playground:

   ```sh
   rustc --version
   cargo --version
   cargo run -p exercises_learn
   ```

3. Read the [learning roadmap](ROADMAP.md), pick one topic, and try the related [Rustlings](https://rustlings.rust-lang.org/) exercises alongside [The Rust Book](https://doc.rust-lang.org/book/).
4. Keep a short learning log in `notes/`; use the [session template](notes/TEMPLATE.md). Try the [Day 1 RESP and DSA exercises](notes/day-01.md) when ready.

## Explore the workspace

```text
rustlab/
├── Cargo.toml                  # Cargo workspace configuration
├── Cargo.lock                  # Shared dependency versions
├── README.md                   # Start here and command reference
├── ROADMAP.md                  # Learning tracks and milestones
├── guides/                     # How to practice and add work
├── exercises_learn/            # Rust concept playground (binary)
├── crates/
│   ├── dsa/                    # DSA practice (library)
│   └── resp/                   # RESP protocol project (library)
└── notes/                      # Exercises and learning journal
```

| Track | Start | Current state |
| --- | --- | --- |
| Rust concepts | [playground](exercises_learn/src/main.rs) | Ownership and `String` experiments |
| DSA | [DSA library](crates/dsa/src/lib.rs) | Brute-force two-sum with tests; more structures to come |
| Projects | [RESP library](crates/resp/src/lib.rs) | Encoder is an exercise; its [tests](crates/resp/tests/encode.rs) describe the expected API and currently do not compile |
| Reflection | [Day 1](notes/day-01.md) | Exercise brief and prompts |

Workspace-wide tests will fail until the RESP exercise is completed. Work on one package at a time; `cargo run -p exercises_learn` does not require the other packages to compile. See the [practice guide](guides/PRACTICE.md) for a repeatable workflow and where new work belongs.

## Rustlings

[Rustlings](https://github.com/rust-lang/rustlings) provides small exercises to use alongside the Book. It is separate from the `exercises_learn` playground in this workspace. Follow its [current setup instructions](https://rustlings.rust-lang.org/) to install and initialize it, then work through the exercises and use their hints when needed. Keep its generated exercises in a separate folder unless you deliberately decide how they should be tracked here.

## Command reference

Run Cargo commands from the workspace root unless noted. `-p NAME` selects a workspace package; `--workspace` selects all members. Use `--help` for options and see the [complete Cargo command reference](https://doc.rust-lang.org/cargo/commands/cargo.html).

### Everyday Cargo

| Command | Purpose |
| --- | --- |
| `cargo new my-project` | Create a new binary package in a new directory (`--lib` for a library). |
| `cargo init` | Initialize a package in an existing directory. |
| `cargo check -p exercises_learn` | Quickly check a package without producing a runnable binary. |
| `cargo build` / `cargo build --release` | Build the default workspace members in debug / optimized release mode. |
| `cargo run -p exercises_learn` | Build and run the concept playground. |
| `cargo run -p exercises_learn -- arg` | Pass an argument to the program after `--`. |
| `cargo test -p resp` | Run one package's tests; this is the RESP exercise's starting red state. |
| `cargo test --workspace` | Run tests across the workspace once exercises compile. |
| `cargo test -p resp simple_string` | Run tests matching a name; add `-- --nocapture` to show test output. |
| `cargo fmt --all` / `cargo fmt --all -- --check` | Format all workspace packages / check formatting. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Run Clippy and treat warnings as errors. |
| `cargo doc --workspace --no-deps --open` | Generate and open local API documentation. |
| `cargo clean` | Delete generated build artifacts in `target/`; later builds will recompile. |

### Dependencies and discovery

| Command | Purpose |
| --- | --- |
| `cargo add serde` / `cargo remove serde` | Add / remove a dependency in the selected package; use `-p NAME` in a workspace. |
| `cargo tree -p resp` | Show a package's dependency tree. |
| `cargo update` | Update lockfile dependencies within manifest constraints; review the resulting changes. |
| `cargo search NAME` | Search crates.io (requires network access). |
| `cargo metadata --format-version 1` | Print workspace and dependency metadata as JSON. |
| `cargo --list` / `cargo help test` | List installed Cargo commands / get help for one command. |

### Compiler, toolchains, and tools

| Command | Purpose |
| --- | --- |
| `rustc --version` / `rustc --help` | Inspect the Rust compiler version / options. |
| `rustc --explain E0382` | Read the explanation for a compiler error code. |
| `rustc hello.rs` | Compile a standalone file; run the resulting `./hello` on Unix. Use Cargo for workspace packages. |
| `rustup show` / `rustup update stable` | Inspect installed toolchains / update stable. |
| `rustup component add rustfmt clippy` | Install formatting and lint components if missing. |
| `rustup doc` | Open the installed Rust documentation. |
| `rustfmt --version` / `cargo clippy --version` | Check formatting / lint tool availability. |

Some commands need network access, a browser, or optional toolchain components. `cargo new` inside this workspace may modify the root manifest; review it before committing. For a full list rather than a fixed cheat sheet, use `cargo --list`, `rustc --help`, and the official documentation.

## References

- [The Rust Book](https://doc.rust-lang.org/book/) — concepts and worked examples.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — focused examples.
- [Rustlings](https://rustlings.rust-lang.org/) — interactive exercises.
- [Standard library](https://doc.rust-lang.org/std/) and [Rust Reference](https://doc.rust-lang.org/reference/) — APIs and language details.
- [Cargo Book](https://doc.rust-lang.org/cargo/) and [rustup book](https://rust-lang.github.io/rustup/) — packages and toolchains.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — designing Rust libraries.
- [Tokio tutorial](https://tokio.rs/tokio/tutorial) and [mini-redis](https://github.com/tokio-rs/mini-redis) — later project and async references.

## Before putting this on GitHub

The playground is tracked as ordinary files in the root repository. Its former nested Git metadata was moved to `target/exercises_learn-git-backup/` locally; it had no commits or configured remote. `target/` is ignored and `cargo clean` deletes it, including that backup. Keep the workspace `Cargo.lock` for reproducible learning builds. Review file contents and any local secrets before committing or pushing.
