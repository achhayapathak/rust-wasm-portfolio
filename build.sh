#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

# Ensure Cargo/Rust bin directory is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# 1. Install Rust & wasm target if not found (e.g. in Cloudflare Pages build container)
if ! command -v cargo &> /dev/null; then
    echo "=== Rust not found. Installing Rust toolchain... ==="
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --target wasm32-unknown-unknown
    source "$HOME/.cargo/env" || true
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Ensure wasm32-unknown-unknown target is installed
if command -v rustup &> /dev/null; then
    rustup target add wasm32-unknown-unknown || true
fi

# 2. Install wasm-pack if not found
if ! command -v wasm-pack &> /dev/null; then
    echo "=== wasm-pack not found. Installing wasm-pack... ==="
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    export PATH="$HOME/.cargo/bin:$PATH"
fi

echo "=== 1. Building SSG Binary ==="
cargo build --release --bin portfolio-ssg

echo "=== 2. Running SSG to generate static HTML and copy assets to dist/ ==="
./target/release/portfolio-ssg

echo "=== 3. Compiling WASM module with wasm-pack ==="
wasm-pack build --target web --out-dir dist/pkg --no-typescript --release

echo "=== 4. Ensuring all static files in dist ==="
cp -r static/* dist/

echo "=== Build Complete! ==="
