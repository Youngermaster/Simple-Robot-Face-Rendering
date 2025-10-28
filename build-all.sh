#!/bin/bash

# Master build script - builds all versions

echo "======================================"
echo "   Robot Face - Build All Targets    "
echo "======================================"
echo ""
echo "This will build:"
echo "  1. Native macOS ARM64 versions"
echo "  2. WebAssembly versions"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Build native
echo -e "${YELLOW}Step 1/2: Building native versions...${NC}"
./build-native.sh

echo ""
echo -e "${YELLOW}Step 2/2: Building WASM versions...${NC}"
./build-wasm.sh

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   All builds complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Summary:"
if [ -f "build/robot_face_raylib" ]; then
    RAYLIB_SIZE=$(du -h build/robot_face_raylib | awk '{print $1}')
    echo -e "  ${GREEN}✓${NC} Raylib native (${RAYLIB_SIZE})"
fi
if [ -f "build/robot_face_skia" ]; then
    SKIA_SIZE=$(du -h build/robot_face_skia | awk '{print $1}')
    echo -e "  ${GREEN}✓${NC} Skia native (${SKIA_SIZE})"
fi
if [ -f "raylib/web/web_output/robot_face_raylib.wasm" ]; then
    WASM_SIZE=$(du -h raylib/web/web_output/robot_face_raylib.wasm | awk '{print $1}')
    echo -e "  ${GREEN}✓${NC} Raylib WASM (${WASM_SIZE})"
fi
if [ -f "skia/web/web_output/robot_face_skia.wasm" ]; then
    SKIA_WASM_SIZE=$(du -h skia/web/web_output/robot_face_skia.wasm | awk '{print $1}')
    echo -e "  ${GREEN}✓${NC} Skia WASM (${SKIA_WASM_SIZE})"
fi

echo ""
echo "Next steps:"
echo "  Run native: ./run-native.sh"
echo "  Test WASM:  ./run-web.sh"
