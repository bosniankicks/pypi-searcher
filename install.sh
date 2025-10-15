#!/bin/bash
# PyPI Search - One-line installer

set -e

echo "Installing PyPI Search..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust/Cargo not found!"
    echo ""
    echo "Install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

echo "[1/3] Building optimized binary..."
cargo build --release --quiet

echo "[2/3] Installing to /usr/local/bin..."
if [ -w "/usr/local/bin" ]; then
    cp target/release/pypi-search /usr/local/bin/
else
    sudo cp target/release/pypi-search /usr/local/bin/
fi

echo "[3/3] Testing installation..."
if command -v pypi-search &> /dev/null; then
    echo ""
    echo "SUCCESS! PyPI Search installed!"
    echo ""
    echo "Usage:"
    echo "  pypi-search django"
    echo "  pypi-search requests --benchmark"
    echo "  pypi-search numpy --json"
    echo ""
else
    echo "ERROR: Installation failed"
    exit 1
fi
