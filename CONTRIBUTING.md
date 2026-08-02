# Contributing to Ruskey

Thank you for your interest in contributing to Ruskey — contributions make this project better and help others learn. This document explains how to get started, the preferred workflow, and guidelines for proposed changes.

## Ways to contribute

- Report bugs or unexpected behavior by opening an issue with steps to reproduce.
- Suggest new features or improvements via issues or discussions.
- Submit pull requests with bug fixes, documentation improvements, tests, or small features.
- Add examples and clarify README content for newcomers.

## Getting started

1. Fork the repository and clone your fork:

```bash
git clone https://github.com/<your-username>/Ruskey-1.git
cd Ruskey-1
```

2. Create a feature branch for your work:

```bash
git checkout -b feat/my-change
```

3. Run the test suite and linters locally before opening a PR:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings || true
cargo test
```

4. Commit related changes in small, focused commits with clear commit messages.

5. Push and open a pull request against `vxssroott/Ruskey-1:main` with a descriptive title and summary.

## Pull request guidelines

- Make PRs small and focused — one logical change per PR.
- Include tests for bug fixes and new functionality where practical.
- Reference any related issue in the PR description (e.g., `Fixes #123`).
- Ensure the code builds and tests pass locally.
- Run `cargo fmt` and `cargo clippy` and address warnings.

## Coding style

- Prefer idiomatic Rust (use iterators, pattern matching, and ownership when appropriate).
- Keep functions small and single-responsibility.
- Document public functions/types with rustdoc comments (///).
- Follow standard module layout and naming conventions.

## Tests

- Add unit tests for library code and integration tests in the `tests/` directory when appropriate.
- Tests should be deterministic and not rely on external network or filesystem state.

## Reporting security issues

If you discover a security vulnerability, please avoid opening a public issue. Contact the repository owner directly via GitHub (private message) or email.

## Code of Conduct

By participating in this project you agree to abide by the repository's Code of Conduct. If you would like, I can add a `CODE_OF_CONDUCT.md` template to the repo.

---

Thanks for helping make Ruskey better — contributions are appreciated!
