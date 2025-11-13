#!/bin/bash
# Test runner for TopLang example files

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL=0
PASSED=0
FAILED=0
SKIPPED=0

# Build the project first
echo -e "${BLUE}Building toplang...${NC}"
if cargo build --release 2>&1 | grep -q "error"; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
echo -e "${GREEN}Build successful!${NC}"
echo ""

TOPC="./target/release/topc"

# Check if examples directory exists
if [ ! -d "examples" ]; then
    echo -e "${RED}Error: examples directory not found${NC}"
    exit 1
fi

# Find all .top files in examples directory
echo -e "${BLUE}Running tests...${NC}"
echo "================================"
echo ""

for file in examples/*.top; do
    # Skip if no files found
    if [ ! -f "$file" ]; then
        continue
    fi

    TOTAL=$((TOTAL + 1))
    filename=$(basename "$file")

    # Skip files that require user input
    if [[ "$filename" == "input.top" ]] || [[ "$filename" == "arrays_with_input.top" ]]; then
        echo -e "${YELLOW}⊘ SKIP${NC}: $filename (requires user input)"
        SKIPPED=$((SKIPPED + 1))
        continue
    fi

    # Run the file and capture output and exit code
    if timeout 5s "$TOPC" "$file" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}: $filename"
        PASSED=$((PASSED + 1))
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo -e "${RED}✗ FAIL${NC}: $filename (timeout after 5s)"
        else
            echo -e "${RED}✗ FAIL${NC}: $filename (exit code: $exit_code)"
        fi
        FAILED=$((FAILED + 1))

        # Show error output for failed tests
        echo -e "${YELLOW}  Error output:${NC}"
        "$TOPC" "$file" 2>&1 | head -5 | sed 's/^/  /'
        echo ""
    fi
done

# Summary
echo ""
echo "================================"
echo -e "${BLUE}Test Summary${NC}"
echo "================================"
echo "Total:   $TOTAL"
echo -e "${GREEN}Passed:  $PASSED${NC}"
echo -e "${RED}Failed:  $FAILED${NC}"
echo -e "${YELLOW}Skipped: $SKIPPED${NC}"
echo ""

# Exit with error if any tests failed
if [ $FAILED -gt 0 ]; then
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
