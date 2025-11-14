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

- `run_native_benchmarks.sh` - Native compilation benchmarks
- `run_complete_benchmarks.sh` - Complete suite with Python comparison
- `run_vm_benchmarks.sh` - VM vs Interpreter comparison

## Benchmark Programs

All benchmark programs are in `benchmarks/toplang/`:

- `fibonacci.top` - 37.8x faster when natively compiled
- `primes.top` - 34.0x faster
- `array_sum.top` - 99.3x faster
- `nested_loops.top` - 80.7x faster  
- `factorial.top` - 348.9x faster! 🔥
