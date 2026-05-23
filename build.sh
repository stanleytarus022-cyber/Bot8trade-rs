#!/bin/bash

# Bot8trade-rs Build & Deploy Script

set -e

echo "🚀 Bot8trade-rs Build Pipeline"
echo "================================"

# Check prerequisites
echo "✓ Checking prerequisites..."
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo not found. Install from https://rustup.rs/"; exit 1; }
command -v sqlite3 >/dev/null 2>&1 || { echo "❌ SQLite3 not found. Install it first."; exit 1; }

# Load environment
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "⚠️  .env file not found. Using defaults."
fi

# Format code
echo "\n📝 Formatting code..."
cargo fmt --all

# Run clippy
echo "\n🔍 Running clippy lint..."
cargo clippy --all -- -D warnings || true

# Run tests
echo "\n🧪 Running tests..."
cargo test --all --verbose

# Build release
echo "\n🔨 Building release binary..."
cargo build --release

# Create database
echo "\n💾 Initializing database..."
sqlite3 bot8trade.db < scripts/schema.sql || echo "Database already initialized"

echo "\n✅ Build completed successfully!"
echo "\nNext steps:"
echo "1. Configure .env file with your MT5 credentials"
echo "2. Run: cargo run --release"
echo "3. Check logs in: tail -f logs/bot8trade.log"
