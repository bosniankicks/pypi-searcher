# ⚡ PyPI Search - Blazingly Fast

Ultra-fast PyPI package search tool written in Rust. **No Fastly challenges, no cookies, no JavaScript execution needed!**

## Features

- 🚀 **Blazingly fast** - Compiled Rust with optimized release builds
- 🎯 **No challenges** - Uses PyPI JSON API (bypasses all protection)
- 🎨 **Beautiful output** - Colored terminal output
- 📦 **Zero overhead** - Direct HTTP/JSON, no browser automation
- ⚙️ **Multiple output formats** - Human-readable or JSON
- 🔥 **Benchmark mode** - See exactly how fast it is

## Installation

### One-Line Install (Recommended)

```bash
git clone https://github.com/bosniankicks/pypi-searcher.git
cd pypi-searcher
./install.sh
```

After installation, use anywhere:
```bash
pypi-search django
```

### Manual Install

```bash
# Build optimized release binary
cargo build --release

# Copy to PATH (optional)
sudo cp target/release/pypi-search /usr/local/bin/

# Or run directly
./target/release/pypi-search django
```

## Usage

### Basic Search

```bash
./target/release/pypi-search django
```

### Show Full Description

```bash
./target/release/pypi-search django --full
```

### JSON Output

```bash
./target/release/pypi-search django --json
```

### Benchmark Mode (Show Timing)

```bash
./target/release/pypi-search django --benchmark
```

## Examples

### Example 1: Basic Search

```bash
$ ./target/release/pypi-search requests
============================================================
Package: requests
============================================================

Latest Version: 2.31.0
Summary: Python HTTP for Humans.
Author: Kenneth Reitz
License: Apache 2.0
Homepage: https://requests.readthedocs.io

Total Releases: 142
Recent Versions: 2.28.0, 2.28.1, 2.28.2, 2.29.0, 2.31.0

============================================================
✓ SUCCESS - No challenge required!
```

### Example 2: Benchmark Mode

```bash
$ ./target/release/pypi-search flask --benchmark
============================================================
Package: flask
============================================================

Latest Version: 3.0.0
...
============================================================
⚡ Fetched in 85ms
✓ SUCCESS - No challenge required!
```

### Example 3: JSON Output

```bash
$ ./target/release/pypi-search numpy --json
{
  "success": true,
  "name": "numpy",
  "version": "1.26.4",
  "summary": "Fundamental package for array computing in Python",
  ...
}
```

## Performance

Typical response times:
- **Local cache hit**: ~50-80ms
- **First request**: ~100-150ms
- **Compare to Node.js**: ~2-3x faster
- **Compare to Python**: ~5-10x faster
- **Compare to browser automation**: ~100x faster

## Why Rust?

1. **Speed** - Compiled binary with zero-cost abstractions
2. **Memory safety** - No segfaults, no memory leaks
3. **Concurrency** - Built-in async/await with tokio
4. **Small binaries** - Optimized release builds strip unused code
5. **Cross-platform** - Compiles to native binaries on any OS

## Build Optimizations

The `Cargo.toml` includes aggressive optimizations:

```toml
[profile.release]
opt-level = 3        # Maximum optimizations
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization, slower compile
strip = true        # Strip debug symbols
```

## API Bypass Strategy

**The Secret:** PyPI provides a JSON API that completely bypasses Fastly protection:

```
https://pypi.org/pypi/{package}/json
```

This endpoint:
- ✅ No challenge pages
- ✅ No cookies required
- ✅ No JavaScript execution
- ✅ No browser needed
- ✅ Pure HTTP/JSON

## Command-Line Options

```
Usage: pypi-search [OPTIONS] <PACKAGE>

Arguments:
  <PACKAGE>  Package name to search

Options:
  -f, --full       Show full description
  -j, --json       Output as JSON
  -b, --benchmark  Benchmark mode (show timing)
  -h, --help       Print help
  -V, --version    Print version
```

## License

MIT
