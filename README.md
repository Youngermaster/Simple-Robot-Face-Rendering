# Simple Robot Face Rendering

A comprehensive comparison of **Skia** vs **Raylib** graphics libraries by implementing an animated robot face renderer with both native (macOS ARM64) and WebAssembly builds.

## Features

- **Animated Robot Face**: Two eyes with blinking animation and an expressive mouth
- **Interactive Controls**:
  - H: Happy emotion
  - S: Sad emotion
  - N: Neutral emotion
  - Click: Trigger manual blink
  - Hover over mouth: Increase happiness
- **Performance Metrics**: FPS counter, frame time, binary size comparison
- **Dual Rendering**: Native ARM64 builds + WASM builds for browser

## Project Structure

```
.
├── raylib/                 # Raylib implementation
│   ├── src/               # C source code
│   └── web/               # WASM build scripts
├── skia/                   # Skia implementation
│   ├── src/               # C++ source code
│   └── web/               # CanvasKit WASM build scripts
├── web/                    # Web comparison interface
│   ├── index.html         # Project home page
│   └── comparison.html    # Side-by-side comparison
├── build/                  # Native builds output
└── docs/                   # Performance comparison docs
```

## Quick Start

### Native Builds (macOS ARM64)

```bash
# Build Raylib version
mkdir build && cd build
cmake .. -DBUILD_SKIA=OFF
make
./robot_face_raylib

# Build Skia version (after Skia library is built)
cmake .. -DBUILD_RAYLIB=OFF -DBUILD_SKIA=ON
make
./robot_face_skia
```

### WASM Builds

```bash
# Build Raylib WASM
source emsdk/emsdk_env.sh
cd raylib/web
./build_wasm.sh

# Build Skia CanvasKit WASM
cd skia/web
./build_wasm.sh

# Serve and test
cd web
python3 -m http.server 8000
# Open http://localhost:8000
```

## Dependencies

### macOS

```bash
brew install cmake sdl2 raylib
```

### Skia

Skia is built from source and included in the project setup. The build process automatically handles this.

### Emscripten (for WASM)

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

## Performance Comparison

See `docs/comparison.md` for detailed performance metrics including:

- Native binary sizes
- FPS performance
- WASM bundle sizes
- Load times
- Visual quality comparison

## Technical Details

### Robot Face Specifications

- **Canvas**: 800x600 pixels
- **Eyes**: Two circles at (250, 200) and (550, 200), radius 60px
- **Pupils**: Animate from 40px to 5px during blink
- **Mouth**: Quadratic Bezier curve, control point varies with emotion
- **Blink Animation**: 200ms duration, automatic every 3 seconds

### Rendering Features

**Raylib**:

- Simple 2D API with immediate mode rendering
- Built-in shape primitives and Bezier curves
- Cross-platform compatibility

**Skia**:

- Advanced 2D graphics with antialiasing
- Path-based rendering with effects
- Used by Chrome, Android, and Flutter

## License

MIT License - see LICENSE file for details
