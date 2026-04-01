#!/bin/bash

set -e

echo "========================================="
echo "Building bench.im Monorepo"
echo "========================================="

# Build Server (includes web via rust-embed)
echo ""
echo "[1/2] Building Server (includes embedded web assets)..."
cd server
cargo build --release
cd ..

# Build Client
echo ""
echo "[2/2] Building Client..."
cd client
cargo build --release
cd ..

echo ""
echo "========================================="
echo "Build Complete!"
echo "========================================="
echo ""
echo "Output locations:"
echo "  - Server:  target/release/bim-server (includes embedded web assets)"
echo "  - Client:  target/release/bim"
