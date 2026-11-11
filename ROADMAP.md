# Learning roadmap

Use this as a flexible sequence, not a deadline. Work in three tracks together: learn a Rust concept, practice a DSA idea, and apply both to a project. The [practice guide](guides/PRACTICE.md) describes how to record and review a session.

## 1. Rust foundations

- [ ] Install Rust; understand `rustup`, `rustc`, Cargo, packages, and the workspace.
- [ ] Variables, mutability, types, functions, control flow, and pattern matching.
- [ ] Ownership, moves, borrowing, references, slices, and strings.
- [ ] Structs, enums, `Option`, `Result`, and error handling.
- [ ] Collections, modules, visibility, tests, and documentation.
- [ ] Generics, traits, lifetimes, iterators, and closures.
- [ ] Smart pointers, interior mutability, threads, and shared state.
- [ ] Async basics and performance measurement after the synchronous foundations.

For each topic: read a Book chapter, solve relevant Rustlings exercises, and make a small experiment in `exercises_learn/`.

## 2. DSA in Rust

Start with brute force, explain the complexity, then improve the approach and test edge cases. Implement a structure yourself when that structure is the subject of the exercise; standard collections are useful tools for other problems.

- [ ] Arrays, slices, strings, two pointers, and binary search; finish two-sum.
- [ ] Dynamic arrays, linked structures, stacks, queues, and ring buffers.
- [ ] Hashing, sets, maps, and frequency counting.
- [ ] Recursion, trees, heaps, and priority queues.
- [ ] Graph traversal, sorting, searching, and dynamic programming.
- [ ] Advanced practice: LRU cache, tries, and skiplists where useful.

Keep implementations and tests in `crates/dsa/`; record the input assumptions, complexity, edge cases, and what you learned.

## 3. Projects

Build small, runnable slices. Suggested progression:

1. **Concept experiments:** small programs exploring ownership, enums, files, and error handling.
2. **Command-line tool:** e.g. a task tracker or text search utility with tests and useful errors.
3. **RESP / Redis-inspired server:** begin with the [Day 1 encoder](notes/day-01.md), then decoding, TCP, a key-value store, expiry, and concurrency. Start with the standard library; explore async and Tokio later.
4. **Choose a capstone:** a file indexer, chat server, or other project that combines your interests. Document how to run it, its limitations, and what you learned.

These are ideas and milestones, not features already implemented. Add a new Cargo package only when you start a project and explain how to run it.

## Suggested session

1. Pick one small goal and read the relevant documentation.
2. Try it yourself; read compiler errors and use hints when stuck.
3. Run focused tests, then format and lint when the work is ready.
4. Write what broke, what clicked, and one question in `notes/`.
5. Review the diff and commit when ready, once Git is deliberately set up.

For a finished slice, aim for a working example, meaningful tests including edge cases, a complexity explanation for DSA, and a short note. Run `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets -- -D warnings` once all exercises compile. Until then, use package-specific checks and expect unfinished exercises to fail.
