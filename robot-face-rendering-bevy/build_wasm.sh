#!/bin/bash

# Build script for WASM version of Robot Face Rendering with Bevy
# This script builds the project for WebAssembly and prepares it for web deployment

set -e

echo "🤖 Building Robot Face Rendering for WASM..."
echo ""

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "❌ wasm-bindgen-cli is not installed"
    echo "📦 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Check if basic-http-server is installed (optional but recommended)
if ! command -v basic-http-server &> /dev/null; then
    echo "💡 Tip: Install basic-http-server for easy local testing:"
    echo "   cargo install basic-http-server"
    echo ""
fi

# Add wasm32 target if not already added
echo "🎯 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

# Build the WASM binary
echo "🔨 Building WASM binary (this may take a while)..."
cargo build --release --target wasm32-unknown-unknown

# Generate JavaScript bindings
echo "🔗 Generating JavaScript bindings..."
wasm-bindgen --out-dir wasm --out-name robot-face-rendering-bevy --target web \
    target/wasm32-unknown-unknown/release/robot-face-rendering-bevy.wasm

# Copy index.html to wasm directory
echo "📄 Copying index.html..."
cp index.html wasm/

# Get the WASM file size
WASM_SIZE=$(ls -lh wasm/robot-face-rendering-bevy_bg.wasm | awk '{print $5}')

echo ""
echo "✅ Build complete!"
echo "📦 WASM file size: $WASM_SIZE"
echo ""
echo "🚀 To run locally:"
echo "   cd wasm"
echo "   python3 -m http.server 8080"
echo "   # OR if you have basic-http-server installed:"
echo "   basic-http-server ."
echo ""
echo "Then open http://localhost:8080 in your browser"
echo ""
echo "🌐 To deploy, upload the contents of the 'wasm' directory to your web server"
