#!/bin/bash

# Benchmark runner script for TopLang vs Python
# This script runs benchmarks and compares performance

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Results directory
RESULTS_DIR="benchmarks/results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="${RESULTS_DIR}/results_${TIMESTAMP}.txt"

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}TopLang vs Python Benchmark Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if topc exists
if [ ! -f "target/release/topc" ]; then
    echo -e "${YELLOW}Building TopLang compiler...${NC}"
    cargo build --release
    echo -e "${GREEN}Build complete!${NC}"
    echo ""
fi

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Python3 is not installed!${NC}"
    exit 1
fi

# Benchmarks to run
BENCHMARKS=("fibonacci" "primes" "array_sum" "nested_loops" "factorial")

# Initialize results file
echo "TopLang vs Python Benchmark Results" > "$RESULTS_FILE"
echo "Generated: $(date)" >> "$RESULTS_FILE"
echo "========================================" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Print header
printf "${BLUE}%-20s %-15s %-15s %-15s${NC}\n" "Benchmark" "TopLang (s)" "Python (s)" "Speedup"
echo "------------------------------------------------------------------------"

# Run benchmarks
for benchmark in "${BENCHMARKS[@]}"; do
    echo -e "${YELLOW}Running: $benchmark${NC}"

    # Run TopLang benchmark
    TOPLANG_FILE="benchmarks/toplang/${benchmark}.top"
    PYTHON_FILE="benchmarks/python/${benchmark}.py"

    # Time TopLang execution (3 runs, take average)
    TOPLANG_TOTAL=0
    for i in {1..3}; do
        START=$(date +%s.%N)
        ./target/release/topc "$TOPLANG_FILE" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        TOPLANG_TOTAL=$(echo "$TOPLANG_TOTAL + $DURATION" | bc)
    done
    TOPLANG_TIME=$(echo "scale=3; $TOPLANG_TOTAL / 3" | bc)

    # Time Python execution (3 runs, take average)
    PYTHON_TOTAL=0
    for i in {1..3}; do
        START=$(date +%s.%N)
        python3 "$PYTHON_FILE" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        PYTHON_TOTAL=$(echo "$PYTHON_TOTAL + $DURATION" | bc)
    done
    PYTHON_TIME=$(echo "scale=3; $PYTHON_TOTAL / 3" | bc)

    # Calculate speedup
    if (( $(echo "$TOPLANG_TIME > 0" | bc -l) )); then
        SPEEDUP=$(echo "scale=2; $PYTHON_TIME / $TOPLANG_TIME" | bc)
    else
        SPEEDUP="N/A"
    fi

    # Determine color based on speedup
    if (( $(echo "$SPEEDUP > 1" | bc -l) )); then
        COLOR=$GREEN
    else
        COLOR=$RED
    fi

    # Print results
    printf "${COLOR}%-20s %-15s %-15s %-15s${NC}\n" "$benchmark" "$TOPLANG_TIME" "$PYTHON_TIME" "${SPEEDUP}x"

    # Save to file
    echo "$benchmark: TopLang=${TOPLANG_TIME}s, Python=${PYTHON_TIME}s, Speedup=${SPEEDUP}x" >> "$RESULTS_FILE"
done

echo ""
echo "------------------------------------------------------------------------"
echo -e "${GREEN}Benchmark completed!${NC}"
echo -e "${BLUE}Results saved to: $RESULTS_FILE${NC}"
echo ""

# Generate summary
echo "" >> "$RESULTS_FILE"
echo "Summary:" >> "$RESULTS_FILE"
echo "--------" >> "$RESULTS_FILE"
echo "Total benchmarks: ${#BENCHMARKS[@]}" >> "$RESULTS_FILE"
