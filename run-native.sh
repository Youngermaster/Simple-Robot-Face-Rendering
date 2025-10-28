#!/bin/bash

# Run native builds with option selection

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "======================================"
echo "   Run Native Robot Face"
echo "======================================"
echo ""

# Check what's available
RAYLIB_AVAILABLE=false
SKIA_AVAILABLE=false

if [ -f "build/robot_face_raylib" ]; then
    RAYLIB_AVAILABLE=true
    RAYLIB_SIZE=$(du -h build/robot_face_raylib | awk '{print $1}')
fi

if [ -f "build/robot_face_skia" ]; then
    SKIA_AVAILABLE=true
    SKIA_SIZE=$(du -h build/robot_face_skia | awk '{print $1}')
fi

if [ "$RAYLIB_AVAILABLE" = false ] && [ "$SKIA_AVAILABLE" = false ]; then
    echo -e "${RED}No native builds found!${NC}"
    echo ""
    echo "Please build first:"
    echo "  ./build-native.sh"
    exit 1
fi

echo "Available builds:"
if [ "$RAYLIB_AVAILABLE" = true ]; then
    echo -e "  ${GREEN}1)${NC} Raylib (${RAYLIB_SIZE})"
fi
if [ "$SKIA_AVAILABLE" = true ]; then
    echo -e "  ${GREEN}2)${NC} Skia (${SKIA_SIZE})"
fi
if [ "$RAYLIB_AVAILABLE" = true ] && [ "$SKIA_AVAILABLE" = true ]; then
    echo -e "  ${GREEN}3)${NC} Both (side by side)"
fi

echo ""
echo -n "Select which to run (or press Enter for default): "
read choice

case $choice in
    1)
        if [ "$RAYLIB_AVAILABLE" = true ]; then
            echo -e "${YELLOW}Running Raylib...${NC}"
            ./build/robot_face_raylib
        fi
        ;;
    2)
        if [ "$SKIA_AVAILABLE" = true ]; then
            echo -e "${YELLOW}Running Skia...${NC}"
            ./build/robot_face_skia
        fi
        ;;
    3)
        if [ "$RAYLIB_AVAILABLE" = true ] && [ "$SKIA_AVAILABLE" = true ]; then
            echo -e "${YELLOW}Running both...${NC}"
            ./build/robot_face_raylib &
            ./build/robot_face_skia &
            wait
        fi
        ;;
    *)
        # Default to Raylib if available
        if [ "$RAYLIB_AVAILABLE" = true ]; then
            echo -e "${YELLOW}Running Raylib (default)...${NC}"
            ./build/robot_face_raylib
        elif [ "$SKIA_AVAILABLE" = true ]; then
            echo -e "${YELLOW}Running Skia...${NC}"
            ./build/robot_face_skia
        fi
        ;;
esac
