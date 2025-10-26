#!/bin/bash
set -e

echo "🦉 OWLSOL Test Suite"
echo "===================="
echo ""

# Run unit tests
echo "Running unit tests..."
cargo test --lib --all

echo ""
echo "✓ Unit tests passed"
echo ""

# Run integration tests
echo "Running integration tests..."
cargo test --test '*' --all

echo ""
echo "✓ Integration tests passed"
echo ""

# Run with coverage (if tarpaulin is installed)
if command -v cargo-tarpaulin &> /dev/null; then
    echo "Generating coverage report..."
    cargo tarpaulin --out Html --output-dir coverage
    echo "✓ Coverage report generated in coverage/"
fi

echo ""
echo "✅ All tests passed!"
