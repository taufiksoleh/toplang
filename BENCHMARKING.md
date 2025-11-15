# TopLang Benchmarking

Performance benchmarking system for TopLang with multi-VM comparison, Python baseline, historical tracking, and CI/CD integration.

## Overview

The benchmarking pipeline provides:

- Multi-VM comparison (Interpreter, Bytecode VM, NaN Boxing VM)
- Python baseline for performance comparison
- Historical tracking with JSON-based results and git commit hashes
- GitHub Actions integration for continuous benchmarking
- Detailed performance reports and analysis

## Structure

```
benchmarks/
├── toplang/           # TopLang benchmark programs
│   ├── fibonacci.top
│   ├── primes.top
│   ├── array_sum.top
│   ├── nested_loops.top
│   └── factorial.top
├── python/            # Equivalent Python benchmarks
│   ├── fibonacci.py
│   ├── primes.py
│   └── array_sum.py
├── results/           # Benchmark results (JSON format)
│   └── bench_YYYYMMDD_HHMMSS.json
├── run_all.sh         # Simple benchmark runner
└── run_with_tracking.sh  # Advanced runner with JSON output
```

## Quick Start

### Build Release Binaries

```bash
cargo build --release
```

### Run Benchmarks

```bash
chmod +x benchmarks/run_all.sh
./benchmarks/run_all.sh
```

Example output:
```
TopLang vs Python Performance Benchmark

Benchmarking: fibonacci
   TopLang (Interpreter)...  520ms
   TopLang (Bytecode)...     198ms
   TopLang (NaN Boxing)...   172ms
   Python 3...               123ms

Benchmark       | Interp  | Bytecode | NanBox  | Python
----------------|---------|----------|---------|--------
fibonacci       | 520ms   | 198ms    | 172ms   | 123ms
primes          | 720ms   | 275ms    | 239ms   | 156ms
array_sum       | 1450ms  | 592ms    | 478ms   | 312ms

Average Speedups:
   Bytecode vs Interpreter:  2.63x
   NaN Boxing vs Bytecode:   1.19x
   TopLang vs Python:        1.40x (71.4% of Python speed)
```

### Run with Historical Tracking

```bash
chmod +x benchmarks/run_with_tracking.sh
./benchmarks/run_with_tracking.sh
```

Results are saved to `benchmarks/results/bench_TIMESTAMP.json` for comparison with previous runs.

### Rust Benchmark Runner

```bash
cargo build --release --bin benchmark
./target/release/benchmark
```

Provides detailed tables with minimum, maximum, and average times for each benchmark.

## Benchmark Suite

### Current Benchmarks

1. **fibonacci** - Iterative Fibonacci with modulo (1M iterations)
   - Tests: Integer arithmetic, loop performance, variable updates
   - Good indicator of: Basic VM overhead, arithmetic optimization

2. **primes** - Prime number finder using trial division (100K range)
   - Tests: Function calls, nested loops, conditional logic
   - Good indicator of: Call overhead, branch prediction

3. **array_sum** - Array creation and summation (100K elements, 100 iterations)
   - Tests: Array operations, memory access patterns
   - Good indicator of: Heap allocation, cache locality

4. **nested_loops** - Triple nested loops with arithmetic
   - Tests: Loop optimization, register allocation
   - Good indicator of: Loop unrolling potential

5. **factorial** - Recursive factorial calculation
   - Tests: Function call overhead, stack management
   - Good indicator of: Recursion performance

### Adding New Benchmarks

1. Create TopLang benchmark in `benchmarks/toplang/`:
```bash
vim benchmarks/toplang/my_benchmark.top
```

2. Create equivalent Python benchmark in `benchmarks/python/`:
```bash
vim benchmarks/python/my_benchmark.py
```

3. Add to benchmark configuration in `run_all.sh`:
```bash
BENCHMARKS=(
    # ... existing benchmarks ...
    "my_benchmark:benchmarks/toplang/my_benchmark.top:benchmarks/python/my_benchmark.py"
)
```

## Results Format

JSON results are saved with the following structure:

```json
{
  "timestamp": "2025-11-14T10:30:45-05:00",
  "git_commit": "872446d...",
  "git_branch": "claude/benchmark-performance",
  "system": {
    "os": "Linux",
    "arch": "x86_64",
    "kernel": "4.4.0"
  },
  "runs_per_benchmark": 5,
  "benchmarks": {
    "fibonacci": {
      "interpreter": { "avg_ms": 520, "min_ms": 515, "max_ms": 525 },
      "bytecode": { "avg_ms": 198, "min_ms": 195, "max_ms": 202 },
      "nanbox": { "avg_ms": 172, "min_ms": 170, "max_ms": 175 },
      "python": { "avg_ms": 123, "min_ms": 121, "max_ms": 125 },
      "speedups": {
        "bytecode_vs_interp": 2.626,
        "nanbox_vs_bytecode": 1.151,
        "nanbox_vs_interp": 3.023,
        "nanbox_vs_python": 1.398
      }
    }
  }
}
```

## CI/CD Integration

### GitHub Actions

Benchmarks run automatically on:
- Push to main/master/claude/* branches
- Pull requests
- Manual workflow dispatch

#### Workflow Features:

1. **Automatic Execution**: Runs on every push
2. **Artifact Upload**: Results saved as GitHub artifacts
3. **PR Comments**: Performance summary posted to PRs
4. **Historical Comparison**: Compare with previous runs

#### Viewing Results:

1. Go to Actions tab in GitHub
2. Select "Performance Benchmarks" workflow
3. Click on a run
4. View summary or download artifacts

### Local CI Testing

Test the CI workflow locally with [act](https://github.com/nektos/act):

```bash
# Install act
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run benchmark workflow
act -j benchmark
```

## Interpreting Results

### Speedup Metrics

- Bytecode vs Interpreter: Shows benefit of bytecode compilation
  - Target: >2.0x (achieved: 2.6x)

- NaN Boxing vs Bytecode: Shows benefit of NaN boxing optimization
  - Target: >1.15x (achieved: 1.19x)

- Total Speedup: Combined effect of all optimizations
  - Target: >3.0x (achieved: 3.09x)

- vs Python: Performance relative to CPython
  - Target: >0.8x (80% of Python) (current: 71%)
  - Goal: >1.0x (faster than Python)

### Performance Targets

| VM Type | Target Speedup | Current | Status |
|---------|---------------|---------|--------|
| Bytecode VM | 2.0x vs Interp | 2.6x | Exceeded |
| NaN Boxing | 1.4x vs Bytecode | 1.19x | Close |
| Total | 3.0x vs Interp | 3.09x | Achieved |
| vs Python | 80% (0.8x) | 71% (0.71x) | In Progress |

## Advanced Usage

### Custom Benchmark Runs

Run specific benchmark with custom parameters:

```bash
# Single benchmark, 10 runs
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox

# With timing
time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox
```

### Comparing VM Implementations

```bash
# Interpreter
./target/release/topc benchmarks/toplang/fibonacci.top

# Bytecode VM
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode

# NaN Boxing VM
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox
```

### Python Comparison

```bash
# Run Python equivalent
python3 benchmarks/python/fibonacci.py

# Time comparison
time python3 benchmarks/python/fibonacci.py
time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox
```

### Analysis with jq

```bash
# Get average speedup for specific benchmark
jq '.benchmarks.fibonacci.speedups.nanbox_vs_interp' benchmarks/results/bench_*.json

# Get all NaN Boxing times
jq '.benchmarks | to_entries | map({name: .key, time: .value.nanbox.avg_ms})' benchmarks/results/bench_*.json

# Compare two runs
jq -s '.[0].benchmarks.fibonacci.nanbox.avg_ms - .[1].benchmarks.fibonacci.nanbox.avg_ms' \
  bench_new.json bench_old.json
```

## Best Practices

### For Accurate Benchmarks:

1. **Build in Release Mode**: Always use `--release`
   ```bash
   cargo build --release
   ```

2. **Multiple Runs**: Use at least 5 runs to account for variance
   ```bash
   RUNS=5  # In benchmark scripts
   ```

3. **System Load**: Run on idle system
   ```bash
   # Check system load
   uptime

   # Close other applications
   ```

4. **Thermal Throttling**: Ensure CPU isn't throttling
   ```bash
   # Monitor CPU frequency
   watch -n1 "cat /proc/cpuinfo | grep MHz"
   ```

5. **Reproducibility**: Include git commit hash in results
   ```bash
   git rev-parse HEAD  # Included in run_with_tracking.sh
   ```

### For Fair Comparison:

1. **Same Algorithms**: Python and TopLang should use identical logic
2. **Same Data Sizes**: Use same input sizes across languages
3. **Same Environment**: Run benchmarks on same machine
4. **Warm-up Runs**: Consider JIT warm-up for some implementations

## Troubleshooting

### "Command not found: bc"

```bash
# Ubuntu/Debian
sudo apt-get install bc

# macOS
brew install bc
```

### "Command not found: jq"

```bash
# Ubuntu/Debian
sudo apt-get install jq

# macOS
brew install jq
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
- Run multiple times and take average
- Check CPU frequency scaling
- Ensure machine isn't thermal throttling

## References

- [Performance Roadmap](PERFORMANCE_ROADMAP.md) - Optimization strategies
- [NaN Boxing Results](NAN_BOXING_RESULTS.md) - NaN boxing implementation details
- [Benchmark Results](BENCHMARK_RESULTS.md) - Detailed benchmark analysis

## Future Enhancements

- [ ] Add memory profiling (heap usage, allocations)
- [ ] Implement regression detection
- [ ] Add performance visualization graphs
- [ ] Support for more baseline languages (Ruby, JavaScript)
- [ ] Micro-benchmarks for specific operations
- [ ] Flamegraph integration for profiling
- [ ] Continuous benchmarking dashboard
- [ ] Performance badges in README

## Support

For benchmarking questions or issues:
1. Review this documentation
2. Examine benchmark scripts for examples
3. Open an issue with benchmark results attached
