# 🤖 Robot Face Rendering with Bevy

A real-time robot face renderer built with the Bevy game engine, showcasing proper ECS (Entity Component System) architecture and cross-platform support including WebAssembly.

![Bevy](https://img.shields.io/badge/Bevy-0.17.3-purple?style=flat-square&logo=rust)
![Rust](https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust)
![WASM](https://img.shields.io/badge/WASM-Ready-blue?style=flat-square)

## ✨ Features

- **🎭 Emotional Expressions**: Happy, Neutral, and Sad states with smooth transitions
- **👁️ Animated Blinking**: Automatic blinking every 3 seconds with manual trigger
- **👀 Pupil Movement**: Random "look around" behavior for lifelike appearance
- **⌨️ Interactive Controls**: Keyboard-driven emotion and gesture controls
- **🌐 WASM Support**: Runs natively and in web browsers
- **🦀 Pure Rust**: Built with modern Rust practices following Bevy's ECS paradigm

## 🏗️ ECS Architecture

This project follows Bevy's Entity Component System pattern:

### Components (Data)
- `RobotFace`: Marker for the face container
- `Eye`: Identifies left/right eyes
- `EyeWhite`: Outer white part of the eye
- `Pupil`: Black pupil with look offset
- `Highlight`: White highlight on pupil
- `Mouth`: Mouth component
- `Blinking`: Blink animation state

### Resources (Global State)
- `Emotion`: Current happiness level (0.0 = sad, 1.0 = happy)
- `AutoBlinkTimer`: Timer for automatic blinking

### Systems (Logic)
- `setup_camera`: Initialize 2D camera
- `setup_robot_face`: Spawn all face entities
- `setup_ui`: Create UI text
- `keyboard_input_system`: Handle user input
- `auto_blink_system`: Trigger periodic blinks
- `blink_animation_system`: Animate eye closing/opening
- `emotion_update_system`: Update mouth based on emotion
- `pupil_look_system`: Smooth pupil movement
- `update_ui_system`: Update UI with current state

## 🎮 Controls

| Key | Action |
|-----|--------|
| <kbd>H</kbd> | Set emotion to Happy 😊 |
| <kbd>N</kbd> | Set emotion to Neutral 😐 |
| <kbd>S</kbd> | Set emotion to Sad 😢 |
| <kbd>B</kbd> | Trigger manual blink 👁️ |
| <kbd>L</kbd> | Look around (random pupil movement) 👀 |
| <kbd>ESC</kbd> | Quit application |

## 🚀 Quick Start

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### Running Natively

```bash
# Clone the repository
cd robot-face-rendering-bevy

# Run in development mode
cargo run

# Run in release mode (better performance)
cargo run --release
```

## 🌐 Building for WASM

### Prerequisites

```bash
# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Optional: Install a simple HTTP server
cargo install basic-http-server
# OR use Python's built-in server (no installation needed)
```

### Build Steps

```bash
# Make the build script executable
chmod +x build_wasm.sh

# Run the build script
./build_wasm.sh
```

### Serving Locally

```bash
# Navigate to the wasm directory
cd wasm

# Option 1: Using Python
python3 -m http.server 8080

# Option 2: Using basic-http-server (if installed)
basic-http-server .

# Open your browser to:
# http://localhost:8080
```

## 📁 Project Structure

```
robot-face-rendering-bevy/
├── Cargo.toml              # Project dependencies and configuration
├── README.md               # This file
├── build_wasm.sh           # WASM build script
├── index.html              # HTML wrapper for WASM
├── src/
│   └── main.rs             # Main application code
└── wasm/                   # Generated WASM build (created after build)
    ├── index.html
    ├── robot-face-rendering-bevy.js
    └── robot-face-rendering-bevy_bg.wasm
```

## 🎨 Design Details

### Face Layout

```
        👁️        👁️
     (Left Eye) (Right Eye)
     @ (-100, 80) @ (100, 80)

         _____
        (     )  <- Mouth
         -----
       @ (0, -100)
```

### Dimensions
- **Eye Radius**: 60px (white outer circle)
- **Pupil Radius**: 40px (black circle)
- **Highlight Radius**: 15px (white dot)
- **Eye Spacing**: 200px apart
- **Mouth Width**: 200px

### Animation Details
- **Blink Speed**: 8.0 units/second
- **Auto-blink Interval**: Every 3 seconds
- **Pupil Movement**: Smooth interpolation (lerp factor: 0.1)

## 🛠️ Development

### Code Organization

The code follows Rust and Bevy best practices:

1. **Separation of Concerns**: Components contain only data, systems contain only logic
2. **Type Safety**: Strong typing with custom component types
3. **Query Efficiency**: Systems query only the components they need
4. **Change Detection**: Using Bevy's change detection (`is_changed()`) for optimization
5. **Child Entities**: Proper parent-child relationships for eyes and pupils

### Adding New Features

To add a new gesture or animation:

1. **Add a Component** (if needed):
   ```rust
   #[derive(Component)]
   struct NewGesture {
       // Your data here
   }
   ```

2. **Create a System**:
   ```rust
   fn new_gesture_system(query: Query<&mut NewGesture>) {
       // Your logic here
   }
   ```

3. **Register the System**:
   ```rust
   .add_systems(Update, new_gesture_system)
   ```

## 🎯 Use Cases

This project is ideal for:

- 🤖 **Robotics HMI**: Face rendering for physical robots (Jetson Nano, Raspberry Pi)
- 🎓 **Learning Bevy**: Example of proper ECS architecture
- 🎮 **Game Dev**: Character emotion system template
- 🌐 **Web Apps**: Interactive avatar or chatbot face
- 📱 **Embedded Systems**: Lightweight UI for resource-constrained devices

## 🔧 Troubleshooting

### Native Build Issues

**Problem**: "Cannot find bevy"
```bash
# Solution: Clean and rebuild
cargo clean
cargo build
```

### WASM Build Issues

**Problem**: "wasm-bindgen version mismatch"
```bash
# Solution: Update wasm-bindgen-cli
cargo install wasm-bindgen-cli --force
```

**Problem**: WASM file too large
```bash
# Solution: Use the wasm-release profile
cargo build --profile wasm-release --target wasm32-unknown-unknown
```

**Problem**: Browser shows blank screen
- Check browser console for errors
- Ensure you're serving over HTTP (not file://)
- Try a different browser (Chrome/Firefox recommended)

## 📊 Performance

### Native Performance (M3 Pro)
- **FPS**: 60 (vsync limited)
- **Frame Time**: ~1-2ms
- **Memory**: ~50MB

### WASM Performance
- **WASM Size**: ~3-4MB (uncompressed)
- **Load Time**: ~1-2 seconds
- **FPS**: 60 (browser dependent)

## 🚧 Future Enhancements

- [ ] Mouse tracking for pupil movement
- [ ] More complex mouth shapes (talking animation)
- [ ] Eye gaze smoothing with damping
- [ ] Different face styles (customizable colors, shapes)
- [ ] Sound effects for blinks and emotions
- [ ] Integration with ROS 2 for robotics control
- [ ] Touch/mobile input support
- [ ] Export as library for reuse in other projects

## 📚 Resources

- [Bevy Engine](https://bevyengine.org/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly](https://webassembly.org/)

## 📄 License

This project is open source and available for educational and commercial use.

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## 🙏 Acknowledgments

- Built with [Bevy Engine](https://bevyengine.org/)
- Inspired by robotics and HMI design principles
- Following Rust and Bevy best practices

---

**Made with 🦀 Rust and ❤️ for robotics**
