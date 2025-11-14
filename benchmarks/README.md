# TopLang Benchmarks

Comprehensive benchmark suite for TopLang performance testing.

## Quick Start

```bash
# Run native compilation benchmarks
./benchmarks/run_native_benchmarks.sh

# Run complete benchmarks (includes Python)
./benchmarks/run_complete_benchmarks.sh
```

## Latest Results

**Native compilation is 133.5x faster than interpreter!**

| Mode | Avg Time | Speedup |
|------|----------|---------|
| Interpreter | 1.736s | 1.0x |
| Bytecode VM | 0.761s | 2.3x |
| **Native** | **0.013s** | **133.5x** 🚀 |

See [BENCHMARK_RESULTS.md](../BENCHMARK_RESULTS.md) for detailed analysis.

## Benchmark Scripts

- `run_native_benchmarks.sh` - Native compilation benchmarks **with historical tracking**
- `run_complete_benchmarks.sh` - Complete suite with Python comparison
- `run_vm_benchmarks.sh` - VM vs Interpreter comparison
- `show_performance_history.sh` - View historical performance data

## Historical Performance Tracking

The native benchmark runner automatically saves results to a CSV file for tracking performance over time:

```bash
# Run benchmarks (automatically saves to CSV)
./benchmarks/run_native_benchmarks.sh

# View performance history
./benchmarks/show_performance_history.sh

# Raw CSV data (can be imported to Excel/Google Sheets)
cat benchmarks/results/performance_history.csv
```

### CSV Columns
- **Date/Time** - When the benchmark was run
- **Commit** - Git commit hash
- **Branch** - Git branch name
- **Benchmark** - Benchmark name (or "AVERAGE")
- **Interpreter_ms** - Interpreter time in milliseconds
- **VM_ms** - Bytecode VM time in milliseconds
- **Native_ms** - Native compiled time in milliseconds
- **Compile_ms** - Compilation time in milliseconds
- **Native_vs_Interp** - Speedup vs interpreter
- **Native_vs_VM** - Speedup vs VM
- **System** - Operating system and architecture
- **Compiler** - C compiler version

### Example: Track Performance Improvements

```bash
# Run benchmarks on current commit
./benchmarks/run_native_benchmarks.sh

# Make code changes...
git commit -m "optimization improvements"

# Run benchmarks again
./benchmarks/run_native_benchmarks.sh

# View performance comparison
./benchmarks/show_performance_history.sh
```

The history viewer will show if your changes improved or regressed performance!

## Benchmark Programs

All benchmark programs are in `benchmarks/toplang/`:

- `fibonacci.top` - 37.8x faster when natively compiled
- `primes.top` - 34.0x faster
- `array_sum.top` - 99.3x faster
- `nested_loops.top` - 80.7x faster  
- `factorial.top` - 348.9x faster! 🔥
