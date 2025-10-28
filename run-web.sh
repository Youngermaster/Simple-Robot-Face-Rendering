#!/bin/bash

# Start web server to test WASM builds

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "======================================"
echo "   Robot Face - Web Server"
echo "======================================"
echo ""

# Check if WASM builds exist
RAYLIB_WASM=false
SKIA_WASM=false

if [ -f "raylib/web/web_output/robot_face_raylib.html" ]; then
    RAYLIB_WASM=true
    WASM_SIZE=$(du -h raylib/web/web_output/robot_face_raylib.wasm | awk '{print $1}')
fi

if [ -f "skia/web/web_output/robot_face_skia.html" ]; then
    SKIA_WASM=true
fi

if [ "$RAYLIB_WASM" = false ] && [ "$SKIA_WASM" = false ]; then
    echo -e "${YELLOW}⚠ No WASM builds found${NC}"
    echo ""
    echo "Build WASM first:"
    echo "  ./build-wasm.sh"
    echo ""
    echo "Starting web server anyway for comparison page..."
    echo ""
fi

# Show what's available
echo "Available demos:"
if [ "$RAYLIB_WASM" = true ]; then
    echo -e "  ${GREEN}✓${NC} Raylib WASM (${WASM_SIZE})"
    echo "    → http://localhost:8000/raylib/web/web_output/robot_face_raylib.html"
fi
if [ "$SKIA_WASM" = true ]; then
    echo -e "  ${GREEN}✓${NC} Skia CanvasKit WASM"
    echo "    → http://localhost:8000/skia/web/web_output/robot_face_skia.html"
fi
echo -e "  ${GREEN}✓${NC} Comparison page (always available)"
echo "    → http://localhost:8000/web/comparison.html"
echo ""

# Start server
echo -e "${YELLOW}Starting web server on port 8000...${NC}"
echo "Press Ctrl+C to stop"
echo ""

cd web
python3 -m http.server 8000
