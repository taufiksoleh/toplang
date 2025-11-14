#!/bin/bash

# Native Compilation Benchmark Runner with Historical Tracking
# Compares: Interpreter vs Bytecode VM vs Native Compiled
# Saves results in CSV format for historical analysis

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
DATE=$(date +%Y-%m-%d)
TIME=$(date +%H:%M:%S)
RESULTS_FILE="${RESULTS_DIR}/native_results_${TIMESTAMP}.txt"
CSV_FILE="${RESULTS_DIR}/performance_history.csv"

echo -e "${BLUE}============================================================${NC}"
echo -e "${BLUE}    TopLang Native Compilation Benchmark Suite${NC}"
echo -e "${BLUE}    With Historical Performance Tracking${NC}"
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

# Get system info
SYSTEM_INFO="$(uname -s) $(uname -m)"
COMPILER_INFO="$(cc --version | head -1)"
GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

# Create CSV header if file doesn't exist
if [ ! -f "$CSV_FILE" ]; then
    echo "Date,Time,Commit,Branch,Benchmark,Interpreter_ms,VM_ms,Native_ms,Compile_ms,Native_vs_Interp,Native_vs_VM,System,Compiler" > "$CSV_FILE"
    echo -e "${CYAN}Created new performance history file: $CSV_FILE${NC}"
fi

# Benchmarks to run
BENCHMARKS=("fibonacci" "primes" "array_sum" "nested_loops" "factorial")

# Initialize results file
{
    echo "TopLang Native Compilation Benchmark Results"
    echo "Generated: $(date)"
    echo "Git Commit: $GIT_COMMIT"
    echo "Git Branch: $GIT_BRANCH"
    echo "System: $SYSTEM_INFO"
    echo "Compiler: $COMPILER_INFO"
    echo "============================================================"
    echo ""
} > "$RESULTS_FILE"

# Print header
printf "${CYAN}Historical Performance Tracking Enabled${NC}\n"
printf "${CYAN}Results will be saved to: $CSV_FILE${NC}\n"
echo ""
printf "${BLUE}%-15s %-12s %-12s %-12s %-12s %-12s${NC}\n" \
    "Benchmark" "Interp(ms)" "VM(ms)" "Native(ms)" "Native/I" "Native/VM"
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
    COMPILE_START=$(date +%s.%N)
    ./target/release/topc --compile "$TOPLANG_FILE" -o "$NATIVE_BIN" > /dev/null 2>&1 || {
        echo -e "${RED}  Compilation failed for $benchmark${NC}"
        continue
    }
    COMPILE_END=$(date +%s.%N)
    COMPILE_TIME=$(echo "$COMPILE_END - $COMPILE_START" | bc)
    COMPILE_MS=$(echo "$COMPILE_TIME * 1000" | bc | cut -d. -f1)

    # Time Interpreter (5 runs, take median)
    INTERP_RUNS=()
    for i in {1..5}; do
        START=$(date +%s.%N)
        ./target/release/topc "$TOPLANG_FILE" > /dev/null 2>&1
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        INTERP_RUNS+=($DURATION)
    done
    IFS=$'\n' INTERP_SORTED=($(sort -n <<<"${INTERP_RUNS[*]}"))
    INTERP_TIME=${INTERP_SORTED[2]}
    INTERP_MS=$(echo "$INTERP_TIME * 1000" | bc | cut -d. -f1)

    # Time Bytecode VM (5 runs, take median)
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
    VM_MS=$(echo "$VM_TIME * 1000" | bc | cut -d. -f1)

    # Time Native (10 runs, take median)
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
    NATIVE_MS=$(echo "$NATIVE_TIME * 1000" | bc | cut -d. -f1)

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
    if (( $(echo "$NATIVE_SPEEDUP > 100" | bc -l) )); then
        COLOR=$GREEN
    elif (( $(echo "$NATIVE_SPEEDUP > 50" | bc -l) )); then
        COLOR=$CYAN
    else
        COLOR=$YELLOW
    fi

    # Print results
    printf "${COLOR}%-15s %-12s %-12s %-12s %-12s %-12s${NC}\n" \
        "$benchmark" \
        "$INTERP_MS" \
        "$VM_MS" \
        "$NATIVE_MS" \
        "${NATIVE_SPEEDUP}x" \
        "${NATIVE_VS_VM}x"

    # Save to CSV (historical data)
    echo "$DATE,$TIME,$GIT_COMMIT,$GIT_BRANCH,$benchmark,$INTERP_MS,$VM_MS,$NATIVE_MS,$COMPILE_MS,$NATIVE_SPEEDUP,$NATIVE_VS_VM,$SYSTEM_INFO,$COMPILER_INFO" >> "$CSV_FILE"

    # Save to text file
    {
        echo "================================================"
        echo "Benchmark: $benchmark"
        echo "  Interpreter:     ${INTERP_TIME}s (${INTERP_MS}ms)"
        echo "  Bytecode VM:     ${VM_TIME}s (${VM_MS}ms)"
        echo "  Native Compiled: ${NATIVE_TIME}s (${NATIVE_MS}ms)"
        echo "  Compile Time:    ${COMPILE_TIME}s (${COMPILE_MS}ms)"
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
    # Average times
    AVG_INTERP=0
    for time in "${INTERP_TIMES[@]}"; do
        AVG_INTERP=$(echo "$AVG_INTERP + $time" | bc)
    done
    AVG_INTERP=$(echo "scale=3; $AVG_INTERP / $TOTAL_BENCHMARKS" | bc)
    AVG_INTERP_MS=$(echo "$AVG_INTERP * 1000" | bc | cut -d. -f1)

    AVG_VM=0
    for time in "${VM_TIMES[@]}"; do
        AVG_VM=$(echo "$AVG_VM + $time" | bc)
    done
    AVG_VM=$(echo "scale=3; $AVG_VM / $TOTAL_BENCHMARKS" | bc)
    AVG_VM_MS=$(echo "$AVG_VM * 1000" | bc | cut -d. -f1)

    AVG_NATIVE=0
    for time in "${NATIVE_TIMES[@]}"; do
        AVG_NATIVE=$(echo "$AVG_NATIVE + $time" | bc)
    done
    AVG_NATIVE=$(echo "scale=3; $AVG_NATIVE / $TOTAL_BENCHMARKS" | bc)
    AVG_NATIVE_MS=$(echo "$AVG_NATIVE * 1000" | bc | cut -d. -f1)

    AVG_COMPILE=0
    for time in "${COMPILE_TIMES[@]}"; do
        AVG_COMPILE=$(echo "$AVG_COMPILE + $time" | bc)
    done
    AVG_COMPILE=$(echo "scale=3; $AVG_COMPILE / $TOTAL_BENCHMARKS" | bc)
    AVG_COMPILE_MS=$(echo "$AVG_COMPILE * 1000" | bc | cut -d. -f1)

    # Calculate average speedups
    AVG_VM_SPEEDUP=$(echo "scale=2; $AVG_INTERP / $AVG_VM" | bc)
    AVG_NATIVE_SPEEDUP=$(echo "scale=2; $AVG_INTERP / $AVG_NATIVE" | bc)
    AVG_NATIVE_VS_VM=$(echo "scale=2; $AVG_VM / $AVG_NATIVE" | bc)

    echo -e "${MAGENTA}Summary:${NC}"
    printf "${BLUE}%-15s %-12s %-12s %-12s${NC}\n" "" "Avg Time(ms)" "vs Interp" "vs VM"
    echo "--------------------------------------------------------------------------------"
    printf "%-15s ${YELLOW}%-12s${NC} %-12s %-12s\n" "Interpreter" "$AVG_INTERP_MS" "1.00x" "-"
    printf "%-15s ${CYAN}%-12s${NC} ${GREEN}%-12s${NC} %-12s\n" "Bytecode VM" "$AVG_VM_MS" "${AVG_VM_SPEEDUP}x" "1.00x"
    printf "%-15s ${GREEN}%-12s${NC} ${GREEN}%-12s${NC} ${GREEN}%-12s${NC}\n" "Native" "$AVG_NATIVE_MS" "${AVG_NATIVE_SPEEDUP}x" "${AVG_NATIVE_VS_VM}x"
    printf "%-15s %-12s %-12s %-12s\n" "Compile Time" "$AVG_COMPILE_MS" "-" "-"

    # Save summary to CSV
    echo "$DATE,$TIME,$GIT_COMMIT,$GIT_BRANCH,AVERAGE,$AVG_INTERP_MS,$AVG_VM_MS,$AVG_NATIVE_MS,$AVG_COMPILE_MS,$AVG_NATIVE_SPEEDUP,$AVG_NATIVE_VS_VM,$SYSTEM_INFO,$COMPILER_INFO" >> "$CSV_FILE"

    # Save summary to file
    {
        echo "================================================"
        echo "SUMMARY"
        echo "================================================"
        echo "Average Execution Times:"
        echo "  Interpreter:     ${AVG_INTERP}s (${AVG_INTERP_MS}ms)"
        echo "  Bytecode VM:     ${AVG_VM}s (${AVG_VM_MS}ms)"
        echo "  Native Compiled: ${AVG_NATIVE}s (${AVG_NATIVE_MS}ms)"
        echo "  Compile Time:    ${AVG_COMPILE}s (${AVG_COMPILE_MS}ms)"
        echo ""
        echo "Average Speedups:"
        echo "  VM vs Interpreter:     ${AVG_VM_SPEEDUP}x faster"
        echo "  Native vs Interpreter: ${AVG_NATIVE_SPEEDUP}x faster"
        echo "  Native vs VM:          ${AVG_NATIVE_VS_VM}x faster"
    } >> "$RESULTS_FILE"
fi

echo ""
echo "--------------------------------------------------------------------------------"
echo -e "${GREEN}✓ Benchmark completed!${NC}"
echo -e "${BLUE}Results saved to:${NC}"
echo -e "  ${CYAN}Text: $RESULTS_FILE${NC}"
echo -e "  ${CYAN}CSV:  $CSV_FILE${NC}"
echo ""
echo -e "${MAGENTA}📊 Historical data now available in CSV format${NC}"
echo -e "${MAGENTA}   View trends: cat $CSV_FILE | grep AVERAGE${NC}"
echo ""

if (( $(echo "$AVG_NATIVE_SPEEDUP > 100" | bc -l) )); then
    echo -e "${GREEN}🚀 Native compilation is ${AVG_NATIVE_SPEEDUP}x faster!${NC}"
fi
