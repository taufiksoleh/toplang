# TopLang - A Simple, Human-First Programming Language

[![CI](https://github.com/taufiksoleh/toplang/workflows/CI/badge.svg)](https://github.com/taufiksoleh/toplang/actions/workflows/ci.yml)
[![Code Quality](https://github.com/taufiksoleh/toplang/workflows/Code%20Quality/badge.svg)](https://github.com/taufiksoleh/toplang/actions/workflows/quality.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](https://www.rust-lang.org/)

TopLang is a simple, expressive programming language with natural English-like syntax. It's designed to be easy to learn while being powerful enough to write useful programs. Built with Rust for maximum performance and cross-platform compatibility.

## Features

✨ **Human-Readable Syntax** - Use natural English words instead of symbols

🚀 **Fast** - Written in Rust for blazing fast performance

🌍 **Cross-Platform** - Works on Windows, macOS, and Linux

🎯 **Simple** - Easy to learn and understand

🛠️ **Modern** - Built with modern language design principles

## Installation

TopLang provides multiple installation methods. **No Rust required** for end users!

### Quick Install (Recommended)

#### Linux & macOS

Install with a single command:

```bash
curl -sSL https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.sh | bash
```

Or download and run the script manually:

```bash
wget https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.sh
chmod +x install.sh
./install.sh
```

#### Windows

Using PowerShell:

```powershell
irm https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.ps1 | iex
```

Or download and run manually:

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

### Homebrew (macOS & Linux)

```bash
# Add the tap
brew tap taufiksoleh/toplang https://github.com/taufiksoleh/toplang

# Install toplang
brew install toplang

# Verify installation
topc --version
```

### Pre-built Binaries

Download pre-compiled binaries from the [releases page](https://github.com/taufiksoleh/toplang/releases):

1. Go to [Releases](https://github.com/taufiksoleh/toplang/releases/latest)
2. Download the binary for your platform:
   - **Linux**: `toplang-linux-x64`
   - **macOS**: `toplang-macos-x64`
   - **Windows**: `toplang-windows-x64.exe`
3. Make it executable (Linux/macOS):
   ```bash
   chmod +x toplang-*
   mv toplang-* /usr/local/bin/topc
   ```
4. On Windows, add the directory to your PATH

### Building from Source

If you prefer to build from source or want to contribute:

#### Prerequisites

- **Rust** (1.74 or later) - Install from [rustup.rs](https://rustup.rs/)

#### Build Steps

1. Clone this repository:
```bash
git clone https://github.com/taufiksoleh/toplang.git
cd toplang
```

2. Build with Cargo:
```bash
cargo build --release
```

The compiled `topc` executable will be in `target/release/topc`.

3. Optionally, install it to your system:
```bash
cargo install --path .
```

## Using TopLang

### Basic Usage

To run a TopLang program:

```bash
./target/release/topc path/to/your_program.top
```

Or if you installed it:
```bash
topc path/to/your_program.top
```

### Command-line Options

The `topc` compiler supports several options:

- `-t, --show-tokens` - Display tokens after lexing
- `-a, --show-ast` - Display the Abstract Syntax Tree after parsing
- `-v, --verbose` - Enable verbose output
- `-h, --help` - Display help information
- `-V, --version` - Display version information

Example:
```bash
topc -v examples/hello.top
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

### User Input

```toplang
ask name "What is your name? "
print "Hello, "
print name

ask age "How old are you? "
if age greater than 18 {
    print "You are an adult!"
}
```

### Arrays/Lists

```toplang
# Create a list
var numbers is list 1, 2, 3, 4, 5

# Access elements (0-indexed)
var first is numbers at 0
var third is numbers at 2

# Modify elements
numbers at 1 is 99

# Print the whole array
print numbers  # Output: [1, 99, 3, 4, 5]

# Mixed types are supported
var mixed is list 1, "hello", 3
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
./target/debug/topc examples/hello.top
```

### Release Build

For optimized production builds:
```bash
cargo build --release
./target/release/topc examples/hello.top
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
- No external dependencies
- Easy debugging and error messages

## Why Rust?

The compiler is written in Rust for several reasons:
- **Memory Safety** - No segfaults or memory leaks
- **Cross-Platform** - Single codebase works everywhere
- **Performance** - Fast, native performance
- **Modern Tooling** - Cargo makes building and testing easy
- **No Runtime Dependencies** - Static linking produces standalone executables

**Note**: End users don't need Rust installed! The pre-built binaries are completely standalone and have no dependencies. Rust is only required if you want to build the compiler from source or contribute to development.

