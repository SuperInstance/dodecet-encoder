#!/bin/bash
# Build script for dodecet-wasm package

set -e

echo "🔨 Building dodecet-wasm package..."
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found!"
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Clean previous build
echo "🧹 Cleaning previous build..."
rm -rf pkg
rm -rf ../../target/wasm32-unknown-unknown/release/*.wasm

# Build for web target
echo "📦 Building for web target..."
wasm-pack build --target web --release

# Check if build was successful
if [ -d "pkg" ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📊 Package contents:"
    ls -lh pkg/

    echo ""
    echo "📦 To publish:"
    echo "  cd pkg"
    echo "  npm publish --access public"
else
    echo "❌ Build failed!"
    exit 1
fi
