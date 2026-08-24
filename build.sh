#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "=== 1. Building SSG Binary ==="
cargo build --release --bin portfolio-ssg

echo "=== 2. Running SSG to generate static HTML and copy assets to dist/ ==="
./target/release/portfolio-ssg

echo "=== 3. Compiling WASM module with wasm-pack ==="
wasm-pack build --target web --out-dir dist/pkg --no-typescript --release

echo "=== 4. Ensuring all static files in dist ==="
cp -r static/* dist/

echo "=== Build Complete! ==="
