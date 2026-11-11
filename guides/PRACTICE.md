# Practice guide

## A learning loop

Choose one concept and define a small outcome. Read the relevant Book section, try Rustlings, write or change a small playground example, then explain the result in your own words. For DSA, write examples and edge cases before comparing approaches. Use compiler messages and documentation as part of the exercise; ask for a hint or review when useful.

## Where to put things

- `exercises_learn/src/main.rs`: current scratch program. As examples grow, consider named examples under `exercises_learn/examples/`; run one with `cargo run -p exercises_learn --example NAME`.
- `crates/dsa/src/`: one clearly named module per topic or structure, with unit tests; integration tests can go in `crates/dsa/tests/`.
- `crates/resp/`: protocol project and its integration tests. Keep the provided tests as the exercise contract.
- `notes/`: dated learning logs and exercise briefs. Copy [TEMPLATE.md](../notes/TEMPLATE.md) for a new session.
- New projects: create a named package under `crates/` when ready. The root manifest includes `crates/*`; check workspace membership and add a README with the project's purpose and run commands.

Keep project code, practice code, and notes identifiable. Avoid committing generated `target/` output, credentials, or unrelated downloads. Rustlings is an external exercise set, separate from this playground.

## Review prompts

- Rust: who owns each value, and how long do borrows last? What error cases exist?
- DSA: what are time and space costs? What happens for empty input, duplicates, and boundary values?
- Projects: how does someone run and test this? What is implemented and what remains?
- Reflection: what did the compiler teach me, and what should I revisit?
