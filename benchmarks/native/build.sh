#!/bin/bash
# Build native benchmarks

set -e

echo "Building native benchmarks..."

for file in benchmarks/native/*.rs; do
    if [ -f "$file" ]; then
        basename=$(basename "$file" .rs)
        echo "  Compiling $basename..."
        rustc -O "$file" -o "benchmarks/native/$basename"
    fi
done

echo "✅ Native benchmarks built successfully!"
