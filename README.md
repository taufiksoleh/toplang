# TopLang

[![CI](https://github.com/taufiksoleh/toplang/workflows/CI/badge.svg)](https://github.com/taufiksoleh/toplang/actions/workflows/ci.yml)
[![Code Quality](https://github.com/taufiksoleh/toplang/workflows/Code%20Quality/badge.svg)](https://github.com/taufiksoleh/toplang/actions/workflows/quality.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](https://www.rust-lang.org/)

TopLang is a programming language with English-like syntax designed for readability and ease of learning. The implementation includes a bytecode virtual machine and an optimizing native code compiler, both written in Rust.

## Features

- Natural language syntax using English keywords
- Native code compilation with GCC optimization backend
- Bytecode virtual machine with NaN-boxing value representation
- Cross-platform support (Linux, macOS, Windows)
- Zero runtime dependencies for compiled executables

## Performance

TopLang supports three execution modes with varying performance characteristics:

| Execution Mode | Avg Time | Speedup vs Interpreter |
|----------------|----------|------------------------|
| Tree-walking interpreter | 1760ms | 1.0x |
| Bytecode VM | 768ms | 2.3x |
| Native compilation | 15ms | 117.3x |

### Benchmark Results

Fibonacci, prime calculation, array operations, nested loops, and factorial benchmarks (Linux x86_64, GCC 13.3.0 with `-O3 -march=native -ffast-math`):

| Benchmark | Interpreter | Native | Speedup |
|-----------|-------------|--------|---------|
| fibonacci | 512ms | 16ms | 31.4x |
| primes | 483ms | 14ms | 33.7x |
| array_sum | 1353ms | 15ms | 86.5x |
| nested_loops | 1101ms | 16ms | 68.4x |
| factorial | 5350ms | 16ms | 319.6x |

Native compilation averages 260ms. See [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md) for detailed analysis.

## Installation

Multiple installation methods are available. Rust is only required when building from source.

### Quick Install

#### Linux and macOS

```bash
curl -sSL https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.sh | bash
```

Manual installation:

```bash
wget https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.sh
chmod +x install.sh
./install.sh
```

#### Windows

PowerShell installation:

```powershell
irm https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.ps1 | iex
```

Manual installation:

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/taufiksoleh/toplang/main/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

### Homebrew (macOS and Linux)

```bash
brew tap taufiksoleh/toplang https://github.com/taufiksoleh/toplang
brew install toplang
topc --version
```

### Pre-built Binaries

Download binaries from the [releases page](https://github.com/taufiksoleh/toplang/releases/latest):

- Linux: `toplang-linux-x64`
- macOS: `toplang-macos-x64`
- Windows: `toplang-windows-x64.exe`

Installation on Linux/macOS:

```bash
chmod +x toplang-*
mv toplang-* /usr/local/bin/topc
```

On Windows, add the binary directory to your PATH environment variable.

### Building from Source

Requirements:
- Rust 1.74 or later ([rustup.rs](https://rustup.rs/))

Build process:

```bash
git clone https://github.com/taufiksoleh/toplang.git
cd toplang
cargo build --release
```

The compiled binary is located at `target/release/topc`. To install system-wide:

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

Execution modes:

```bash
# Interpretation (default)
topc program.top

# Bytecode VM
topc --bytecode program.top
topc --bytecode --nanbox program.top  # With NaN-boxing optimization

# Native compilation
topc --compile program.top
topc --compile program.top -o myapp   # Custom output name
topc --compile -v program.top         # Verbose mode (preserves C source)
```

Additional options:
- `-t, --show-tokens` - Display lexer tokens
- `-a, --show-ast` - Display abstract syntax tree
- `-v, --verbose` - Enable verbose output
- `-h, --help` - Display help
- `-V, --version` - Display version

## Language Syntax

TopLang employs English keywords for improved readability:

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

The TopLang compiler implements a multi-stage compilation pipeline:

1. **Lexer** - Tokenizes source code
2. **Parser** - Constructs an abstract syntax tree (AST)
3. **Execution backends**:
   - Tree-walking interpreter for direct AST evaluation
   - Bytecode compiler and virtual machine
   - Native code generator via C transpilation

## Implementation

The compiler is implemented in Rust, providing:
- Memory safety without garbage collection
- Cross-platform compilation from a single codebase
- Native performance with zero-cost abstractions
- Comprehensive tooling via Cargo

End users do not require Rust installation; pre-built binaries have no runtime dependencies due to static linking.

