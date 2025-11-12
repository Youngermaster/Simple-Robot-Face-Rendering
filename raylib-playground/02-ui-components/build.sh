#!/bin/bash
set -e

echo "Building UI Components example..."

# Create build directory
mkdir -p build
cd build

# Configure and build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . -j$(sysctl -n hw.ncpu)

echo ""
echo "Build complete!"
echo "Run with: ./build/ui_components"
