# Benchmarking Pipeline - Quick Start Guide

## 🎯 Overview

TopLang now has a comprehensive benchmarking system with **3 different tools** for performance testing:

1. **Rust Benchmark Runner** - Fast, detailed native benchmarking
2. **Shell Script with Python** - Compare TopLang vs Python
3. **Historical Tracking** - JSON-based results with git tracking

## ⚡ Quick Commands

### Option 1: Rust Benchmark Runner (Recommended)

```bash
# Build and run
cargo build --release
./target/release/benchmark
```

**Output:**
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

### Option 2: Python Comparison

```bash
./benchmarks/run_all.sh
```

**Output:**
```
┌─────────────────┬──────────┬──────────┬──────────┬──────────┐
│ Benchmark       │ Interp   │ Bytecode │ NanBox   │ Python   │
├─────────────────┼──────────┼──────────┼──────────┼──────────┤
│ fibonacci       │   520ms  │   234ms  │   175ms  │   123ms  │
│ primes          │   454ms  │   287ms  │   245ms  │   156ms  │
│ array_sum       │  1348ms  │   574ms  │   473ms  │   312ms  │
└─────────────────┴──────────┴──────────┴──────────┴──────────┘

🎯 TopLang is currently at 67.4% of Python's speed
```

### Option 3: Historical Tracking

```bash
./benchmarks/run_with_tracking.sh
```

Saves results to `benchmarks/results/bench_TIMESTAMP.json` with:
- Git commit hash
- System info
- Min/max/avg times
- Speedup calculations
- Comparison with previous runs

## 📊 Current Performance

### TopLang NaN Boxing VM Results:

| Benchmark | Time | vs Interpreter | vs Python |
|-----------|------|----------------|-----------|
| **fibonacci** | 175ms | 3.0x faster | 71% |
| **primes** | 245ms | 1.9x faster | 64% |
| **array_sum** | 473ms | 2.8x faster | 66% |
| **nested_loops** | 484ms | 2.3x faster | - |
| **factorial** | 2340ms | 2.3x faster | - |

**Overall: ~2.5x faster than interpreter, ~67% of Python speed**

## 🚀 CI/CD Integration

Benchmarks run automatically on GitHub Actions:
- ✅ Every push to main/claude/* branches
- ✅ Pull requests
- ✅ Manual trigger from Actions tab

**Workflow file:** `.github/workflows/benchmark.yml`

Results are:
- Uploaded as artifacts
- Displayed in job summary
- Posted as PR comments

## 📝 Benchmark Suite

### Included Benchmarks:

1. **fibonacci** (1M iterations) - Tests integer arithmetic and loops
2. **primes** (100K range) - Tests function calls and conditionals
3. **array_sum** (100K × 100) - Tests array operations and memory
4. **nested_loops** - Tests loop optimization
5. **factorial** (recursive) - Tests recursion and stack management

## 🔧 Advanced Usage

### Run Single Benchmark

```bash
# Interpreter
./target/release/topc benchmarks/toplang/fibonacci.top

# Bytecode VM
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode

# NaN Boxing VM
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox
```

### Time Comparison

```bash
time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox
time python3 benchmarks/python/fibonacci.py
```

### Analyze JSON Results

```bash
# Get specific benchmark time
jq '.benchmarks.fibonacci.nanbox.avg_ms' benchmarks/results/bench_*.json

# Get all speedups
jq '.benchmarks | to_entries | map({name: .key, speedup: .value.speedups.nanbox_vs_interp})' \
  benchmarks/results/bench_*.json

# Compare two runs
jq -s '.[0].benchmarks.fibonacci.nanbox.avg_ms - .[1].benchmarks.fibonacci.nanbox.avg_ms' \
  bench_new.json bench_old.json
```

## 📚 Documentation

- **[BENCHMARKING.md](BENCHMARKING.md)** - Complete benchmarking guide (300+ lines)
- **[benchmarks/README.md](benchmarks/README.md)** - Benchmark suite overview
- **[NAN_BOXING_RESULTS.md](NAN_BOXING_RESULTS.md)** - Performance optimization details
- **[OPTIMIZATION_SUMMARY.md](OPTIMIZATION_SUMMARY.md)** - All optimizations summary

## 🎯 Performance Goals

| Goal | Current | Status |
|------|---------|--------|
| 2x faster than interpreter | 2.5x | ✅ **Achieved** |
| 50% of Python speed | 67% | ✅ **Exceeded** |
| 80% of Python speed | 67% | 🎯 **Next Target** |
| 100% of Python speed | 67% | 🚀 **Future Goal** |

## 🔍 What Each Tool Does

### 1. Rust Benchmark Runner
- **Best for:** Quick, accurate benchmarks
- **Output:** Formatted tables, min/max/avg
- **Speed:** Fastest to run
- **Use when:** You want detailed VM comparison

### 2. Shell Script with Python
- **Best for:** Python comparison
- **Output:** Side-by-side tables, percentages
- **Speed:** Slower (runs Python too)
- **Use when:** You want to see how TopLang compares to Python

### 3. Historical Tracking
- **Best for:** Performance tracking over time
- **Output:** JSON files with metadata
- **Speed:** Same as shell script
- **Use when:** You want to track performance changes across commits

## 💡 Tips

1. **Always build in release mode:** `cargo build --release`
2. **Run multiple times:** Benchmarks run 5 iterations by default
3. **Close other apps:** For most accurate results
4. **Use NaN Boxing VM:** Fastest TopLang implementation
5. **Check git commit:** Historical tracking includes commit hash

## 🐛 Troubleshooting

### "Command not found: bc" or "jq"
```bash
# Ubuntu/Debian
sudo apt-get install bc jq

# macOS
brew install bc jq
```

### Python benchmarks not running
```bash
# Check Python installation
python3 --version

# Make scripts executable
chmod +x benchmarks/python/*.py
```

### Inconsistent results
- Close background applications
- Check CPU isn't throttling
- Run on idle system

## 📈 Interpreting Results

### Speedup Metrics:

- **2.0x+** = Bytecode VM is working well ✅
- **1.15x+** = NaN Boxing optimization is effective ✅
- **>0.8x** vs Python = Getting close to Python speed 🎯
- **>1.0x** vs Python = Faster than Python! 🚀

### Current Status:
- **Total speedup:** 2.5x vs interpreter ✅
- **vs Python:** 0.67x (67% of Python speed) 🎯
- **Gap to close:** ~30% to match Python

---

**Ready to benchmark?** Run: `cargo build --release && ./target/release/benchmark`

For questions, see [BENCHMARKING.md](BENCHMARKING.md) or open an issue!
