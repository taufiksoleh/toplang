#!/bin/bash
# Simple benchmark runner

echo "Benchmarking fibonacci (1M iterations)..."
echo ""

echo "Regular Optimized VM:"
time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode > /dev/null
echo ""

echo "NaN Boxing VM:"
time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode --nanbox > /dev/null
echo ""

echo "Benchmarking primes..."
echo ""

echo "Regular Optimized VM:"
time ./target/release/topc benchmarks/toplang/primes.top --bytecode > /dev/null
echo ""

echo "NaN Boxing VM:"
time ./target/release/topc benchmarks/toplang/primes.top --bytecode --nanbox > /dev/null
