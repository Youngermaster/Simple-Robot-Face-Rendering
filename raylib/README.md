# Robot Face - Raylib Implementation

This directory contains **three versions** of the robot face implementation using Raylib:

1. **Original Monolithic C** - Single-file implementation
2. **Modular C** - Separated into headers and implementation files
3. **Modern C++17** - Object-oriented design with RAII and modern features

## 📁 Project Structure

```
raylib/
├── include/
│   ├── robot_face_config.h    # Shared constants for C/C++
│   ├── robot_face.h            # C API (modular version)
│   └── robot_face.hpp          # C++ API (modern version)
├── src/
│   ├── robot_face_raylib.c     # Original monolithic version
│   ├── main.c                  # Entry point for modular C
│   ├── robot_face.c            # Core logic (modular C)
│   ├── robot_face_draw.c       # Drawing functions (modular C)
│   ├── main.cpp                # Entry point for modern C++
│   └── robot_face.cpp          # Implementation (modern C++)
├── CMakeLists.txt              # Build configuration
└── README.md                   # This file
```

## 🎯 Version Comparison

### 1. Original Monolithic C (`robot_face_raylib.c`)

**Characteristics:**
- ✅ Everything in one file (212 lines)
- ✅ Simple to understand and deploy
- ✅ No header dependencies
- ⚠️ Harder to maintain as complexity grows
- ⚠️ Not reusable across projects

**When to use:**
- Quick prototypes
- Learning graphics programming
- Single-file deployments

---

### 2. Modular C (`robot_face.c` + `robot_face_draw.c`)

**Characteristics:**
- ✅ Separated concerns (logic vs rendering)
- ✅ Reusable API via headers
- ✅ Easier to test individual components
- ✅ Configuration extracted to `robot_face_config.h`
- ✅ Pure C11 - compatible with embedded systems

**API Example:**
```c
#include "robot_face.h"

RobotFace face;
InitRobotFace(&face);

// Main loop
UpdateRobotFace(&face, deltaTime);
DrawRobotFace(&face, width, height);

// Control
SetEmotion(&face, 1.0f);  // Happy
TriggerBlink(&face);
```

**When to use:**
- Embedded systems (Jetson Nano, Raspberry Pi)
- Integration with C codebases
- Maximum portability

---

### 3. Modern C++17 (`robot_face.cpp`)

**Characteristics:**
- ✅ RAII design (automatic cleanup)
- ✅ Class-based encapsulation
- ✅ `constexpr` compile-time constants
- ✅ Strong typing with `enum class`
- ✅ Const correctness
- ✅ STL containers (std::string)
- ✅ `[[nodiscard]]` attributes for safety

**API Example:**
```cpp
#include "robot_face.hpp"

using namespace robotface;

RobotFace face(0.8f);  // RAII - no manual cleanup needed

// Main loop
face.update(deltaTime);
face.draw(width, height);

// Control
face.setEmotion(Emotion::Happy);
face.triggerBlink();

// Query state
if (face.isBlinking()) {
    std::cout << "Emotion: " << face.emotionName() << "\n";
}
```

**Modern Features:**
- **RAII**: No manual memory management
- **Namespaces**: `robotface::RobotFace`
- **Enum classes**: `Emotion::Happy` instead of magic numbers
- **Move semantics**: Efficient object transfers
- **Const methods**: Clear which methods modify state
- **Deleted copy**: Prevents accidental face duplication

**When to use:**
- Production robotics software
- Integration with ROS 2 (C++)
- Modern C++ codebases
- Maximum type safety

---

## 🔧 Building

### Prerequisites

```bash
# macOS (Homebrew)
brew install cmake raylib

# Ubuntu/Debian
sudo apt install cmake libraylib-dev

# Arch Linux
sudo pacman -S cmake raylib
```

### Build All Versions

```bash
cd raylib
mkdir build && cd build
cmake ..
make
```

This creates three executables:
- `robot_face_raylib` - Original monolithic C
- `robot_face_c` - Modular C
- `robot_face_cpp` - Modern C++

### Build Specific Version

```bash
# Only C++ version
cmake .. -DBUILD_ORIGINAL=OFF -DBUILD_C_MODULAR=OFF -DBUILD_CPP_MODERN=ON
make

# Only modular C
cmake .. -DBUILD_ORIGINAL=OFF -DBUILD_C_MODULAR=ON -DBUILD_CPP_MODERN=OFF
make
```

### Run

```bash
./robot_face_raylib    # Original
./robot_face_c         # Modular C
./robot_face_cpp       # Modern C++
```

---

## 🎮 Controls

All versions support the same controls:

| Key/Action | Effect |
|------------|--------|
| **H** | Set emotion to Happy (happiness = 1.0) |
| **N** | Set emotion to Neutral (happiness = 0.5) |
| **S** | Set emotion to Sad (happiness = 0.0) |
| **Mouse Click** | Trigger manual blink |
| **Hover over mouth** | Gradually increase happiness |
| **ESC** | Exit application |

---

## 📊 Feature Comparison

| Feature | Original C | Modular C | Modern C++ |
|---------|-----------|-----------|------------|
| **Lines of Code** | 212 | ~250 | ~280 |
| **Files** | 1 | 4 | 3 |
| **Reusability** | ❌ | ✅ | ✅ |
| **Type Safety** | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Maintainability** | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Embedded Friendly** | ✅ | ✅ | ⚠️ (needs C++ runtime) |
| **ROS 2 Ready** | ⚠️ | ⚠️ | ✅ |

---

## 🚀 Integration Examples

### Using Modular C in Your Project

```c
// your_project.c
#include "robot_face.h"

void my_render_loop() {
    RobotFace robot;
    InitRobotFace(&robot);

    while (running) {
        UpdateRobotFace(&robot, GetFrameTime());

        BeginDrawing();
        DrawRobotFace(&robot, 800, 600);
        EndDrawing();
    }
}
```

### Using Modern C++ in ROS 2 Node

```cpp
// robot_face_node.cpp
#include "robot_face.hpp"
#include <rclcpp/rclcpp.hpp>

class RobotFaceNode : public rclcpp::Node {
public:
    RobotFaceNode() : Node("robot_face"), face_(0.8f) {
        timer_ = create_wall_timer(
            16ms, [this]() { this->update(); }
        );
    }

private:
    void update() {
        face_.update(0.016f);
        face_.draw(800, 600);
    }

    robotface::RobotFace face_;
    rclcpp::TimerBase::SharedPtr timer_;
};
```

---

## 🧪 Testing

All three versions should produce **identical visual output**. You can verify by:

1. Running all three executables side-by-side
2. Comparing frame times (should be similar)
3. Testing same keyboard/mouse inputs

---

## 📖 Code Quality

### Modern C++ Features Used

| Feature | Purpose | Example |
|---------|---------|---------|
| `constexpr` | Compile-time constants | `static constexpr float BLINK_SPEED = 5.0f;` |
| `[[nodiscard]]` | Prevent ignoring return values | `[[nodiscard]] bool isBlinking() const;` |
| `noexcept` | Performance hint for compiler | `float happiness() const noexcept;` |
| `enum class` | Strong typing for enums | `enum class Emotion { Happy, Sad };` |
| RAII | Automatic resource cleanup | Constructor/destructor |
| Member initializers | Clean initialization | `float m_happiness = 0.8f;` |
| Move semantics | Efficient transfers | `RobotFace(RobotFace&&) = default;` |
| Deleted functions | Prevent unwanted copies | `RobotFace(const RobotFace&) = delete;` |

---

## 🎓 Learning Path

**Recommended order:**

1. **Start with Original C** - Understand the complete flow
2. **Study Modular C** - Learn separation of concerns
3. **Explore Modern C++** - See OOP and modern practices

Each version builds on the previous, making it easy to see how code evolves from prototype to production.

---

## 🔮 Next Steps

- [ ] Add unit tests for each version
- [ ] Create WASM builds (Emscripten)
- [ ] Benchmark performance differences
- [ ] Add more emotions (surprised, angry, etc.)
- [ ] Integrate with ROS 2
- [ ] Deploy to Jetson Nano

---

## 📝 License

Same as parent project.

## 🤝 Contributing

When adding features, update **all three versions** to maintain parity.

---

**Questions?** See parent [README.md](../README.md) for project overview.
