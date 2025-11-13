# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-11-13

### Removed
- **C++ and LLVM dependencies**: Removed all legacy C++/LLVM build artifacts and configuration
  - Deleted `CMakeLists.txt` (old C++/LLVM build configuration)
  - Deleted entire `build/` directory containing CMake artifacts and compiled C++ object files
  - Removed 102 files and 41,814 lines of legacy code
- **Unused code cleanup**: Removed unused token types and operators
  - Removed `TokenType::Comment`, `TokenType::Eol` (never created by lexer)
  - Removed `TokenType::NotEquals`, `TokenType::GreaterEquals`, `TokenType::LessEquals` (defined but never tokenized)
  - Removed `BinaryOp::NotEquals`, `BinaryOp::GreaterEquals`, `BinaryOp::LessEquals` (unreachable AST nodes)
  - Removed unreachable parser and interpreter code for these operations
  - Total cleanup: 33 lines of dead code removed

### Changed
- **Pure Rust implementation**: Project now uses a tree-walking interpreter instead of LLVM code generation
  - No external dependencies beyond Rust crates
  - Simplified architecture with lexer → parser → interpreter pipeline
- **Documentation updates**: Updated README.md to reflect pure Rust implementation
  - Removed references to LLVM and C++
  - Updated performance claims to emphasize native Rust performance

### Fixed
- **Code quality**: Fixed clippy lint warning in `parse_equality` function
  - Converted `loop` with match-break pattern to cleaner `while matches!` pattern
  - Resolved `clippy::while_let_loop` lint warning

### Technical Details
- All tests pass (5/5)
- All clippy lints pass with `-D warnings`
- Cross-platform support maintained (Linux, macOS, Windows)
- No breaking changes to language syntax or behavior

## [0.1.0] - 2024-11-13

### Added
- Initial release of TopLang compiler written in Rust
- Lexer for tokenizing TopLang source code
- Parser for building Abstract Syntax Tree (AST)
- Tree-walking interpreter for executing TopLang programs
- Support for variables, constants, and basic data types (numbers, strings, booleans)
- Arithmetic operations: `plus`, `minus`, `times`, `divided by`
- Comparison operations: `equals`, `greater than`, `less than`
- Logical operations: `and`, `or`, `not`
- Control flow: `if`/`else`, `while` loops
- Functions with parameters and return values
- Natural English-like syntax
- Command-line interface with options:
  - `-t, --show-tokens`: Display tokens after lexing
  - `-a, --show-ast`: Display AST after parsing
  - `-v, --verbose`: Enable verbose output
- Cross-platform support (Linux, macOS, Windows)
- Comprehensive test suite
- CI/CD pipeline with GitHub Actions
- Installation scripts for Unix and Windows
- Homebrew formula support

[0.2.0]: https://github.com/taufiksoleh/toplang/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taufiksoleh/toplang/releases/tag/v0.1.0
