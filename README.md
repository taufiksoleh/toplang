# TopLang - A Simple Programming Language

TopLang is a small, expressive programming language designed for learning compiler concepts. It features a simple English-like syntax while maintaining the power to write useful programs.

## Getting Started

### Prerequisites

To build TopLang, you'll need:
- CMake (3.10+)
- A C++ compiler with C++17 support
- LLVM (10.0.0+ recommended)

### Building the Compiler

1. Clone this repository:
```bash
git clone https://github.com/taufiksoleh/toplang.git
cd toplang
```

2. Create a build directory and build the project:
```bash
mkdir build && cd build
cmake ..
make
```

This will create a `top` executable in your build directory.

## Using TopLang

### Basic Usage

To compile and run a TopLang program:

```bash
./top path/to/your_program.top
```

This will compile and immediately execute your program.

### Command-line Options

The `top` compiler supports several command-line options:

