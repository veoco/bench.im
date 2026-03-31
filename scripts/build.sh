#!/bin/bash

set -e

echo "========================================="
echo "Building bench.im Monorepo"
echo "========================================="

# Build Web
echo ""
echo "[1/3] Building Web Frontend..."
cd web
npm ci
npm run build
cd ..

# Build Server
echo ""
echo "[2/3] Building Server..."
cd server
cargo build --release
cd ..

# Build Client
echo ""
echo "[3/3] Building Client..."
cd client
cargo build --release
cd ..

echo ""
echo "========================================="
echo "Build Complete!"
echo "========================================="
echo ""
echo "Output locations:"
echo "  - Web:     web/dist/"
echo "  - Server:  server/target/release/bim-server"
echo "  - Client:  client/target/release/bim"
