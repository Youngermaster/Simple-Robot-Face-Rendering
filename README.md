# Graphics Frameworks Exploration: Robot Face Rendering & Developer Tools

A comprehensive exploration of modern graphics and UI frameworks through practical implementations. This repository compares **Skia**, **Raylib**, **Bevy**, and **egui** by building real-world applications including animated robot faces and developer monitoring tools.


## New: Raylib Playground

**Looking to learn Raylib?** Check out [`raylib-playground/`](./raylib-playground) for professional C++ examples:

- **01-solar-system** - Orbital mechanics with physics simulation
- **02-ui-components** - Themes, buttons, and HSV color picker
- **03-particle-system** - 5 emitter types with physics (fire, snow, explosions)
- **04-bezier-curves** - Interactive curve editor (perfect for robot face expressions!)

Each example includes native + WASM builds, separated headers/implementation, and comprehensive documentation. [View full docs →](./raylib-playground/README.md)


## Projects Overview

### 1. Bevy Robot Face (Rust + ECS)

An animated robot face built with the **Bevy game engine**, demonstrating Entity Component System architecture and Rust's powerful type system. Features smooth Bezier curve rendering for natural expressions, modular component design, and cross-platform support including WebAssembly.

**Key Features:**
- Clean ECS architecture with separated components, resources, and systems
- Quadratic Bezier curves for natural mouth expressions
- Smooth blinking animations with periodic auto-blink
- Interactive emotion controls (Happy, Sad, Neutral)
- Modular codebase split across 8 files for maintainability

[View Bevy implementation →](./robot-face-rendering-bevy/)

### 2. Teleoperation Monitoring System (egui + Clean Architecture)

A professional-grade monitoring dashboard built with **egui**, following Clean Architecture principles. Demonstrates real-time sensor data visualization with WebSocket streaming and HTTP APIs, structured in four distinct layers for maximum maintainability.

**Key Features:**
- Clean Architecture: Domain, Application, Infrastructure, Presentation layers
- Real-time WebSocket streaming for Temperature (10Hz) and IMU (20Hz) sensors
- HTTP API for Room Occupancy state updates
- Live plotting with egui_plot for stock-market-style visualizations
- Python sensor simulators included
- Production-ready error handling and validation

[View egui implementation →](./teleoperations-egui/)

### 3. Skia vs Raylib Comparison

Production-grade graphics library comparison implementing identical robot faces in both frameworks, with native and WASM builds for objective performance analysis.

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

## Repository Structure

```
.
├── robot-face-rendering-bevy/    # Bevy (Rust ECS) implementation
│   ├── src/
│   │   ├── components.rs          # ECS components
│   │   ├── resources.rs           # Global resources
│   │   ├── systems/               # Game logic systems
│   │   ├── constants.rs           # Configuration
│   │   └── utils.rs               # Bezier curve generation
│   └── Cargo.toml
│
├── teleoperations-egui/          # egui monitoring dashboard
│   ├── src/
│   │   ├── domain/                # Pure business logic
│   │   ├── application/           # Use cases & services
│   │   ├── infrastructure/        # Network handlers
│   │   ├── presentation/          # UI panels
│   │   └── main.rs                # Entry point (64 lines)
│   ├── python/                    # Sensor simulators
│   │   ├── temperature_sensor.py
│   │   ├── imu_sensor.py
│   │   └── occupancy_sensor.py
│   └── Cargo.toml
│
├── raylib/                        # Raylib C implementation
│   ├── src/                       # C source code
│   └── web/                       # WASM build scripts
│
├── skia/                          # Skia C++ implementation
│   ├── src/                       # C++ source code
│   └── web/                       # CanvasKit WASM build scripts
│
├── raylib-playground/             # Learning examples
│   ├── 01-solar-system/
│   ├── 02-ui-components/
│   ├── 03-particle-system/
│   └── 04-bezier-curves/
│
├── web/                           # Web comparison interface
│   ├── index.html
│   └── comparison.html
│
├── build/                         # Native builds output
└── docs/                          # Performance comparisons
```

## Quick Start

### Bevy Robot Face

```bash
cd robot-face-rendering-bevy

# Native build and run
cargo run --release

# WASM build
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/robot-face-rendering-bevy.wasm
```

**Controls:** H (Happy), S (Sad), N (Neutral), Space (Blink), ESC (Exit)

### egui Teleoperation Monitoring

```bash
cd teleoperations-egui

# Terminal 1: Run the monitoring dashboard
cargo run --release

# Terminal 2: Start temperature sensor (10Hz)
python3 python/temperature_sensor.py

# Terminal 3: Start IMU sensor (20Hz)
python3 python/imu_sensor.py

# Terminal 4: Start occupancy sensor (0.5Hz)
python3 python/occupancy_sensor.py
```

Server runs on `http://localhost:8080` with WebSocket and HTTP endpoints.

### Raylib/Skia Comparison (Native Builds)

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

## Framework Comparison

### Comprehensive Feature Matrix

| Feature | Skia | Raylib | Bevy | egui |
|---------|------|--------|------|------|
| **Primary Use Case** | 2D graphics engine | Game framework | Game engine | Immediate-mode UI |
| **Language** | C++ | C | Rust | Rust |
| **Rendering Quality** | ★★★★★ | ★★★★ | ★★★★ | ★★★ |
| **API Simplicity** | ★★★ | ★★★★★ | ★★★ | ★★★★★ |
| **Performance** | ★★★★★ | ★★★★ | ★★★★★ | ★★★★ |
| **WASM Support** | ★★★★★ (CanvasKit) | ★★★★ | ★★★★★ | ★★★★★ |
| **Learning Curve** | Steep | Gentle | Moderate | Gentle |
| **Architecture** | Retained mode | Immediate mode | ECS | Immediate mode |
| **Binary Size** | ~10MB | ~2-3MB | ~8MB | ~5MB |
| **Ecosystem** | Massive (Google) | Active | Growing fast | Rust-focused |
| **Type Safety** | Manual | Manual | Strong (Rust) | Strong (Rust) |
| **Compile Times** | Slow | Fast | Moderate | Fast |
| **Hot Reload** | No | No | Yes (experimental) | Yes (native) |
| **Built-in UI** | No | Basic | Minimal | Core feature |
| **Animation System** | Manual | Manual | Built-in | Manual |
| **3D Support** | No | Yes | Yes | No |
| **Cross-Platform** | Excellent | Excellent | Excellent | Excellent |

### Use Case Recommendations

**Choose Skia when:**
- You need production-grade 2D rendering quality
- Building browser-based graphics applications
- Performance and visual quality are critical
- You're comfortable with C++ and manual memory management
- Examples: Document renderers, vector graphics editors, charting libraries

**Choose Raylib when:**
- Rapid prototyping is the priority
- You want simple, beginner-friendly API
- Building games or interactive demos
- WASM size matters (smallest footprint)
- Examples: Game jams, educational projects, indie games

**Choose Bevy when:**
- Building complex games or simulations
- You want type-safe, modern architecture
- ECS pattern fits your domain model
- Leveraging Rust's safety guarantees
- Examples: Strategy games, robotics simulators, data visualizations

**Choose egui when:**
- Building developer tools or dashboards
- You need immediate-mode UI reactivity
- Rapid iteration and hot reload are important
- UI is the primary focus, not graphics
- Examples: Profilers, monitoring tools, debug interfaces, control panels

### Project Statistics

| Metric | Bevy Robot Face | egui Monitoring | Raylib Face | Skia Face |
|--------|-----------------|-----------------|-------------|-----------|
| **Lines of Code** | ~400 (8 files) | ~1400 (13 modules) | ~250 | ~300 |
| **Architecture** | Modular ECS | Clean Architecture | Single file | Single file |
| **Dependencies** | Bevy 0.17, rand | egui, axum, tokio | Raylib | Skia |
| **Compile Time** | ~30s (release) | ~25s (release) | <5s | ~15s |
| **Runtime Deps** | None | Python (sensors) | SDL2 | SDL2 |
| **WASM Ready** | Yes | Yes (UI only) | Yes | Yes (CanvasKit) |

## Summary

This repository demonstrates practical implementations across four major graphics/UI frameworks, each with distinct strengths:

**Skia** excels at production-quality 2D rendering with battle-tested stability from billions of Chrome/Android devices. The API is complex but powerful, making it ideal for applications where visual quality cannot be compromised.

**Raylib** provides the gentlest learning curve with a delightfully simple API. Perfect for beginners, prototypes, and situations where getting something working quickly matters more than architectural sophistication.

**Bevy** brings modern game engine architecture to Rust with its Entity Component System. The type system catches bugs at compile time, and the modular design scales well to complex applications. Hot reload and strong tooling make iteration fast.

**egui** redefines immediate-mode UI with Rust safety. The reactive model eliminates state synchronization bugs, and the library feels native to Rust. Exceptional for developer tools where UI responsiveness and ease of modification are paramount.

The **Clean Architecture refactoring** of the egui teleoperation system demonstrates how to structure Rust applications for maintainability. By separating domain logic, application services, infrastructure adapters, and presentation layers, the codebase remains understandable and testable even as complexity grows.

All implementations are production-ready starting points for robotics interfaces, monitoring dashboards, or interactive visualizations. The code prioritizes clarity and best practices over cleverness.

## License

MIT License - see LICENSE file for details
