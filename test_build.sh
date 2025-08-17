#!/bin/bash

echo "🚀 Testing ReoswellEcho Trading Bot Build"
echo "=================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust version: $(rustc --version)"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

echo "✅ Project structure verified"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Check dependencies
echo "📦 Checking dependencies..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Dependencies resolved successfully"
else
    echo "❌ Dependency resolution failed"
    exit 1
fi

# Build in debug mode
echo "🔨 Building in debug mode..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Debug build successful"
else
    echo "❌ Debug build failed"
    exit 1
fi

# Build in release mode
echo "🚀 Building in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Release build successful"
else
    echo "❌ Release build failed"
    exit 1
fi

# Test CLI help
echo "📋 Testing CLI interface..."
./target/release/reoswellecho-bagsfm --help

if [ $? -eq 0 ]; then
    echo "✅ CLI interface working"
else
    echo "❌ CLI interface failed"
    exit 1
fi

echo ""
echo "🎉 All tests passed! ReoswellEcho is ready to use."
echo ""
echo "Next steps:"
echo "1. Copy config.template.env to .env"
echo "2. Configure your wallet and RPC settings"
echo "3. Run: cargo run -- --help"
echo ""
echo "Example commands:"
echo "  cargo run -- sniper --token DemoToken123 --budget-sol 0.5"
echo "  cargo run -- bundler --plan plans/example.json --dry-run"
echo "  cargo run -- market-maker --pair Pool123 --spread-bps 100"

