# Ruskey

![License](https://img.shields.io/github/license/vxssroott/Ruskey-1)
![GitHub stars](https://img.shields.io/github/stars/vxssroott/Ruskey-1?style=social)
![Repo size](https://img.shields.io/github/repo-size/vxssroott/Ruskey-1)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)

> A Rust implementation of the Monkey programming language interpreter (inspired by "Writing An Interpreter In Go" by Thorsten Ball).

Table of contents
- About
- Badges
- Features
- Project structure
- Quick start
- Examples
- Library overview
- Tests
- Contributing
- License

About

Ruskey is an educational interpreter for the Monkey programming language written in Rust. The goal of this project is to follow the designs and exercises in Thorsten Ball's "Writing An Interpreter In Go" while exploring idiomatic Rust, ownership, and type design for interpreters.

Badges

- License: MIT
- GitHub stars and repo size
- Rust toolchain indicator

Features

- Lexer: Tokenizes Monkey source code including strings, identifiers, numbers, operators and delimiters.
- Parser: Recursive-descent parser with Pratt-style expression handling.
- AST: Types representing statements and expressions.
- Evaluator: Walks the AST and evaluates Monkey programs.
- Object system: First-class values (integers, booleans, strings, arrays, hashes, functions).
- Environment: Lexical scoping for variables and closures.
- REPL: Read–Eval–Print Loop for interactive experimentation.

Project structure

```
ruskey/
├── src/
│   ├── token.rs       # Token definitions
│   ├── lexer.rs       # Lexical analyzer
│   ├── ast.rs         # Abstract Syntax Tree (AST) nodes
│   ├── parser.rs      # Parser converting tokens to AST
│   ├── object.rs      # Object/value system for the interpreter
│   ├── environment.rs # Environment for variable bindings and scopes
│   ├── evaluator.rs   # AST evaluator / interpreter
│   ├── repl.rs        # Read-Eval-Print Loop
│   ├── builtins.rs    # Built-in functions (if any)
│   └── lib.rs         # Library exports and module declarations
├── tests/             # Test suite
└── Cargo.toml         # Project configuration
```

(If a file listed above is missing, check the `src/` directory — the README reflects the intended structure.)

Quick start

Requirements
- Rust toolchain (rustc + cargo), recommended Rust 1.70+.

Build

```bash
# clone
git clone https://github.com/vxssroott/Ruskey-1.git
cd Ruskey-1

# build
cargo build --release
```

Run tests

```bash
cargo test
```

Run the REPL

```bash
cargo run --release
# or for development
cargo run
```

Examples

Basic function and recursion (Monkey source):

```monkey
// Define fibonacci function
let fibonacci = fn(x) {
  if (x == 0) {
    return 0;
  } else {
    if (x == 1) {
      return 1;
    } else {
      return fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

fibonacci(10);
```

Repl usage

- Start the REPL with `cargo run` and type Monkey expressions to evaluate them.
- Use `let` to bind variables and `fn` to declare functions.

Library overview

- token.rs: TokenType and Token helpers and keyword lookup.
- lexer.rs: Lexer struct that produces Token values from input bytes. It supports strings, identifiers, numbers (integers), and operators, including multi-character operators like `==` and `!=`.
- ast.rs: (AST node types) program, statements, and expression node definitions.
- parser.rs: Implements a Pratt parser with prefix and infix parsing functions and precedence handling. The parser constructs the AST from a token stream.
- object.rs / evaluator.rs: The runtime representation of values and the evaluator that executes AST nodes.
- environment.rs: Implements scoped variable bindings used by the evaluator.
- repl.rs: Simple loop reading input, parsing and evaluating, printing results.

Tests

Run `cargo test`. The `tests/` directory contains integration and unit tests exercising lexer, parser, and evaluator behavior.

Contributing

Contributions are welcome. Typical ways to contribute:

- Open an issue describing a bug or feature request.
- Send a pull request with a focused change and tests.

Guidelines

- Keep changes small and focused.
- Add tests for new features or bug fixes.
- Follow Rust formatting rules (run `cargo fmt`) and clippy checks (`cargo clippy`).

Suggested improvements

- Add CI (GitHub Actions) for building and running tests on push/PR.
- Add more built-in functions and standard library features.
- Add documentation comments (rustdoc) to public APIs and publish crates.io package if desired.

License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

Acknowledgements

- The architecture and exercises are heavily inspired by "Writing An Interpreter In Go" by Thorsten Ball.

Contact

GitHub: https://github.com/vxssroott

