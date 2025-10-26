#!/bin/bash
# OWLSOL-CLI Test Runner

set -e

# Run unit tests
cargo test --test unit -- --nocapture

# Run edge case tests
cargo test --test edge_cases -- --nocapture

# Run integration tests (ignored by default)
cargo test --test integration -- --ignored --nocapture

# Run stress tests (ignored by default)
cargo test --test stress -- --ignored --nocapture
