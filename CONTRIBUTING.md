# Contributing to TopLang

Thank you for your interest in contributing to TopLang! We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/toplang.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m "Add your feature"`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.74 or later
- Git

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the Compiler

```bash
cargo run -- examples/hello.top
```

## Code Style

We follow standard Rust conventions:

- Run `cargo fmt` before committing to format your code
- Run `cargo clippy` to catch common mistakes
- Write tests for new features
- Update documentation as needed

## Testing

### Running All Tests

```bash
cargo test --verbose
```

### Testing Your Changes

1. Build the compiler: `cargo build --release`
2. Test with example: `./target/release/topc examples/hello.top`
3. Test with your own .top files

### Adding New Tests

- Add unit tests in the same file as your code using `#[cfg(test)]`
- Add integration tests in a `tests/` directory
- Add example `.top` files in the `examples/` directory

## Pull Request Guidelines

1. **One feature per PR** - Keep changes focused and atomic
2. **Write clear commit messages** - Explain what and why, not how
3. **Update documentation** - Update README.md if adding features
4. **Add tests** - Ensure new code is tested
5. **Check CI** - Make sure all CI checks pass

## Adding New Language Features

When adding new language features:

1. Update `token.rs` with new token types
2. Update `lexer.rs` to recognize new syntax
3. Update `ast.rs` with new AST nodes
4. Update `parser.rs` to parse new syntax
5. Update `interpreter.rs` to execute new features
6. Add tests for the new feature
7. Update README.md with examples
8. Add example `.top` files demonstrating the feature

## Reporting Bugs

When reporting bugs, please include:

- TopLang version (run `topc --version`)
- Operating system and version
- Rust version (run `rustc --version`)
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Sample `.top` code that triggers the bug

## Feature Requests

We welcome feature requests! Please:

- Check if the feature already exists
- Explain the use case
- Provide examples of how it would work
- Consider backward compatibility

## Questions?

Feel free to:

- Open an issue for questions
- Start a discussion on GitHub
- Review existing issues and PRs

## Code of Conduct

Be respectful and inclusive. We're all here to learn and build together.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
