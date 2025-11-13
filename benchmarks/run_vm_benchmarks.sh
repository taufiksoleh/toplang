#!/bin/bash

# Benchmark runner comparing Interpreter vs VM vs Python

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Results directory
RESULTS_DIR="benchmarks/results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="${RESULTS_DIR}/vm_results_${TIMESTAMP}.txt"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}TopLang Interpreter vs VM vs Python${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if topc exists
if [ ! -f "target/release/topc" ]; then
    echo -e "${YELLOW}Building TopLang compiler...${NC}"
    cargo build --release
    echo -e "${GREEN}Build complete!${NC}"
    echo ""
fi

# Benchmarks to run
BENCHMARKS=("fibonacci" "primes" "array_sum" "nested_loops" "factorial")

# Initialize results file
echo "TopLang Interpreter vs VM vs Python Benchmark Results" > "$RESULTS_FILE"
echo "Generated: $(date)" >> "$RESULTS_FILE"
echo "========================================" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Print header
printf "${BLUE}%-20s %-15s %-15s %-15s %-15s %-15s${NC}\n" "Benchmark" "Interpreter(s)" "VM (s)" "Python (s)" "VM Speedup" "vs Python"
echo "-----------------------------------------------------------------------------------------------------------"

# Run benchmarks
for benchmark in "${BENCHMARKS[@]}"; do
    echo -e "${YELLOW}Running: $benchmark${NC}"

    TOPLANG_FILE="benchmarks/toplang/${benchmark}.top"
    PYTHON_FILE="benchmarks/python/${benchmark}.py"

    # Time Interpreter (3 runs, take average)
    INTERP_TOTAL=0
    for i in {1..3}; do
        START=$(date +%s.%N)
        ./target/release/topc "$TOPLANG_FILE" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        INTERP_TOTAL=$(echo "$INTERP_TOTAL + $DURATION" | bc)
    done
    INTERP_TIME=$(echo "scale=3; $INTERP_TOTAL / 3" | bc)

    # Time VM (3 runs, take average)
    VM_TOTAL=0
    for i in {1..3}; do
        START=$(date +%s.%N)
        ./target/release/topc "$TOPLANG_FILE" --bytecode > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        VM_TOTAL=$(echo "$VM_TOTAL + $DURATION" | bc)
    done
    VM_TIME=$(echo "scale=3; $VM_TOTAL / 3" | bc)

    # Time Python (3 runs, take average)
    PYTHON_TOTAL=0
    for i in {1..3}; do
        START=$(date +%s.%N)
        python3 "$PYTHON_FILE" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        PYTHON_TOTAL=$(echo "$PYTHON_TOTAL + $DURATION" | bc)
    done
    PYTHON_TIME=$(echo "scale=3; $PYTHON_TOTAL / 3" | bc)

    # Calculate speedups
    if (( $(echo "$VM_TIME > 0" | bc -l) )); then
        VM_SPEEDUP=$(echo "scale=2; $INTERP_TIME / $VM_TIME" | bc)
        VS_PYTHON=$(echo "scale=2; $PYTHON_TIME / $VM_TIME" | bc)
    else
        VM_SPEEDUP="N/A"
        VS_PYTHON="N/A"
    fi

    # Determine color based on VM speedup
    if (( $(echo "$VM_SPEEDUP > 1" | bc -l) )); then
        COLOR=$GREEN
    else
        COLOR=$RED
    fi

    # Print results
    printf "${COLOR}%-20s %-15s %-15s %-15s %-15s %-15s${NC}\n" "$benchmark" "$INTERP_TIME" "$VM_TIME" "$PYTHON_TIME" "${VM_SPEEDUP}x" "${VS_PYTHON}x"

    # Save to file
    echo "$benchmark: Interpreter=${INTERP_TIME}s, VM=${VM_TIME}s, Python=${PYTHON_TIME}s, VM_Speedup=${VM_SPEEDUP}x, vs_Python=${VS_PYTHON}x" >> "$RESULTS_FILE"
done

echo ""
echo "-----------------------------------------------------------------------------------------------------------"
echo -e "${GREEN}Benchmark completed!${NC}"
echo -e "${BLUE}Results saved to: $RESULTS_FILE${NC}"
echo ""
