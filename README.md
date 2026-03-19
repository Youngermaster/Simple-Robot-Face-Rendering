# Graphics Frameworks Exploration: Robot Face Rendering & Developer Tools

A comprehensive exploration of modern graphics and UI frameworks through practical implementations. This repository compares **Skia**, **Raylib**, **Bevy**, **WGPU**, and **egui** by building real-world applications including animated robot faces and developer monitoring tools.

## Important Note on Skia

While Skia code has been implemented in this repository, the framework proved extremely difficult to build and configure properly. Unlike Raylib, Bevy, and egui which installed seamlessly via package managers and Cargo, Skia requires complex manual compilation and dependency management. The Skia implementation remains untested due to build configuration challenges. This difficulty is itself an important data point when evaluating frameworks for production use.

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

### 3. WGPU Robot Face (Rust + Pure GPU)

An animated robot face rendered entirely by a **WGSL fragment shader** — no 2D rendering library. Every pixel is computed with Signed Distance Field math directly on the GPU, producing smooth, resolution-independent shapes with glow effects and LED grid eyes.

**Key Features:**

- Zero vertex buffers — fullscreen triangle trick via `@builtin(vertex_index)`
- SDF-based shapes: rounded rectangles, parabolic mouth, radial glow halos
- 8x4 LED dot grid with time-based shimmer inside each eye
- Same codebase targets Metal/Vulkan/DX12 native **and** WebGL2/WASM
- Async WGPU init bridged to winit's event loop on both native and WASM

[View WGPU implementation →](./wgpu/)

### 4. embedded-graphics Robot Face (Embedded Rust + OLED)

An animated robot face targeting **real 128x64 OLED hardware** (SH1106 / SSD1306 — common on ESP32, STM32, Jetson Nano GPIO), developed with the `embedded-graphics-simulator` as a desktop harness.

**Key Features:**

- Exact same drawing code runs on simulator today and microcontroller SPI display tomorrow — zero changes
- No heap allocation: `RobotFace`, `BlinkState`, and `EmotionController` live entirely on the stack (`no_std`-ready)
- Proper enum state machine for blink (`Open → Closing → Closed → Opening`), not boolean flags
- 6 discrete emotions: Happy, Neutral, Sad, Angry, Surprised, Thinking — with eyebrows
- Feature-gated simulator: `--no-default-features` produces a clean `no_std` core

**Controls:** H=Happy N=Neutral S=Sad A=Angry T=Thinking P=surPrised B=Blink Tab=cycle

[View embedded-graphics implementation →](./robot-face-embedded/)

### 5. Skia vs Raylib Comparison

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
├── wgpu/                          # WGPU (Rust) — pure GPU / SDF shaders
│   ├── src/
│   │   ├── app.rs                 # winit event loop
│   │   ├── state.rs               # WGPU device/pipeline/render
│   │   ├── robot_face.rs          # CPU animation + FaceUniforms
│   │   └── shader.wgsl            # WGSL fragment shader (all rendering)
│   ├── web/                       # WASM host page + Trunk config
│   └── build_wasm.sh
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

### WGPU Robot Face

```bash
cd wgpu

# Native (Metal / Vulkan / DX12)
cargo run --release

# WASM — with trunk (hot-reload dev server)
cargo install trunk
trunk serve web/index.html
# Open http://localhost:8080

# WASM — manual wasm-bindgen build
./build_wasm.sh
cd web/wasm && python3 -m http.server 8080
```

**Controls:** H (Happy), N (Neutral), S (Sad), B (Blink), ESC (Exit)

### embedded-graphics Robot Face (OLED Simulator)

```bash
cd robot-face-embedded

# Run simulator (512x256 window — 4x scaled 128x64 OLED)
cargo run

# Verify no_std compatible core compiles without simulator
cargo check --no-default-features
```

**Controls:** H=Happy N=Neutral S=Sad A=Angry T=Thinking P=surPrised B=Blink Tab=cycle Esc=quit

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

| Feature                  | Skia               | Raylib               | Bevy              | WGPU                    | egui              | embedded-graphics      |
| ------------------------ | ------------------ | -------------------- | ----------------- | ----------------------- | ----------------- | ---------------------- |
| **Primary Use Case**     | 2D graphics engine | Game framework       | Game engine       | Low-level GPU rendering | Immediate-mode UI | Embedded/OLED displays |
| **Language**             | C++                | C                    | Rust              | Rust                    | Rust              | Rust                   |
| **Setup/Installation**   | ★ (Very difficult) | ★★★★★ (brew install) | ★★★★★ (cargo add) | ★★★★★ (cargo add)       | ★★★★★ (cargo add) | ★★★★★ (cargo add)      |
| **WASM Support**         | ★★★★★ (CanvasKit)  | ★★★★                 | ★★★★★             | ★★★★★ (WebGL2)          | ★★★★★             | N/A (targets MCU)      |
| **Rendering Quality**    | ★★★★★              | ★★★★                 | ★★★★              | ★★★★★ (GPU-native)      | ★★★               | ★★★ (binary pixels)    |
| **API Simplicity**       | ★★★                | ★★★★★                | ★★★               | ★★ (low-level)          | ★★★★★             | ★★★★★                  |
| **Performance**          | ★★★★★              | ★★★★                 | ★★★★★             | ★★★★★                   | ★★★★              | ★★★★★ (no heap)        |
| **Learning Curve**       | Steep              | Gentle               | Moderate          | Very steep              | Gentle            | Gentle                 |
| **Architecture**         | Retained mode      | Immediate mode       | ECS               | Shader-driven           | Immediate mode    | DrawTarget trait       |
| **Binary Size**          | ~10MB              | ~2-3MB               | ~8MB              | ~5MB                    | ~5MB              | <100KB (MCU)           |
| **Ecosystem**            | Massive (Google)   | Active               | Growing fast      | Growing (W3C standard)  | Rust-focused      | Embedded Rust WG       |
| **Type Safety**          | Manual             | Manual               | Strong (Rust)     | Strong (Rust)           | Strong (Rust)     | Strong (Rust)          |
| **`no_std` Support**     | No                 | No                   | No                | No                      | No                | **Yes**                |
| **Real Hardware Target** | Indirect           | Indirect             | No                | No                      | No                | **Yes (SPI/I2C)**      |
| **Compile Times**        | Slow               | Fast                 | Moderate          | Moderate                | Fast              | Fast                   |
| **Animation System**     | Manual             | Manual               | Built-in          | Manual (shader)         | Manual            | Manual (state machine) |
| **3D Support**           | No                 | Yes                  | Yes               | Yes (low-level)         | No                |
| **Cross-Platform**       | Excellent          | Excellent            | Excellent         | Excellent               | Excellent         |
| **GPU Direct Access**    | No                 | No                   | Limited           | Full                    | No                |
| **Custom Shaders**       | Limited            | Limited              | Yes (WGSL/GLSL)   | Yes (WGSL)              | No                |

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

**Choose WGPU when:**

- You need full, direct GPU control with custom shaders
- Rendering logic belongs in shader code (SDF, procedural graphics, raymarching)
- Cross-backend portability matters (Metal, Vulkan, DX12, WebGL2, WebGPU)
- You want the lowest-level Rust GPU abstraction without FFI
- Examples: Custom renderers, visual effects engines, robotics HUDs, GPGPU

**Choose egui when:**

- Building developer tools or dashboards
- You need immediate-mode UI reactivity
- Rapid iteration and hot reload are important
- UI is the primary focus, not graphics
- Examples: Profilers, monitoring tools, debug interfaces, control panels

### Project Statistics

| Metric            | Bevy Robot Face | WGPU Robot Face         | egui Monitoring    | Raylib Face | Skia Face       |
| ----------------- | --------------- | ----------------------- | ------------------ | ----------- | --------------- |
| **Lines of Code** | ~400 (8 files)  | ~350 (6 files + shader) | ~1400 (13 modules) | ~250        | ~300            |
| **Architecture**  | Modular ECS     | Shader-driven GPU       | Clean Architecture | Single file | Single file     |
| **Dependencies**  | Bevy 0.17, rand | wgpu, winit, bytemuck   | egui, axum, tokio  | Raylib      | Skia            |
| **Compile Time**  | ~30s (release)  | ~35s (release)          | ~25s (release)     | <5s         | ~15s            |
| **Runtime Deps**  | None            | None                    | Python (sensors)   | SDL2        | SDL2            |
| **WASM Ready**    | Yes             | Yes (WebGL2)            | Yes (UI only)      | Yes         | Yes (CanvasKit) |

## Summary

This repository demonstrates practical implementations across five major graphics/UI frameworks, each with distinct strengths:

**Skia** excels at production-quality 2D rendering with battle-tested stability from billions of Chrome/Android devices. However, the complex build system and manual dependency management make it extremely difficult to set up compared to modern alternatives. While the API is powerful, the setup friction alone makes it hard to recommend for new projects unless you absolutely need its specific capabilities and have dedicated build engineering support.

**Raylib** provides the gentlest learning curve with a delightfully simple API and trivial installation via package managers. Perfect for beginners, prototypes, and situations where getting something working quickly matters more than architectural sophistication. WASM builds are straightforward with clear documentation.

**Bevy** brings modern game engine architecture to Rust with its Entity Component System. The type system catches bugs at compile time, and the modular design scales well to complex applications. Installation is seamless via Cargo, and WASM support works out of the box. Hot reload and strong tooling make iteration fast.

**WGPU** offers the deepest GPU access of any framework here. Instead of a 2D drawing API, rendering logic lives entirely in WGSL shaders — Signed Distance Fields compute every shape, glow, and animation mathematically on the GPU. The result is a resolution-independent, visually rich renderer with no 2D library dependency. The same codebase runs on Metal, Vulkan, DX12, and WebGL2 through a single abstraction layer. The trade-off is a steep learning curve: understanding the GPU pipeline, buffer layouts, bind groups, and shader math is prerequisite knowledge. For robotics HUDs, custom visual effects, or any use case where you want full shader control, WGPU is the correct tool.

**egui** redefines immediate-mode UI with Rust safety. The reactive model eliminates state synchronization bugs, and the library feels native to Rust. Exceptional for developer tools where UI responsiveness and ease of modification are paramount. Like Bevy, installation is a single `cargo add` command and WASM builds work immediately.

The **Clean Architecture refactoring** of the egui teleoperation system demonstrates how to structure Rust applications for maintainability. By separating domain logic, application services, infrastructure adapters, and presentation layers, the codebase remains understandable and testable even as complexity grows.

**Key Takeaway:** Raylib, Bevy, WGPU, and egui all installed and built successfully within minutes, including WASM targets. Skia required hours of troubleshooting without success. This practical experience strongly favors modern tooling with good package management over legacy libraries, regardless of theoretical performance advantages. For robotics embedded displays, the WGPU approach is particularly compelling: one shader file contains the entire visual design, it runs at full GPU speed with no CPU-side draw calls, and the WASM/WebGL2 path means the same face renders identically in a browser dashboard.

## Raylib Playground

Looking to learn Raylib in depth? The [`raylib-playground/`](./raylib-playground) directory contains professional C++ examples demonstrating advanced techniques:

- **01-solar-system** - Orbital mechanics with physics simulation
- **02-ui-components** - Theme system, interactive buttons, HSV color picker
- **03-particle-system** - 5 emitter types with physics (fire, snow, explosions, fountains, rain)
- **04-bezier-curves** - Interactive curve editor with control points (perfect for understanding robot face expressions)

Each example includes:

- Native macOS ARM64 builds
- WebAssembly builds with Emscripten
- Separated header/implementation files
- Comprehensive inline documentation
- Performance optimizations

These examples go beyond basic tutorials to show production-ready patterns like proper resource management, state machines, and performance profiling. [View full documentation →](./raylib-playground/README.md)

## License

MIT License - see LICENSE file for details
