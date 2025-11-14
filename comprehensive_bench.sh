#!/bin/bash
# Comprehensive benchmark with multiple runs

RUNS=5

benchmark() {
    local name=$1
    local file=$2
    local vm_flag=$3

    echo -n "$name: "

    total=0
    for i in $(seq 1 $RUNS); do
        # Run and extract real time in milliseconds
        result=$( { time ./target/release/topc "$file" --bytecode $vm_flag > /dev/null; } 2>&1 | grep real | awk '{print $2}' | sed 's/[^0-9.]//g' )

        # Convert to milliseconds if needed (assuming format like 0.123s or 0m0.123s)
        if [[ $result == *"m"* ]]; then
            mins=$(echo $result | cut -d'm' -f1)
            secs=$(echo $result | cut -d'm' -f2 | sed 's/s//')
            ms=$(echo "($mins * 60 + $secs) * 1000" | bc)
        else
            ms=$(echo "$result * 1000" | bc 2>/dev/null || echo "0")
        fi

        total=$(echo "$total + $ms" | bc)
    done

    avg=$(echo "$total / $RUNS" | bc)
    echo "${avg}ms (avg of $RUNS runs)"
}

echo "========================================="
echo "NaN Boxing Performance Benchmark"
echo "Running each benchmark $RUNS times..."
echo "========================================="
echo ""

for bench_file in benchmarks/toplang/fibonacci.top benchmarks/toplang/primes.top benchmarks/toplang/array_sum.top; do
    if [ ! -f "$bench_file" ]; then
        continue
    fi

    bench_name=$(basename "$bench_file" .top)
    echo "--- $bench_name ---"
    benchmark "Optimized VM " "$bench_file" ""
    benchmark "NaN Boxing VM" "$bench_file" "--nanbox"
    echo ""
done
