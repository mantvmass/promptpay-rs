#!/bin/bash

# PromptPay-RS WebAssembly Build Script
# This script builds the Rust library for WebAssembly targets

set -e

echo "🚀 Building PromptPay-RS for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf dist target

# Build for web browsers
echo "🌐 Building for web browsers..."
wasm-pack build --target web --out-dir dist

# Build for Node.js
echo "📦 Building for Node.js..."
wasm-pack build --target nodejs --out-dir dist-node

# Build for bundlers
echo "📦 Building for bundlers..."
wasm-pack build --target bundler --out-dir dist-bundler

# Copy TypeScript definitions
echo "📝 Copying TypeScript definitions..."
cp types/promptpay-rs.d.ts dist/
cp types/promptpay-rs.d.ts dist-node/
cp types/promptpay-rs.d.ts dist-bundler/

# Copy package.json for npm publishing
echo "📄 Copying package.json..."
cp package.json dist/

# Run tests
echo "🧪 Running tests..."
cargo test
wasm-pack test --headless --firefox

echo "✅ Build completed successfully!"
echo ""
echo "📁 Build outputs:"
echo "  - dist/          (Web browsers)"
echo "  - dist-node/     (Node.js)"
echo "  - dist-bundler/  (Bundlers)"
echo ""
echo "🚀 To publish to npm:"
echo "  cd dist && npm publish" 