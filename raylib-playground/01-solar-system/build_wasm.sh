#!/bin/bash
set -e

echo "Building Solar System for WASM..."

# Check if Emscripten is available
if ! command -v emcc &> /dev/null; then
    echo "Error: Emscripten not found!"
    echo "Please install and activate Emscripten:"
    echo "  source ~/emsdk/emsdk_env.sh"
    exit 1
fi

# Create output directory
mkdir -p web_output

# Compile all source files
emcc src/main.cpp src/planet.cpp src/solar_system.cpp \
    -o web_output/solar_system.html \
    -I include \
    -I/opt/homebrew/include \
    -L/opt/homebrew/lib \
    -Os \
    -Wall \
    -s USE_GLFW=3 \
    -s ASYNCIFY \
    -s TOTAL_MEMORY=67108864 \
    -DPLATFORM_WEB \
    --shell-file /opt/homebrew/Cellar/raylib/*/share/raylib/shell.html \
    -lraylib

echo ""
echo "WASM build complete!"
echo "Files generated in web_output/"
echo ""
echo "To run locally:"
echo "  cd web_output"
echo "  python3 -m http.server 8000"
echo "  Open http://localhost:8000/solar_system.html"
