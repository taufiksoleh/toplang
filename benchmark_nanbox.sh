#!/bin/bash
# Benchmark NaN Boxing VM vs Regular Optimized VM

echo "========================================="
echo "NaN Boxing Performance Comparison"
echo "========================================="
echo ""

BENCHMARKS=(
    "benchmarks/toplang/fibonacci.top"
    "benchmarks/toplang/primes.top"
    "benchmarks/toplang/array_sum.top"
)

for bench in "${BENCHMARKS[@]}"; do
    if [ ! -f "$bench" ]; then
        continue
    fi

    name=$(basename "$bench" .top)
    echo "Benchmark: $name"
    echo "-----------------------------------------"

    # Regular Optimized VM
    echo -n "Optimized VM:  "
    /usr/bin/time -f "%E elapsed, %Mkb max memory" ./target/release/topc "$bench" --bytecode > /dev/null 2>&1

    # NaN Boxing VM
    echo -n "NaN Boxing VM: "
    /usr/bin/time -f "%E elapsed, %Mkb max memory" ./target/release/topc "$bench" --bytecode --nanbox > /dev/null 2>&1

    echo ""
done

echo "========================================="
echo "Testing correctness with fibonacci..."
echo "========================================="
echo ""

echo "Regular VM output:"
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode

echo ""
echo "NaN Boxing VM output:"
./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox

echo ""
echo "Done!"
