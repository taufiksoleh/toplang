#!/bin/bash

# Complete Benchmark Suite
# Compares: Interpreter vs Bytecode VM vs Native vs Python

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
RESULTS_FILE="${RESULTS_DIR}/complete_results_${TIMESTAMP}.txt"

echo -e "${MAGENTA}============================================================${NC}"
echo -e "${MAGENTA}    TopLang Complete Performance Benchmark Suite${NC}"
echo -e "${MAGENTA}    Interpreter | VM | Native | Python${NC}"
echo -e "${MAGENTA}============================================================${NC}"
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
    echo -e "${RED}Warning: C compiler not found, skipping native compilation${NC}"
    HAS_CC=false
else
    HAS_CC=true
fi

# Check if Python exists
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Warning: Python3 not found, skipping Python benchmarks${NC}"
    HAS_PYTHON=false
else
    HAS_PYTHON=true
fi

# Benchmarks to run
BENCHMARKS=("fibonacci" "primes" "array_sum" "nested_loops" "factorial")

# Initialize results file
{
    echo "TopLang Complete Performance Benchmark"
    echo "Generated: $(date)"
    echo "System: $(uname -s) $(uname -m) $(uname -r)"
    if [ "$HAS_CC" = true ]; then
        echo "C Compiler: $(cc --version | head -1)"
    fi
    if [ "$HAS_PYTHON" = true ]; then
        echo "Python: $(python3 --version)"
    fi
    echo "============================================================"
    echo ""
} > "$RESULTS_FILE"

# Print header
if [ "$HAS_CC" = true ] && [ "$HAS_PYTHON" = true ]; then
    printf "${BLUE}%-15s %-11s %-11s %-11s %-11s %-11s %-11s${NC}\n" \
        "Benchmark" "Interp(s)" "VM(s)" "Native(s)" "Python(s)" "Native/I" "Native/Py"
    echo "--------------------------------------------------------------------------------------------"
elif [ "$HAS_CC" = true ]; then
    printf "${BLUE}%-15s %-11s %-11s %-11s %-11s${NC}\n" \
        "Benchmark" "Interp(s)" "VM(s)" "Native(s)" "Native/I"
    echo "------------------------------------------------------------------------"
else
    printf "${BLUE}%-15s %-11s %-11s %-11s${NC}\n" \
        "Benchmark" "Interp(s)" "VM(s)" "Python(s)"
    echo "------------------------------------------------------------"
fi

# Arrays to store results
declare -a INTERP_TIMES
declare -a VM_TIMES
declare -a NATIVE_TIMES
declare -a PYTHON_TIMES

# Run benchmarks
for benchmark in "${BENCHMARKS[@]}"; do
    echo -e "${YELLOW}▸ Running: $benchmark${NC}"

    TOPLANG_FILE="benchmarks/toplang/${benchmark}.top"
    PYTHON_FILE="benchmarks/python/${benchmark}.py"
    NATIVE_BIN="benchmarks/toplang/${benchmark}_native"

    # Clean up any previous native binary
    rm -f "$NATIVE_BIN" "${NATIVE_BIN}.c"

    # Time Interpreter (5 runs, median)
    INTERP_RUNS=()
    for i in {1..5}; do
        START=$(date +%s.%N)
        timeout 30 ./target/release/topc "$TOPLANG_FILE" > /dev/null 2>&1 || continue
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        INTERP_RUNS+=($DURATION)
    done
    IFS=$'\n' INTERP_SORTED=($(sort -n <<<"${INTERP_RUNS[*]}"))
    INTERP_TIME=${INTERP_SORTED[2]:-0}

    # Time Bytecode VM (5 runs, median)
    VM_RUNS=()
    for i in {1..5}; do
        START=$(date +%s.%N)
        timeout 30 ./target/release/topc "$TOPLANG_FILE" --bytecode --nanbox > /dev/null 2>&1 || continue
        END=$(date +%s.%N)
        DURATION=$(echo "$END - $START" | bc)
        VM_RUNS+=($DURATION)
    done
    IFS=$'\n' VM_SORTED=($(sort -n <<<"${VM_RUNS[*]}"))
    VM_TIME=${VM_SORTED[2]:-0}

    # Time Native (if available)
    if [ "$HAS_CC" = true ]; then
        timeout 10 ./target/release/topc --compile "$TOPLANG_FILE" -o "$NATIVE_BIN" > /dev/null 2>&1
        if [ -f "$NATIVE_BIN" ]; then
            NATIVE_RUNS=()
            for i in {1..10}; do
                START=$(date +%s.%N)
                timeout 30 "$NATIVE_BIN" > /dev/null 2>&1 || continue
                END=$(date +%s.%N)
                DURATION=$(echo "$END - $START" | bc)
                NATIVE_RUNS+=($DURATION)
            done
            IFS=$'\n' NATIVE_SORTED=($(sort -n <<<"${NATIVE_RUNS[*]}"))
            NATIVE_TIME=${NATIVE_SORTED[5]:-0}
        else
            NATIVE_TIME=0
        fi
    else
        NATIVE_TIME=0
    fi

    # Time Python (if available)
    if [ "$HAS_PYTHON" = true ] && [ -f "$PYTHON_FILE" ]; then
        PYTHON_RUNS=()
        for i in {1..5}; do
            START=$(date +%s.%N)
            timeout 30 python3 "$PYTHON_FILE" > /dev/null 2>&1 || continue
            END=$(date +%s.%N)
            DURATION=$(echo "$END - $START" | bc)
            PYTHON_RUNS+=($DURATION)
        done
        IFS=$'\n' PYTHON_SORTED=($(sort -n <<<"${PYTHON_RUNS[*]}"))
        PYTHON_TIME=${PYTHON_SORTED[2]:-0}
    else
        PYTHON_TIME=0
    fi

    # Store results
    INTERP_TIMES+=($INTERP_TIME)
    VM_TIMES+=($VM_TIME)
    NATIVE_TIMES+=($NATIVE_TIME)
    PYTHON_TIMES+=($PYTHON_TIME)

    # Calculate speedups
    if (( $(echo "$NATIVE_TIME > 0" | bc -l) )); then
        NATIVE_SPEEDUP=$(echo "scale=1; $INTERP_TIME / $NATIVE_TIME" | bc)
        if (( $(echo "$PYTHON_TIME > 0" | bc -l) )); then
            NATIVE_VS_PY=$(echo "scale=1; $PYTHON_TIME / $NATIVE_TIME" | bc)
        else
            NATIVE_VS_PY="N/A"
        fi
    else
        NATIVE_SPEEDUP="N/A"
        NATIVE_VS_PY="N/A"
    fi

    # Color based on native speedup
    if (( $(echo "$NATIVE_SPEEDUP > 15" | bc -l) )); then
        COLOR=$GREEN
    elif (( $(echo "$NATIVE_SPEEDUP > 8" | bc -l) )); then
        COLOR=$CYAN
    else
        COLOR=$YELLOW
    fi

    # Print results based on what's available
    if [ "$HAS_CC" = true ] && [ "$HAS_PYTHON" = true ]; then
        printf "${COLOR}%-15s %-11s %-11s %-11s %-11s %-11s %-11s${NC}\n" \
            "$benchmark" "$INTERP_TIME" "$VM_TIME" "$NATIVE_TIME" "$PYTHON_TIME" \
            "${NATIVE_SPEEDUP}x" "${NATIVE_VS_PY}x"
    elif [ "$HAS_CC" = true ]; then
        printf "${COLOR}%-15s %-11s %-11s %-11s %-11s${NC}\n" \
            "$benchmark" "$INTERP_TIME" "$VM_TIME" "$NATIVE_TIME" "${NATIVE_SPEEDUP}x"
    else
        printf "${COLOR}%-15s %-11s %-11s %-11s${NC}\n" \
            "$benchmark" "$INTERP_TIME" "$VM_TIME" "$PYTHON_TIME"
    fi

    # Save to file
    {
        echo "Benchmark: $benchmark"
        echo "  Interpreter:     ${INTERP_TIME}s"
        echo "  Bytecode VM:     ${VM_TIME}s"
        if [ "$HAS_CC" = true ]; then
            echo "  Native Compiled: ${NATIVE_TIME}s"
            echo "  Native Speedup:  ${NATIVE_SPEEDUP}x vs Interpreter"
        fi
        if [ "$HAS_PYTHON" = true ]; then
            echo "  Python:          ${PYTHON_TIME}s"
            if [ "$HAS_CC" = true ]; then
                echo "  Native vs Python: ${NATIVE_VS_PY}x"
            fi
        fi
        echo ""
    } >> "$RESULTS_FILE"

    # Clean up
    rm -f "$NATIVE_BIN"
done

echo ""
if [ "$HAS_CC" = true ] && [ "$HAS_PYTHON" = true ]; then
    echo "--------------------------------------------------------------------------------------------"
elif [ "$HAS_CC" = true ]; then
    echo "------------------------------------------------------------------------"
else
    echo "------------------------------------------------------------"
fi

echo -e "${GREEN}✓ Complete benchmark suite finished!${NC}"
echo -e "${BLUE}Results saved to: $RESULTS_FILE${NC}"
echo ""
