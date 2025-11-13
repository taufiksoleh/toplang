# TopLang - A Simple, Human-First Programming Language

TopLang is a simple, expressive programming language with natural English-like syntax. It's designed to be easy to learn while being powerful enough to write useful programs. Built with Rust for maximum performance and cross-platform compatibility.

## Features

✨ **Human-Readable Syntax** - Use natural English words instead of symbols
🚀 **Fast** - Written in Rust for blazing fast performance
🌍 **Cross-Platform** - Works on Windows, macOS, and Linux
🎯 **Simple** - Easy to learn and understand
🛠️ **Modern** - Built with modern language design principles

## Getting Started

### Prerequisites

To build TopLang, you only need:
- **Rust** (1.70 or later) - Install from [rustup.rs](https://rustup.rs/)

That's it! No complex dependencies or build tools required.

### Building the Compiler

1. Clone this repository:
```bash
git clone https://github.com/taufiksoleh/toplang.git
cd toplang
```

2. Build with Cargo:
```bash
cargo build --release
```

The compiled `top` executable will be in `target/release/top`.

Alternatively, you can install it directly:
```bash
cargo install --path .
```

## Using TopLang

### Basic Usage

To run a TopLang program:

```bash
./target/release/top path/to/your_program.top
```

Or if you installed it:
```bash
top path/to/your_program.top
```

### Command-line Options

The `top` compiler supports several options:

- `-t, --show-tokens` - Display tokens after lexing
- `-a, --show-ast` - Display the Abstract Syntax Tree after parsing
- `-v, --verbose` - Enable verbose output
- `-h, --help` - Display help information
- `-V, --version` - Display version information

Example:
```bash
top -v examples/hello.top
```

## Language Syntax

TopLang uses natural English words to make programming more intuitive:

### Variables

```toplang
var name is "Alice"
var age is 25
var score is 95.5
const PI is 3.14159
```

### Arithmetic Operations

```toplang
var sum is 5 plus 3
var difference is 10 minus 4
var product is 6 times 7
var quotient is 20 divided by 4
```

### Comparison Operations

```toplang
var isEqual is x equals 10
var isGreater is y greater than 5
var isLess is z less than 100
```

### Control Flow

```toplang
if age greater than 18 {
    print "Adult"
} else {
    print "Minor"
}

while count less than 10 {
    print count
    count is count plus 1
}
```

### Functions

```toplang
function greet(name) {
    print "Hello"
    print name
    return 0
}

function main() {
    greet("World")
    return 0
}
```

### Complete Example

See `examples/hello.top` for a complete working example:

```toplang
# This is a simple TopLang example program

function main() {
    print "Hello, World!"

    var count is 1
    while count less than 5 {
        print "Count is"
        print count
        count is count plus 1
    }

    if count equals 5 {
        print "Count reached 5!"
    } else {
        print "Something went wrong!"
    }

    return 0
}
```

## Project Structure

```
toplang/
├── src/
│   ├── main.rs         # Entry point and CLI
│   ├── token.rs        # Token types and definitions
│   ├── lexer.rs        # Lexical analyzer
│   ├── ast.rs          # Abstract Syntax Tree definitions
│   ├── parser.rs       # Parser implementation
│   └── interpreter.rs  # Bytecode interpreter/VM
├── examples/
│   └── hello.top       # Example program
├── Cargo.toml          # Rust dependencies
└── README.md          # This file
```

## Development

### Running Tests

```bash
cargo test
```

### Development Build

For faster compilation during development:
```bash
cargo build
./target/debug/top examples/hello.top
```

### Release Build

For optimized production builds:
```bash
cargo build --release
./target/release/top examples/hello.top
```

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Submit pull requests

## License

MIT License - See LICENSE file for details

## Architecture

TopLang uses a modern interpreter architecture:

1. **Lexer** - Tokenizes the source code into meaningful tokens
2. **Parser** - Builds an Abstract Syntax Tree (AST) from tokens
3. **Interpreter** - Directly executes the AST using a tree-walking interpreter

This architecture ensures:
- Fast compilation times
- Cross-platform compatibility
- No external dependencies (like LLVM)
- Easy debugging and error messages

## Why Rust?

The compiler is written in Rust for several reasons:
- **Memory Safety** - No segfaults or memory leaks
- **Cross-Platform** - Single codebase works everywhere
- **Performance** - Near C/C++ performance
- **Modern Tooling** - Cargo makes building and testing easy
- **No Runtime Dependencies** - Static linking produces standalone executables

