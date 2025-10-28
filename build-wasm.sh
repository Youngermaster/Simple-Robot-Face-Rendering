#!/bin/bash

# Build WASM versions of the robot face

set -e

echo "======================================"
echo "Building Robot Face WASM Versions"
echo "======================================"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check if emsdk is available
if [ ! -d "emsdk" ]; then
    echo -e "${RED}Error: emsdk not found!${NC}"
    echo "Please install emsdk first:"
    echo "  git clone https://github.com/emscripten-core/emsdk.git"
    echo "  cd emsdk"
    echo "  ./emsdk install latest"
    echo "  ./emsdk activate latest"
    exit 1
fi

# Activate Emscripten
echo -e "${YELLOW}Activating Emscripten...${NC}"
source emsdk/emsdk_env.sh

# Build Raylib WASM
echo ""
echo -e "${YELLOW}Building Raylib WASM...${NC}"
cd raylib/web
EMSDK_QUIET=1 ./build_wasm.sh

if [ -f "web_output/robot_face_raylib.wasm" ]; then
    WASM_SIZE=$(du -h web_output/robot_face_raylib.wasm | awk '{print $1}')
    echo -e "${GREEN}✓ Raylib WASM build successful!${NC}"
    echo "  WASM: raylib/web/web_output/ (${WASM_SIZE})"
else
    echo -e "${RED}✗ Raylib WASM build failed${NC}"
fi

cd ../..

# Build Skia WASM (if available)
if [ -f "skia/web/build_wasm.sh" ]; then
    echo ""
    echo -e "${YELLOW}Building Skia CanvasKit WASM...${NC}"
    cd skia/web
    ./build_wasm.sh || echo -e "${YELLOW}⚠ Skia WASM build not ready yet${NC}"
    cd ../..
else
    echo -e "${YELLOW}⚠ Skia WASM build script not created yet${NC}"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}WASM builds complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "To test, run:"
echo "  cd web && python3 -m http.server 8000"
echo "  Then open: http://localhost:8000"
