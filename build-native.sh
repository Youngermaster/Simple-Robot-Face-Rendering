#!/bin/bash

# Build native macOS ARM64 versions of the robot face

set -e

echo "======================================="
echo "Building Native Robot Face (macOS ARM64)"
echo "======================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Create build directory
mkdir -p build
cd build

# Build Raylib version
echo -e "${YELLOW}Building Raylib native version...${NC}"
cmake .. -DBUILD_SKIA=OFF -DCMAKE_BUILD_TYPE=Release
make -j4

if [ -f "robot_face_raylib" ]; then
    RAYLIB_SIZE=$(du -h robot_face_raylib | awk '{print $1}')
    echo -e "${GREEN}✓ Raylib build successful!${NC}"
    echo "  Binary: build/robot_face_raylib (${RAYLIB_SIZE})"
else
    echo -e "${RED}✗ Raylib build failed${NC}"
    exit 1
fi

# Check if Skia library is built
if [ -f "../skia-lib/out/AppleSilicon/libskia.a" ]; then
    echo -e "${YELLOW}Building Skia native version...${NC}"
    cmake .. -DBUILD_RAYLIB=OFF -DBUILD_SKIA=ON -DCMAKE_BUILD_TYPE=Release
    make -j4

    if [ -f "robot_face_skia" ]; then
        SKIA_SIZE=$(du -h robot_face_skia | awk '{print $1}')
        echo -e "${GREEN}✓ Skia build successful!${NC}"
        echo "  Binary: build/robot_face_skia (${SKIA_SIZE})"
    else
        echo -e "${RED}✗ Skia build failed${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Skia library not built yet${NC}"
    echo "  To build Skia, run:"
    echo "    cd skia-lib"
    echo "    export PATH=\"\${PATH}:../depot_tools\""
    echo "    depot_tools/ninja -C out/AppleSilicon skia -j 4"
fi

cd ..

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Native builds complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "To run:"
echo "  ./build/robot_face_raylib"
if [ -f "build/robot_face_skia" ]; then
    echo "  ./build/robot_face_skia"
fi
