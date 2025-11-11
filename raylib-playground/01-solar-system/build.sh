#!/bin/bash
set -e

echo "Building Solar System example..."

# Create build directory
mkdir -p build
cd build

# Configure and build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . -j$(sysctl -n hw.ncpu)

echo ""
echo "Build complete!"
echo "Run with: ./build/solar_system"
