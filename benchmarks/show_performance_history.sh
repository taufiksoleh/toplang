#!/bin/bash

# Performance History Analyzer
# Displays historical benchmark data from CSV file
#
# Shows:
# - Total benchmark runs recorded
# - Recent performance (last 10 runs) with comparison
# - Performance improvement/regression over time
# - Best native performance achieved
#
# Expected Output:
# - Native performance should be 115-135x faster than interpreter
# - Compilation time should be 200-300ms
# - Performance should be stable across runs (±5%)
#
# Usage:
#   ./benchmarks/show_performance_history.sh
#
# CSV Format:
#   Date,Time,Commit,Branch,Benchmark,Interpreter_ms,VM_ms,Native_ms,
#   Compile_ms,Native_vs_Interp,Native_vs_VM,System,Compiler

CSV_FILE="benchmarks/results/performance_history.csv"

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

if [ ! -f "$CSV_FILE" ]; then
    echo -e "${YELLOW}No historical data found.${NC}"
    echo "Run benchmarks first: ./benchmarks/run_native_benchmarks.sh"
    exit 1
fi

echo -e "${BLUE}============================================================${NC}"
echo -e "${BLUE}         TopLang Performance History${NC}"
echo -e "${BLUE}============================================================${NC}"
echo ""

# Count total runs
TOTAL_RUNS=$(grep -c "AVERAGE" "$CSV_FILE" || echo "0")
echo -e "${CYAN}Total benchmark runs recorded: ${TOTAL_RUNS}${NC}"
echo ""

# Show recent averages (last 10 runs)
echo -e "${MAGENTA}Recent Performance (Last 10 Runs):${NC}"
echo ""
printf "${BLUE}%-12s %-8s %-10s %-10s %-10s %-10s${NC}\n" \
    "Date" "Commit" "Interp(ms)" "VM(ms)" "Native(ms)" "Native/I"
echo "------------------------------------------------------------------------"

grep "AVERAGE" "$CSV_FILE" | tail -10 | while IFS=, read -r date time commit branch bench interp vm native compile speedup_i speedup_vm system compiler; do
    printf "%-12s %-8s %-10s %-10s %-10s ${GREEN}%-10s${NC}\n" \
        "$date" "$commit" "$interp" "$vm" "$native" "${speedup_i}x"
done

echo ""

# Calculate improvement over time
FIRST_NATIVE=$(grep "AVERAGE" "$CSV_FILE" | head -1 | cut -d, -f8)
LATEST_NATIVE=$(grep "AVERAGE" "$CSV_FILE" | tail -1 | cut -d, -f8)

if [ -n "$FIRST_NATIVE" ] && [ -n "$LATEST_NATIVE" ] && [ "$TOTAL_RUNS" -gt 1 ]; then
    IMPROVEMENT=$(echo "scale=2; ($FIRST_NATIVE - $LATEST_NATIVE) / $FIRST_NATIVE * 100" | bc)
    if (( $(echo "$IMPROVEMENT > 0" | bc -l) )); then
        echo -e "${GREEN}📈 Native performance improved by ${IMPROVEMENT}% since first run${NC}"
    elif (( $(echo "$IMPROVEMENT < 0" | bc -l) )); then
        IMPROVEMENT_ABS=$(echo "$IMPROVEMENT * -1" | bc)
        echo -e "${YELLOW}📉 Native performance regressed by ${IMPROVEMENT_ABS}% since first run${NC}"
    else
        echo -e "${CYAN}➡️  Native performance stable since first run${NC}"
    fi
    echo ""
fi

# Show best native performance
echo -e "${MAGENTA}Best Native Performance:${NC}"
BEST_LINE=$(grep "AVERAGE" "$CSV_FILE" | sort -t, -k8 -n | head -1)
if [ -n "$BEST_LINE" ]; then
    BEST_DATE=$(echo "$BEST_LINE" | cut -d, -f1)
    BEST_COMMIT=$(echo "$BEST_LINE" | cut -d, -f3)
    BEST_NATIVE=$(echo "$BEST_LINE" | cut -d, -f8)
    BEST_SPEEDUP=$(echo "$BEST_LINE" | cut -d, -f10)
    echo -e "  Date: ${CYAN}$BEST_DATE${NC}"
    echo -e "  Commit: ${CYAN}$BEST_COMMIT${NC}"
    echo -e "  Native time: ${GREEN}${BEST_NATIVE}ms${NC}"
    echo -e "  Speedup: ${GREEN}${BEST_SPEEDUP}x vs Interpreter${NC}"
fi

echo ""
echo -e "${BLUE}============================================================${NC}"
echo -e "${CYAN}View full data: cat $CSV_FILE${NC}"
echo -e "${CYAN}View averages: cat $CSV_FILE | grep AVERAGE${NC}"
echo -e "${CYAN}Export for Excel/Google Sheets: $CSV_FILE${NC}"
echo ""
