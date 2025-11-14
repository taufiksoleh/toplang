#!/bin/bash
# Comprehensive benchmark runner comparing TopLang vs Python

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║        TopLang vs Python Performance Benchmark            ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if binaries exist
if [ ! -f "./target/release/topc" ]; then
    echo -e "${RED}Error: topc binary not found. Run 'cargo build --release' first${NC}"
    exit 1
fi

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo -e "${YELLOW}Warning: python3 not found. Skipping Python benchmarks${NC}"
    RUN_PYTHON=false
else
    RUN_PYTHON=true
fi

# Make Python scripts executable
chmod +x benchmarks/python/*.py 2>/dev/null || true

# Benchmark configurations
BENCHMARKS=(
    "fibonacci:benchmarks/toplang/fibonacci.top:benchmarks/python/fibonacci.py"
    "primes:benchmarks/toplang/primes.top:benchmarks/python/primes.py"
    "array_sum:benchmarks/toplang/array_sum.top:benchmarks/python/array_sum.py"
)

RUNS=5

# Function to run a benchmark multiple times and get average
run_benchmark() {
    local cmd="$1"
    local runs=$2
    local total=0

    for ((i=1; i<=runs; i++)); do
        start=$(date +%s%N)
        eval "$cmd" > /dev/null 2>&1
        end=$(date +%s%N)
        duration=$(( (end - start) / 1000000 )) # Convert to milliseconds
        total=$((total + duration))
    done

    avg=$((total / runs))
    echo "$avg"
}

echo -e "${BLUE}Running benchmarks (${RUNS} runs each)...${NC}"
echo ""

# Store results
declare -A results

# Run TopLang benchmarks
for bench_config in "${BENCHMARKS[@]}"; do
    IFS=':' read -r name toplang_file python_file <<< "$bench_config"

    if [ ! -f "$toplang_file" ]; then
        echo -e "${YELLOW}⚠️  Skipping $name (TopLang file not found)${NC}"
        continue
    fi

    echo "📊 Benchmarking: $name"

    # TopLang Interpreter
    echo -n "   TopLang (Interpreter)... "
    time_interp=$(run_benchmark "./target/release/topc $toplang_file" $RUNS)
    echo -e "${GREEN}${time_interp}ms${NC}"
    results["${name}_interp"]=$time_interp

    # TopLang Bytecode
    echo -n "   TopLang (Bytecode)...    "
    time_bytecode=$(run_benchmark "./target/release/topc $toplang_file --bytecode" $RUNS)
    echo -e "${GREEN}${time_bytecode}ms${NC}"
    results["${name}_bytecode"]=$time_bytecode

    # TopLang NaN Boxing
    echo -n "   TopLang (NaN Boxing)...  "
    time_nanbox=$(run_benchmark "./target/release/topc $toplang_file --bytecode --nanbox" $RUNS)
    echo -e "${GREEN}${time_nanbox}ms${NC}"
    results["${name}_nanbox"]=$time_nanbox

    # Python
    if [ "$RUN_PYTHON" = true ] && [ -f "$python_file" ]; then
        echo -n "   Python 3...              "
        time_python=$(run_benchmark "python3 $python_file" $RUNS)
        echo -e "${GREEN}${time_python}ms${NC}"
        results["${name}_python"]=$time_python
    fi

    echo ""
done

echo ""
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                    Results Summary                        ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Print results table
printf "┌─────────────────┬──────────┬──────────┬──────────┬──────────┐\n"
printf "│ %-15s │ %-8s │ %-8s │ %-8s │ %-8s │\n" "Benchmark" "Interp" "Bytecode" "NanBox" "Python"
printf "├─────────────────┼──────────┼──────────┼──────────┼──────────┤\n"

for bench_config in "${BENCHMARKS[@]}"; do
    IFS=':' read -r name _ _ <<< "$bench_config"

    interp="${results[${name}_interp]:-N/A}"
    bytecode="${results[${name}_bytecode]:-N/A}"
    nanbox="${results[${name}_nanbox]:-N/A}"
    python="${results[${name}_python]:-N/A}"

    printf "│ %-15s │ %6sms │ %6sms │ %6sms │ %6sms │\n" \
        "$name" "$interp" "$bytecode" "$nanbox" "$python"
done

printf "└─────────────────┴──────────┴──────────┴──────────┴──────────┘\n"
echo ""

# Calculate and display speedups
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                  Speedup Analysis                         ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

printf "┌─────────────────┬──────────────┬──────────────┬──────────────┐\n"
printf "│ %-15s │ %-12s │ %-12s │ %-12s │\n" "Benchmark" "Byte/Interp" "NanBox/Byte" "NanBox/Python"
printf "├─────────────────┼──────────────┼──────────────┼──────────────┤\n"

total_speedup1=0
total_speedup2=0
total_speedup3=0
count=0

for bench_config in "${BENCHMARKS[@]}"; do
    IFS=':' read -r name _ _ <<< "$bench_config"

    interp="${results[${name}_interp]}"
    bytecode="${results[${name}_bytecode]}"
    nanbox="${results[${name}_nanbox]}"
    python="${results[${name}_python]}"

    if [ -n "$interp" ] && [ -n "$bytecode" ] && [ "$interp" != "N/A" ] && [ "$bytecode" != "N/A" ]; then
        speedup1=$(echo "scale=2; $interp / $bytecode" | bc)
    else
        speedup1="N/A"
    fi

    if [ -n "$bytecode" ] && [ -n "$nanbox" ] && [ "$bytecode" != "N/A" ] && [ "$nanbox" != "N/A" ]; then
        speedup2=$(echo "scale=2; $bytecode / $nanbox" | bc)
    else
        speedup2="N/A"
    fi

    if [ -n "$nanbox" ] && [ -n "$python" ] && [ "$nanbox" != "N/A" ] && [ "$python" != "N/A" ]; then
        speedup3=$(echo "scale=2; $nanbox / $python" | bc)
        percent=$(echo "scale=1; ($nanbox / $python) * 100" | bc)
    else
        speedup3="N/A"
        percent="N/A"
    fi

    printf "│ %-15s │ %10sx │ %10sx │ %10sx │\n" \
        "$name" "$speedup1" "$speedup2" "$speedup3"

    # Accumulate for averages
    if [ "$speedup1" != "N/A" ]; then
        total_speedup1=$(echo "$total_speedup1 + $speedup1" | bc)
        ((count++)) || true
    fi
    if [ "$speedup2" != "N/A" ]; then
        total_speedup2=$(echo "$total_speedup2 + $speedup2" | bc)
    fi
    if [ "$speedup3" != "N/A" ]; then
        total_speedup3=$(echo "$total_speedup3 + $speedup3" | bc)
    fi
done

printf "└─────────────────┴──────────────┴──────────────┴──────────────┘\n"
echo ""

# Print averages
if [ $count -gt 0 ]; then
    avg1=$(echo "scale=2; $total_speedup1 / $count" | bc)
    avg2=$(echo "scale=2; $total_speedup2 / $count" | bc)
    avg3=$(echo "scale=2; $total_speedup3 / $count" | bc)
    avg_percent=$(echo "scale=1; ($avg3) * 100" | bc)

    echo -e "${BLUE}📈 Average Speedups:${NC}"
    echo -e "   Bytecode vs Interpreter:  ${GREEN}${avg1}x${NC}"
    echo -e "   NaN Boxing vs Bytecode:   ${GREEN}${avg2}x${NC}"

    if [ "$RUN_PYTHON" = true ]; then
        echo -e "   TopLang vs Python:        ${GREEN}${avg3}x${NC} (${avg_percent}% of Python speed)"

        # Determine color based on performance
        if (( $(echo "$avg_percent > 80" | bc -l) )); then
            COLOR=$GREEN
        elif (( $(echo "$avg_percent > 50" | bc -l) )); then
            COLOR=$YELLOW
        else
            COLOR=$RED
        fi

        echo ""
        echo -e "${COLOR}🎯 TopLang is currently at ${avg_percent}% of Python's speed${NC}"
    fi
fi

echo ""
echo "✅ Benchmark complete!"
echo ""
