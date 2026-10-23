# AGENTS.md

Rust CLI that finds Node.js versions in project files and bumps them to the
latest patch release. See [README.md](README.md) for behavior.

## Commands

- `mise install`: install Rust and tools
- `mise run build`: build the executable
- `mise run test`: run the tests
- `mise run cover`: measure line coverage (must stay at 100%)

## Rules

- Format with `cargo fmt` and keep `cargo clippy --all-targets` clean.
- Add tests for every change; coverage must stay at 100%.
- Specs live in `specs/`. Update them when behavior changes.
- Commit messages need a body, not only a subject line. Write it around
  intent: why the change is needed and why this approach, not a list of
  changed files.
- Wrap code in backticks in commit messages: identifiers, file names, paths,
  commands and options.
