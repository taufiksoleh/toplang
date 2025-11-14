#!/bin/bash

# Native Compilation Benchmark Runner
# Compares: Interpreter vs Bytecode VM vs Native Compiled

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Results directory
RESULTS_DIR="benchmarks/results"
mkdir -p "$RESULTS_DIR"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="${RESULTS_DIR}/native_results_${TIMESTAMP}.txt"

echo -e "${BLUE}============================================================${NC}"
echo -e "${BLUE}    TopLang Native Compilation Benchmark Suite${NC}"
echo -e "${BLUE}============================================================${NC}"
echo ""

# Check if topc exists
if [ ! -f "target/release/topc" ]; then
    echo -e "${YELLOW}Building TopLang compiler...${NC}"
    cargo build --release
    echo -e "${GREEN}Build complete!${NC}"
    echo ""
fi

# Check if C compiler exists
if ! command -v cc &> /dev/null; then
    echo -e "${RED}Error: C compiler (cc/gcc/clang) not found!${NC}"
    echo -e "${YELLOW}Native compilation requires a C compiler.${NC}"
    exit 1
fi

# Benchmarks to run (simple ones first that don't require function calls)
BENCHMARKS=("fibonacci" "primes" "array_sum" "nested_loops" "factorial")

# Initialize results file
{
    echo "TopLang Native Compilation Benchmark Results"
    echo "Generated: $(date)"
    echo "System: $(uname -s) $(uname -m)"
    echo "Compiler: $(cc --version | head -1)"
    echo "============================================================"
    echo ""
} > "$RESULTS_FILE"

# Print header
echo -e "${CYAN}Execution Time Comparison:${NC}"
printf "${BLUE}%-15s %-12s %-12s %-12s %-12s %-12s${NC}\n" \
    "Benchmark" "Interp(s)" "VM(s)" "Native(s)" "VM vs I" "Native vs I"
echo "--------------------------------------------------------------------------------"

# Arrays to store results for summary
declare -a INTERP_TIMES
declare -a VM_TIMES
declare -a NATIVE_TIMES
declare -a COMPILE_TIMES

# Run benchmarks
for benchmark in "${BENCHMARKS[@]}"; do
    echo -e "${YELLOW}Running: $benchmark${NC}"

    TOPLANG_FILE="benchmarks/toplang/${benchmark}.top"
    NATIVE_BIN="benchmarks/toplang/${benchmark}_native"

    # Clean up any previous native binary
    rm -f "$NATIVE_BIN" "${NATIVE_BIN}.c"

    # Measure compilation time for native
    echo -e "  ${CYAN}Compiling to native...${NC}"
    COMPILE_START=$(date +%s.%N)
    ./target/release/topc --compile "$TOPLANG_FILE" -o "$NATIVE_BIN" > /dev/null 2>&1 || {
        echo -e "${RED}  Compilation failed for $benchmark${NC}"
        continue
    }
    COMPILE_END=$(date +%s.%N)
    COMPILE_TIME=$(echo "$COMPILE_END - $COMPILE_START" | bc)

    # Time Interpreter (5 runs, take median)
    echo -e "  ${CYAN}Running interpreter...${NC}"
    INTERP_RUNS=()
    for i in {1..5}; do
        START=$(date +%s.%N)
        ./target/release/topc "$TOPLANG_FILE" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        INTERP_RUNS+=($DURATION)
    done
    # Sort and take median
    IFS=$'\n' INTERP_SORTED=($(sort -n <<<"${INTERP_RUNS[*]}"))
    INTERP_TIME=${INTERP_SORTED[2]}

    # Time Bytecode VM (5 runs, take median)
    echo -e "  ${CYAN}Running bytecode VM...${NC}"
    VM_RUNS=()
    for i in {1..5}; do
        START=$(date +%s.%N)
        ./target/release/topc "$TOPLANG_FILE" --bytecode --nanbox > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        VM_RUNS+=($DURATION)
    done
    IFS=$'\n' VM_SORTED=($(sort -n <<<"${VM_RUNS[*]}"))
    VM_TIME=${VM_SORTED[2]}

    # Time Native (10 runs, take median - native is fast so more runs)
    echo -e "  ${CYAN}Running native binary...${NC}"
    NATIVE_RUNS=()
    for i in {1..10}; do
        START=$(date +%s.%N)
        "$NATIVE_BIN" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        NATIVE_RUNS+=($DURATION)
    done
    IFS=$'\n' NATIVE_SORTED=($(sort -n <<<"${NATIVE_RUNS[*]}"))
    NATIVE_TIME=${NATIVE_SORTED[5]}

    # Store for summary
    INTERP_TIMES+=($INTERP_TIME)
    VM_TIMES+=($VM_TIME)
    NATIVE_TIMES+=($NATIVE_TIME)
    COMPILE_TIMES+=($COMPILE_TIME)

    # Calculate speedups
    if (( $(echo "$VM_TIME > 0" | bc -l) )); then
        VM_SPEEDUP=$(echo "scale=2; $INTERP_TIME / $VM_TIME" | bc)
    else
        VM_SPEEDUP="N/A"
    fi

    if (( $(echo "$NATIVE_TIME > 0" | bc -l) )); then
        NATIVE_SPEEDUP=$(echo "scale=2; $INTERP_TIME / $NATIVE_TIME" | bc)
        NATIVE_VS_VM=$(echo "scale=2; $VM_TIME / $NATIVE_TIME" | bc)
    else
        NATIVE_SPEEDUP="N/A"
        NATIVE_VS_VM="N/A"
    fi

    # Color based on native speedup
    if (( $(echo "$NATIVE_SPEEDUP > 10" | bc -l) )); then
        COLOR=$GREEN
    elif (( $(echo "$NATIVE_SPEEDUP > 5" | bc -l) )); then
        COLOR=$CYAN
    else
        COLOR=$YELLOW
    fi

    # Print results
    printf "${COLOR}%-15s %-12s %-12s %-12s %-12s %-12s${NC}\n" \
        "$benchmark" \
        "$INTERP_TIME" \
        "$VM_TIME" \
        "$NATIVE_TIME" \
        "${VM_SPEEDUP}x" \
        "${NATIVE_SPEEDUP}x"

    # Save to file
    {
        echo "================================================"
        echo "Benchmark: $benchmark"
        echo "  Interpreter:     ${INTERP_TIME}s"
        echo "  Bytecode VM:     ${VM_TIME}s"
        echo "  Native Compiled: ${NATIVE_TIME}s"
        echo "  Compile Time:    ${COMPILE_TIME}s"
        echo "  Speedups:"
        echo "    VM vs Interpreter:     ${VM_SPEEDUP}x"
        echo "    Native vs Interpreter: ${NATIVE_SPEEDUP}x"
        echo "    Native vs VM:          ${NATIVE_VS_VM}x"
        echo ""
    } >> "$RESULTS_FILE"

    # Clean up native binary
    rm -f "$NATIVE_BIN"
done

echo ""
echo "--------------------------------------------------------------------------------"

# Calculate average speedups
TOTAL_BENCHMARKS=${#INTERP_TIMES[@]}

if [ $TOTAL_BENCHMARKS -gt 0 ]; then
    # Average interpreter time
    AVG_INTERP=0
    for time in "${INTERP_TIMES[@]}"; do
        AVG_INTERP=$(echo "$AVG_INTERP + $time" | bc)
    done
    AVG_INTERP=$(echo "scale=3; $AVG_INTERP / $TOTAL_BENCHMARKS" | bc)

    # Average VM time
    AVG_VM=0
    for time in "${VM_TIMES[@]}"; do
        AVG_VM=$(echo "$AVG_VM + $time" | bc)
    done
    AVG_VM=$(echo "scale=3; $AVG_VM / $TOTAL_BENCHMARKS" | bc)

    # Average native time
    AVG_NATIVE=0
    for time in "${NATIVE_TIMES[@]}"; do
        AVG_NATIVE=$(echo "$AVG_NATIVE + $time" | bc)
    done
    AVG_NATIVE=$(echo "scale=3; $AVG_NATIVE / $TOTAL_BENCHMARKS" | bc)

    # Average compile time
    AVG_COMPILE=0
    for time in "${COMPILE_TIMES[@]}"; do
        AVG_COMPILE=$(echo "$AVG_COMPILE + $time" | bc)
    done
    AVG_COMPILE=$(echo "scale=3; $AVG_COMPILE / $TOTAL_BENCHMARKS" | bc)

    # Calculate average speedups
    AVG_VM_SPEEDUP=$(echo "scale=2; $AVG_INTERP / $AVG_VM" | bc)
    AVG_NATIVE_SPEEDUP=$(echo "scale=2; $AVG_INTERP / $AVG_NATIVE" | bc)
    AVG_NATIVE_VS_VM=$(echo "scale=2; $AVG_VM / $AVG_NATIVE" | bc)

    echo -e "${MAGENTA}Summary:${NC}"
    printf "${BLUE}%-15s %-12s %-12s %-12s${NC}\n" "" "Avg Time(s)" "vs Interp" "vs VM"
    echo "--------------------------------------------------------------------------------"
    printf "%-15s ${YELLOW}%-12s${NC} %-12s %-12s\n" "Interpreter" "$AVG_INTERP" "1.00x" "-"
    printf "%-15s ${CYAN}%-12s${NC} ${GREEN}%-12s${NC} %-12s\n" "Bytecode VM" "$AVG_VM" "${AVG_VM_SPEEDUP}x" "1.00x"
    printf "%-15s ${GREEN}%-12s${NC} ${GREEN}%-12s${NC} ${GREEN}%-12s${NC}\n" "Native" "$AVG_NATIVE" "${AVG_NATIVE_SPEEDUP}x" "${AVG_NATIVE_VS_VM}x"
    printf "%-15s %-12s %-12s %-12s\n" "Compile Time" "$AVG_COMPILE" "-" "-"

    # Save summary to file
    {
        echo "================================================"
        echo "SUMMARY"
        echo "================================================"
        echo "Average Execution Times:"
        echo "  Interpreter:     ${AVG_INTERP}s"
        echo "  Bytecode VM:     ${AVG_VM}s"
        echo "  Native Compiled: ${AVG_NATIVE}s"
        echo "  Compile Time:    ${AVG_COMPILE}s"
        echo ""
        echo "Average Speedups:"
        echo "  VM vs Interpreter:     ${AVG_VM_SPEEDUP}x faster"
        echo "  Native vs Interpreter: ${AVG_NATIVE_SPEEDUP}x faster"
        echo "  Native vs VM:          ${AVG_NATIVE_VS_VM}x faster"
        echo ""
        echo "Key Insights:"
        if (( $(echo "$AVG_NATIVE_SPEEDUP > 10" | bc -l) )); then
            echo "  🚀 Native compilation provides EXCELLENT performance!"
            echo "     More than 10x faster than interpretation."
        elif (( $(echo "$AVG_NATIVE_SPEEDUP > 5" | bc -l) )); then
            echo "  ✨ Native compilation provides GREAT performance!"
            echo "     5-10x faster than interpretation."
        else
            echo "  ✓ Native compilation provides good performance."
        fi

        if (( $(echo "$AVG_COMPILE < 0.1" | bc -l) )); then
            echo "  ⚡ Compilation is VERY fast (< 0.1s average)"
        elif (( $(echo "$AVG_COMPILE < 0.5" | bc -l) )); then
            echo "  ⚡ Compilation is fast (< 0.5s average)"
        fi
    } >> "$RESULTS_FILE"
fi

echo ""
echo "--------------------------------------------------------------------------------"
echo -e "${GREEN}✓ Benchmark completed!${NC}"
echo -e "${BLUE}Results saved to: $RESULTS_FILE${NC}"
echo ""

# Display key insight
if (( $(echo "$AVG_NATIVE_SPEEDUP > 10" | bc -l) )); then
    echo -e "${GREEN}🚀 Native compilation is ${AVG_NATIVE_SPEEDUP}x faster than interpretation!${NC}"
    echo -e "${GREEN}   This is a MASSIVE performance improvement!${NC}"
elif (( $(echo "$AVG_NATIVE_SPEEDUP > 5" | bc -l) )); then
    echo -e "${CYAN}✨ Native compilation is ${AVG_NATIVE_SPEEDUP}x faster than interpretation!${NC}"
    echo -e "${CYAN}   This is a significant performance boost!${NC}"
fi

echo ""
