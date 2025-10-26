#!/bin/bash
set -e

echo "🦉 OWLSOL Benchmark Suite"
echo "========================="
echo ""

# Create test data if it doesn't exist
mkdir -p benchmark/data

if [ ! -f "benchmark/data/test_text.txt" ]; then
    echo "Generating test data..."
    
    # Text data
    for i in {1..1000}; do
        echo "This is test line $i with some repeated content" >> benchmark/data/test_text.txt
    done
    
    # JSON data
    echo '[' > benchmark/data/test_json.json
    for i in {1..100}; do
        echo "{\"id\": $i, \"name\": \"Item $i\", \"value\": $((i * 100))}" >> benchmark/data/test_json.json
        if [ $i -lt 100 ]; then echo "," >> benchmark/data/test_json.json; fi
    done
    echo ']' >> benchmark/data/test_json.json
    
    # Binary data
    dd if=/dev/urandom of=benchmark/data/test_binary.bin bs=1024 count=100 2>/dev/null
    
    echo "✓ Test data generated"
fi

echo ""

# Build in release mode
echo "Building in release mode..."
cargo build --release

echo "✓ Build complete"
echo ""

# Run benchmarks
echo "Running benchmarks..."
echo ""

FILES=("test_text.txt" "test_json.json" "test_binary.bin")

for file in "${FILES[@]}"; do
    echo "Benchmarking $file..."
    ./target/release/owlsol benchmark \
        -i "benchmark/data/$file" \
        -n 100 \
        -o "benchmark/results/${file%.* }_report.txt"
    echo ""
done

echo "✅ Benchmarks complete!"
echo ""
echo "Results saved in: benchmark/results/"
