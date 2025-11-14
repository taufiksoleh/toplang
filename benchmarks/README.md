# TopLang Performance Benchmarks

Comprehensive benchmarking suite comparing TopLang's different VM implementations with Python.

## Quick Start

```bash
# Build release binaries
cargo build --release

# Run Rust-based benchmark runner (recommended)
./target/release/benchmark

# Run with Python comparison
./benchmarks/run_all.sh

# Run with historical tracking (JSON output)
./benchmarks/run_with_tracking.sh
```

## Benchmark Suite

### TopLang Benchmarks (`toplang/`)

1. **fibonacci.top** - Iterative Fibonacci (1M iterations with modulo)
2. **primes.top** - Prime number counting up to 100,000
3. **array_sum.top** - Array summation (100K elements, 100 iterations)
4. **nested_loops.top** - Triple nested loop benchmark
5. **factorial.top** - Recursive factorial calculations

### Python Benchmarks (`python/`)

Equivalent Python implementations for performance comparison:
- `fibonacci.py`
- `primes.py`
- `array_sum.py`
- `nested_loops.py`
- `factorial.py`

## Latest Results (NaN Boxing VM)

### Performance Summary

| Benchmark | Interpreter | Bytecode VM | NaN Boxing | Python | vs Python |
|-----------|-------------|-------------|------------|--------|-----------|
| fibonacci | 520ms | 234ms | **175ms** | 123ms | **71%** ✅ |
| primes | 454ms | 287ms | **245ms** | 156ms | **64%** ✅ |
| array_sum | 1348ms | 574ms | **473ms** | 312ms | **66%** ✅ |
| nested_loops | 1117ms | 573ms | **484ms** | - | - |
| factorial | 5444ms | 2822ms | **2340ms** | - | - |

### Speedup Analysis

- **Bytecode VM vs Interpreter**: 2.0x faster
- **NaN Boxing vs Bytecode**: 1.2x faster  
- **Total (NaN Boxing vs Interpreter)**: **2.5x faster** 🚀

### vs Python Performance

**TopLang (NaN Boxing) is currently at ~67% of Python's speed**

This represents a **major improvement** from earlier versions:
- Previous (Interpreter): 26% of Python speed (3.8x slower)
- Current (NaN Boxing): **67% of Python speed** (1.5x slower)

**Goal**: Reach 100%+ of Python speed (faster than Python)

## Benchmark Tools

### 1. Rust Benchmark Runner (`./target/release/benchmark`)

Native benchmark runner with detailed statistics.

**Features:**
- Multiple runs with min/max/avg
- Formatted tables
- Speedup analysis
- VM comparison (Interpreter, Bytecode, NaN Boxing)

**Example output:**
```
╔═══════════════════════════════════════════════════╗
║    TopLang Performance Benchmark Suite           ║
╚═══════════════════════════════════════════════════╝

📊 Benchmarking: Fibonacci
   Interpreter    avg:  520ms  min:  499ms  max:  565ms
   Bytecode VM    avg:  234ms  min:  214ms  max:  299ms
   NaN Boxing     avg:  175ms  min:  165ms  max:  187ms

📈 Average Speedups:
   Bytecode VM vs Interpreter: 2.01x
   NaN Boxing vs Bytecode:     1.22x
   NaN Boxing vs Interpreter:  2.46x (total)
```

### 2. Shell Runner with Python Comparison (`run_all.sh`)

Compare all TopLang VMs with Python baseline.

**Features:**
- Side-by-side comparison
- Color-coded output
- Performance percentage calculation
- Speedup analysis

### 3. Historical Tracking Runner (`run_with_tracking.sh`)

Save detailed results in JSON format.

**Features:**
- JSON output in `results/` directory
- Git commit/branch tracking
- System information
- Comparison with previous runs
- jq integration for analysis

**Output location**: `benchmarks/results/bench_YYYYMMDD_HHMMSS.json`

## CI/CD Integration

Benchmarks run automatically via GitHub Actions:
- ✅ On every push to main/claude/* branches
- ✅ On pull requests
- ✅ Manual workflow dispatch
- ✅ Results uploaded as artifacts
- ✅ Performance summary in PR comments

See: `.github/workflows/benchmark.yml`

## Documentation

For comprehensive documentation, see:
- **[BENCHMARKING.md](../BENCHMARKING.md)** - Complete benchmarking guide
- **[NAN_BOXING_RESULTS.md](../NAN_BOXING_RESULTS.md)** - NaN Boxing implementation details
- **[OPTIMIZATION_SUMMARY.md](../OPTIMIZATION_SUMMARY.md)** - All optimizations applied

## Adding New Benchmarks

1. Create TopLang version in `toplang/my_bench.top`
2. Create Python equivalent in `python/my_bench.py`
3. Add to benchmark runner configuration
4. Run benchmarks to establish baseline

## Performance Targets

| Target | Current | Status |
|--------|---------|--------|
| 2x faster than interpreter | 2.5x | ✅ **Exceeded** |
| 50% of Python speed | 67% | ✅ **Exceeded** |
| 80% of Python speed | 67% | 🎯 **In Progress** |
| 100% of Python speed | 67% | 🚀 **Next Goal** |

## Recent Improvements

- ✅ Implemented NaN Boxing: +19% performance (1.19x speedup)
- ✅ Added inline caching: +46% performance (1.46x speedup)
- ✅ Peephole optimization: +3% performance (1.03x speedup)
- ✅ Constant folding: +5% performance (1.05x speedup)

**Next optimization**: Computed goto dispatch (expected +15-20%)

---

**Last Updated**: 2025-11-14  
**TopLang Version**: 0.0.22  
**Best Performance**: NaN Boxing VM (~67% of Python)
