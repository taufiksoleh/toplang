#!/bin/bash
# Benchmark runner with historical tracking and JSON output

set -e

RESULTS_DIR="benchmarks/results"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULT_FILE="$RESULTS_DIR/bench_$TIMESTAMP.json"

mkdir -p "$RESULTS_DIR"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║    TopLang Performance Benchmark (with tracking)         ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "Results will be saved to: $RESULT_FILE"
echo ""

# Check prerequisites
if [ ! -f "./target/release/topc" ]; then
    echo "Error: topc binary not found. Run 'cargo build --release' first"
    exit 1
fi

# Benchmark configurations
declare -A bench_files=(
    ["fibonacci"]="benchmarks/toplang/fibonacci.top"
    ["primes"]="benchmarks/toplang/primes.top"
    ["array_sum"]="benchmarks/toplang/array_sum.top"
)

RUNS=5

# Function to run benchmark and return time in ms
run_bench() {
    local cmd="$1"
    local runs=$2
    local times=()

    for ((i=1; i<=runs; i++)); do
        start=$(date +%s%N)
        eval "$cmd" > /dev/null 2>&1
        end=$(date +%s%N)
        duration=$(( (end - start) / 1000000 ))
        times+=($duration)
    done

    # Calculate average, min, max
    local total=0
    local min=999999999
    local max=0

    for time in "${times[@]}"; do
        total=$((total + time))
        if [ $time -lt $min ]; then min=$time; fi
        if [ $time -gt $max ]; then max=$time; fi
    done

    local avg=$((total / runs))
    echo "$avg $min $max"
}

# Start JSON output
cat > "$RESULT_FILE" << EOF
{
  "timestamp": "$(date -Iseconds)",
  "git_commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
  "git_branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')",
  "system": {
    "os": "$(uname -s)",
    "arch": "$(uname -m)",
    "kernel": "$(uname -r)"
  },
  "runs_per_benchmark": $RUNS,
  "benchmarks": {
EOF

first=true

for bench_name in "${!bench_files[@]}"; do
    bench_file="${bench_files[$bench_name]}"

    if [ ! -f "$bench_file" ]; then
        continue
    fi

    echo "📊 Running: $bench_name ($RUNS runs)"

    # Interpreter
    echo -n "   Interpreter...  "
    read -r avg_i min_i max_i <<< $(run_bench "./target/release/topc $bench_file" $RUNS)
    echo "${avg_i}ms"

    # Bytecode
    echo -n "   Bytecode...     "
    read -r avg_b min_b max_b <<< $(run_bench "./target/release/topc $bench_file --bytecode" $RUNS)
    echo "${avg_b}ms"

    # NaN Boxing
    echo -n "   NaN Boxing...   "
    read -r avg_n min_n max_n <<< $(run_bench "./target/release/topc $bench_file --bytecode --nanbox" $RUNS)
    echo "${avg_n}ms"

    # Native (if available)
    native_file="benchmarks/native/${bench_name}"
    if [ -f "$native_file" ]; then
        echo -n "   Native (Rust)... "
        read -r avg_nat min_nat max_nat <<< $(run_bench "$native_file" $RUNS)
        echo "${avg_nat}ms"
        has_native=true
    else
        avg_nat=0
        min_nat=0
        max_nat=0
        has_native=false
    fi

    # Python (if available)
    python_file="benchmarks/python/${bench_name}.py"
    if [ -f "$python_file" ] && command -v python3 &> /dev/null; then
        echo -n "   Python...       "
        read -r avg_p min_p max_p <<< $(run_bench "python3 $python_file" $RUNS)
        echo "${avg_p}ms"
        has_python=true
    else
        avg_p=0
        min_p=0
        max_p=0
        has_python=false
    fi

    # Calculate speedups (ensure leading zero for JSON compatibility)
    speedup_bytecode=$(echo "scale=3; $avg_i / $avg_b" | bc | awk '{printf "%.3f", $0}')
    speedup_nanbox=$(echo "scale=3; $avg_b / $avg_n" | bc | awk '{printf "%.3f", $0}')
    speedup_total=$(echo "scale=3; $avg_i / $avg_n" | bc | awk '{printf "%.3f", $0}')

    if [ "$has_native" = true ] && [ "$avg_nat" != "0" ]; then
        vs_native=$(echo "scale=3; $avg_n / $avg_nat" | bc | awk '{printf "%.3f", $0}')
        native_json="\"native\": { \"avg_ms\": $avg_nat, \"min_ms\": $min_nat, \"max_ms\": $max_nat },"
    else
        vs_native="0.000"
        native_json=""
    fi

    if [ "$has_python" = true ]; then
        vs_python=$(echo "scale=3; $avg_n / $avg_p" | bc | awk '{printf "%.3f", $0}')
        python_json="\"python\": { \"avg_ms\": $avg_p, \"min_ms\": $min_p, \"max_ms\": $max_p },"
    else
        vs_python="0.000"
        python_json=""
    fi

    # Add comma if not first entry
    if [ "$first" = false ]; then
        echo "," >> "$RESULT_FILE"
    fi
    first=false

    # Write JSON entry
    cat >> "$RESULT_FILE" << EOF
    "$bench_name": {
      "interpreter": { "avg_ms": $avg_i, "min_ms": $min_i, "max_ms": $max_i },
      "bytecode": { "avg_ms": $avg_b, "min_ms": $min_b, "max_ms": $max_b },
      "nanbox": { "avg_ms": $avg_n, "min_ms": $min_n, "max_ms": $max_n },
      $native_json
      $python_json
      "speedups": {
        "bytecode_vs_interp": $speedup_bytecode,
        "nanbox_vs_bytecode": $speedup_nanbox,
        "nanbox_vs_interp": $speedup_total,
        "nanbox_vs_native": $vs_native,
        "nanbox_vs_python": $vs_python
      }
    }
EOF

    echo ""
done

# Close JSON
cat >> "$RESULT_FILE" << EOF

  }
}
EOF

echo ""
echo "✅ Results saved to: $RESULT_FILE"
echo ""

# Display summary from JSON
if command -v jq &> /dev/null; then
    echo "📈 Summary:"
    echo ""

    # Calculate average speedups
    avg_bytecode=$(jq '.benchmarks | to_entries | map(.value.speedups.bytecode_vs_interp) | add / length' "$RESULT_FILE")
    avg_nanbox=$(jq '.benchmarks | to_entries | map(.value.speedups.nanbox_vs_bytecode) | add / length' "$RESULT_FILE")
    avg_total=$(jq '.benchmarks | to_entries | map(.value.speedups.nanbox_vs_interp) | add / length' "$RESULT_FILE")

    echo "   Average Bytecode speedup:  ${avg_bytecode}x"
    echo "   Average NaN Boxing speedup: ${avg_nanbox}x"
    echo "   Total speedup:              ${avg_total}x"
    echo ""
fi

# Generate comparison if previous results exist
LATEST_PREVIOUS=$(ls -t "$RESULTS_DIR"/bench_*.json 2>/dev/null | sed -n '2p')

if [ -n "$LATEST_PREVIOUS" ] && command -v jq &> /dev/null; then
    echo "📊 Comparison with previous run:"
    echo "   Previous: $(basename "$LATEST_PREVIOUS")"
    echo ""

    # Compare key metrics
    for bench_name in "${!bench_files[@]}"; do
        curr_time=$(jq -r ".benchmarks.${bench_name}.nanbox.avg_ms // 0" "$RESULT_FILE")
        prev_time=$(jq -r ".benchmarks.${bench_name}.nanbox.avg_ms // 0" "$LATEST_PREVIOUS")

        if [ "$curr_time" != "0" ] && [ "$prev_time" != "0" ]; then
            diff=$((curr_time - prev_time))
            percent=$(echo "scale=1; ($diff / $prev_time) * 100" | bc | awk '{printf "%.1f", $0}')

            if [ $diff -lt 0 ]; then
                echo "   $bench_name: FASTER by ${diff#-}ms (${percent#-}%)"
            elif [ $diff -gt 0 ]; then
                echo "   $bench_name: slower by ${diff}ms (+${percent}%)"
            else
                echo "   $bench_name: same performance"
            fi
        fi
    done
    echo ""
fi

echo "✅ Benchmark complete!"
