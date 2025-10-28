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

# Check for raylib
RAYLIB_PATH="/opt/homebrew"
if [ ! -f "$RAYLIB_PATH/lib/libraylib.a" ]; then
    echo -e "${YELLOW}Warning: Raylib not found at $RAYLIB_PATH${NC}"
    echo "Attempting to build with system raylib..."
fi

echo -e "${YELLOW}Building WASM...${NC}"

# Build Raylib for WASM
# Note: We're using a simpler approach - compile directly with emscripten
emcc ../src/robot_face_raylib.c \
    -o web_output/robot_face_raylib.html \
    -s USE_GLFW=3 \
    -s ASYNCIFY \
    -s TOTAL_MEMORY=67108864 \
    -s ALLOW_MEMORY_GROWTH=1 \
    -DPLATFORM_WEB \
    -Os \
    -Wall \
    -I/opt/homebrew/include \
    -L/opt/homebrew/lib \
    -lraylib \
    --shell-file shell_minimal.html \
    || echo -e "${RED}Build failed. Trying alternative method...${NC}"

# If the above fails, try building raylib from source for WASM
if [ ! -f "web_output/robot_face_raylib.html" ]; then
    echo -e "${YELLOW}Attempting to build with embedded raylib...${NC}"

    # Clone raylib if not present
    if [ ! -d "raylib_src" ]; then
        git clone --depth 1 --branch 5.5 https://github.com/raysan5/raylib.git raylib_src
    fi

    # Build with raylib source
    emcc ../src/robot_face_raylib.c \
        raylib_src/src/rcore.c \
        raylib_src/src/rshapes.c \
        raylib_src/src/rtextures.c \
        raylib_src/src/rtext.c \
        raylib_src/src/rmodels.c \
        raylib_src/src/utils.c \
        raylib_src/src/raudio.c \
        -o web_output/robot_face_raylib.html \
        -I raylib_src/src \
        -I raylib_src/src/external \
        -L raylib_src/src \
        -s USE_GLFW=3 \
        -s ASYNCIFY \
        -s TOTAL_MEMORY=67108864 \
        -s ALLOW_MEMORY_GROWTH=1 \
        -s GL_ENABLE_GET_PROC_ADDRESS=1 \
        -DPLATFORM_WEB \
        -DGRAPHICS_API_OPENGL_ES2 \
        -Os \
        -Wall
fi

if [ -f "web_output/robot_face_raylib.html" ]; then
    echo -e "${GREEN}✓ Build successful!${NC}"
    echo ""
    echo "Output files:"
    ls -lh web_output/
    echo ""
    echo "To test, run:"
    echo "  cd web_output && python3 -m http.server 8000"
    echo "  Then open: http://localhost:8000/robot_face_raylib.html"
else
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi
