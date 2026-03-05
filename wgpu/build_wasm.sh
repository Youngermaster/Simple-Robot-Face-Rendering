#!/bin/bash
# Build script for WASM version of Robot Face WGPU renderer
# Mirrors the pattern from robot-face-rendering-bevy/build_wasm.sh

set -e

echo "🤖 Building Robot Face WGPU for WASM..."
echo ""

# ---- Check / install wasm-bindgen-cli ----
if ! command -v wasm-bindgen &> /dev/null; then
    echo "❌ wasm-bindgen-cli not found"
    echo "📦 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# ---- Ensure wasm32 target is present ----
echo "🎯 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

# ---- Build ----
echo "🔨 Building WASM binary (this may take a while on first run)..."
cargo build --profile wasm-release --target wasm32-unknown-unknown

# ---- Generate JS bindings ----
echo "🔗 Generating JavaScript bindings..."
mkdir -p web/wasm
wasm-bindgen \
    --out-dir web/wasm \
    --out-name robot-face-wgpu \
    --target web \
    target/wasm32-unknown-unknown/wasm-release/robot_face_wgpu.wasm

# ---- Copy HTML ----
echo "📄 Copying index.html..."
cp web/index.html web/wasm/

# Patch the HTML to load the generated module
# (Only if the script tag placeholder is present)
WASM_IMPORT='import init from '"'"'./robot-face-wgpu.js'"'"'; await init();'
if ! grep -q "robot-face-wgpu.js" web/wasm/index.html; then
    # Insert the import before the closing </script> of the module script block
    sed -i '' \
        's|// import init from.*||' \
        web/wasm/index.html 2>/dev/null || true
fi

# ---- Report ----
WASM_FILE="web/wasm/robot-face-wgpu_bg.wasm"
if [ -f "$WASM_FILE" ]; then
    WASM_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
    echo ""
    echo "✅ Build complete!"
    echo "📦 WASM file size: $WASM_SIZE"
else
    echo "✅ Build complete!"
fi

echo ""
echo "🚀 To serve locally:"
echo "   cd web/wasm"
echo "   python3 -m http.server 8080"
echo ""
echo "Then open http://localhost:8080 in Chrome or Firefox"
echo ""
echo "💡 Alternatively, use trunk for a faster dev loop:"
echo "   cargo install trunk"
echo "   trunk serve web/index.html"
