#!/bin/bash

echo "=== Fibonacci Benchmark ==="
echo -n "Regular VM:   "
{ time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode >/dev/null; } 2>&1 | grep real

echo -n "NaN Box VM:   "
{ time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox >/dev/null; } 2>&1 | grep real

echo ""
echo "=== Primes Benchmark ==="
echo -n "Regular VM:   "
{ time ./target/release/topc benchmarks/toplang/primes.top --bytecode >/dev/null; } 2>&1 | grep real

echo -n "NaN Box VM:   "
{ time ./target/release/topc benchmarks/toplang/primes.top --bytecode --nanbox >/dev/null; } 2>&1 | grep real

echo ""
echo "=== Array Sum Benchmark ==="
echo -n "Regular VM:   "
{ time ./target/release/topc benchmarks/toplang/array_sum.top --bytecode >/dev/null; } 2>&1 | grep real

echo -n "NaN Box VM:   "
{ time ./target/release/topc benchmarks/toplang/array_sum.top --bytecode --nanbox >/dev/null; } 2>&1 | grep real
