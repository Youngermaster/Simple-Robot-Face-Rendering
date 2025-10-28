#!/bin/bash
set -e

echo "================================"
echo "Building Raylib Robot Face WASM"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if emcc is available
if ! command -v emcc &> /dev/null; then
    echo -e "${RED}Error: emcc not found!${NC}"
    echo "Please activate emsdk first:"
    echo "  source ../../emsdk/emsdk_env.sh"
    exit 1
fi

echo -e "${GREEN}✓${NC} Emscripten found: $(emcc --version | head -n 1)"

# Create output directory
mkdir -p web_output

# Clone raylib if not present
if [ ! -d "raylib_src" ]; then
    echo -e "${YELLOW}Cloning Raylib 5.5 for WASM build...${NC}"
    git clone --depth 1 --branch 5.5 https://github.com/raysan5/raylib.git raylib_src
fi

# Build raylib library for web if not already built
if [ ! -f "raylib_src/src/libraylib.a" ]; then
    echo -e "${YELLOW}Building Raylib library for WASM...${NC}"
    cd raylib_src/src
    make PLATFORM=PLATFORM_WEB -j4
    cd ../..
fi

echo -e "${YELLOW}Building robot face WASM...${NC}"

# Build the robot face with the raylib library
emcc ../src/robot_face_raylib.c \
    -o web_output/robot_face_raylib.html \
    -I raylib_src/src \
    raylib_src/src/libraylib.a \
    -s USE_GLFW=3 \
    -s ASYNCIFY \
    -s TOTAL_MEMORY=67108864 \
    -s ALLOW_MEMORY_GROWTH=1 \
    -DPLATFORM_WEB \
    -Os \
    -Wall \
    --shell-file shell_minimal.html

if [ -f "web_output/robot_face_raylib.html" ]; then
    echo -e "${GREEN}✓ Build successful!${NC}"
    echo ""
    echo "Output files:"
    ls -lh web_output/
    echo ""
    if [ -f "web_output/robot_face_raylib.wasm" ]; then
        WASM_SIZE=$(du -h web_output/robot_face_raylib.wasm | awk '{print $1}')
        JS_SIZE=$(du -h web_output/robot_face_raylib.js | awk '{print $1}')
        echo "WASM size: ${WASM_SIZE}"
        echo "JS size: ${JS_SIZE}"
    fi
    echo ""
    echo "To test, run:"
    echo "  cd web_output && python3 -m http.server 8000"
    echo "  Then open: http://localhost:8000/robot_face_raylib.html"
else
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi
