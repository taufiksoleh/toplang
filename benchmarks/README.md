# TopLang Performance Benchmarks

This directory contains performance benchmarks comparing TopLang with Python.

## Quick Start

Run all benchmarks:
```bash
./benchmarks/run_benchmarks.sh
```

Results will be saved to `benchmarks/results/` with timestamp.

## Benchmark Suite

### Benchmarks Included

1. **Fibonacci** - Iterative fibonacci calculation (1M iterations with modulo)
2. **Primes** - Prime number counting up to 50,000
3. **Array Sum** - Summing integers from 0 to 5 million
4. **Nested Loops** - 5,000 x 1,000 nested loop iterations
5. **Factorial** - Calculate factorials for numbers 0-5000 with modulo

### Latest Results

| Benchmark | TopLang (s) | Python (s) | Relative Performance |
|-----------|-------------|------------|---------------------|
| fibonacci | 0.537 | 0.131 | Python 4.1x faster |
| primes | 0.475 | 0.180 | Python 2.6x faster |
| array_sum | 1.331 | 0.326 | Python 4.1x faster |
| nested_loops | 1.148 | 0.303 | Python 3.8x faster |
| factorial | 5.552 | 1.202 | Python 4.6x faster |

**Average**: Python is approximately **3.8x faster** than TopLang across these benchmarks.

## Analysis

### Current Performance Profile

TopLang is currently implemented as a tree-walking interpreter, which explains the performance characteristics:

- **Why slower than Python?**
  - Python uses a bytecode compiler + VM with extensive optimizations
  - TopLang directly interprets the AST without compilation
  - No JIT compilation or runtime optimizations yet

### Optimization Opportunities

1. **Bytecode Compilation** - Compile to bytecode instead of interpreting AST
2. **Stack-based VM** - Implement an efficient stack-based virtual machine
3. **Constant Folding** - Optimize constant expressions at parse time
4. **Loop Optimization** - Special handling for common loop patterns
5. **Type Specialization** - Cache type information for faster operations

## Directory Structure

```
benchmarks/
├── README.md           # This file
├── run_benchmarks.sh   # Main benchmark runner script
├── toplang/            # TopLang benchmark programs
│   ├── fibonacci.top
│   ├── primes.top
│   ├── array_sum.top
│   ├── nested_loops.top
│   └── factorial.top
├── python/             # Equivalent Python programs
│   ├── fibonacci.py
│   ├── primes.py
│   ├── array_sum.py
│   ├── nested_loops.py
│   └── factorial.py
└── results/            # Benchmark results with timestamps
```

## Adding New Benchmarks

1. Create equivalent programs in `toplang/` and `python/`
2. Add benchmark name to `BENCHMARKS` array in `run_benchmarks.sh`
3. Ensure programs produce identical output for verification
4. Run the benchmark suite

## Notes

- Each benchmark runs 3 times and reports the average
- Results may vary based on system load and hardware
- Benchmarks use consistent problem sizes for fair comparison
- All results are averaged over multiple runs for accuracy

## Future Work

- Add more language comparisons (JavaScript, Ruby, Lua)
- Include memory usage benchmarks
- Add real-world application benchmarks
- Implement and measure optimization improvements
