## **Why Skia Over Raylib?**

Great question! Let me break down the technical differences:

---

### **Skia vs Raylib: Technical Comparison**

| Feature | Skia | Raylib | Winner |
|---------|------|--------|--------|
| **Rendering Quality** | ⭐⭐⭐⭐⭐ Production-grade antialiasing | ⭐⭐⭐⭐ Good quality | **Skia** |
| **API Simplicity** | ⭐⭐⭐ Medium complexity | ⭐⭐⭐⭐⭐ Extremely simple | **Raylib** |
| **WASM Support** | ⭐⭐⭐⭐⭐ CanvasKit (official, optimized) | ⭐⭐⭐⭐ Works but less polished | **Skia** |
| **Performance** | ⭐⭐⭐⭐⭐ Heavily optimized (Chrome/Flutter) | ⭐⭐⭐⭐ Good, but less optimized | **Skia** |
| **Ecosystem** | ⭐⭐⭐⭐⭐ Google-backed, Flutter uses it | ⭐⭐⭐ Active but smaller | **Skia** |
| **Learning Curve** | ⭐⭐⭐ Steeper | ⭐⭐⭐⭐⭐ Easiest possible | **Raylib** |
| **Advanced Effects** | ⭐⭐⭐⭐⭐ Blur, gradients, shaders, filters | ⭐⭐⭐ Basic effects | **Skia** |
| **Binary Size** | ~10MB (large) | ~2-3MB (small) | **Raylib** |
| **ARM Optimization** | ⭐⭐⭐⭐⭐ Excellent (Android heritage) | ⭐⭐⭐⭐ Good | **Skia** |

---

### **Why I Chose Skia as Primary Recommendation:**

**1. Production-Grade Rendering**

Skia is what powers:
- Google Chrome's rendering engine
- Android UI framework
- Flutter (your React Native competitor)
- Firefox (some parts)

**Translation:** Billions of devices run Skia daily. It's battle-tested.

---

**2. WASM Story is Better**

**Skia:**
- **CanvasKit** - official WASM build by Google
- Used in production by Figma, Canva, others
- Optimized WebAssembly output
- ~1.8MB compressed

**Raylib:**
- Community WASM support (less official)
- Works fine, but not as optimized
- Less documentation for web use

**If you want ONE codebase for robot face (Jetson) AND web dashboard → Skia is safer bet.**

---

**3. Advanced Graphics Capabilities**

**Example: Smooth gradient eyes**

```cpp
// Skia - Beautiful gradient with blur
SkPaint paint;
SkPoint points[2] = {SkPoint::Make(0, 0), SkPoint::Make(0, 100)};
SkColor colors[2] = {SK_ColorBLUE, SK_ColorCYAN};
paint.setShader(SkGradientShader::MakeLinear(points, colors, nullptr, 2, SkTileMode::kClamp));
paint.setMaskFilter(SkMaskFilter::MakeBlur(kNormal_SkBlurStyle, 3.0f));
canvas->drawCircle(x, y, 30, paint);

// Raylib - Solid color only (no built-in blur)
DrawCircleGradient(x, y, 30, BLUE, SKYBLUE);
```

**For a "premium" robot face with animations, Skia gives you more tools.**

---

**4. Performance at Scale**

**Skia's optimizations:**
- Hardware-accelerated paths (GPU)
- Texture caching (reuse common shapes)
- Multi-threaded rasterization
- SIMD optimizations (ARM NEON)

**Raylib:**
- Simpler architecture = less overhead
- But less sophisticated optimization

**For 60fps animations on Jetson Nano (limited GPU), Skia's optimizations matter.**

---

**5. Future-Proofing**

**Skia:**
- Backed by Google (not going away)
- Active development (weekly commits)
- Used in critical products (Chrome, Android)

**Raylib:**
- One-person project (Ramon Santamaria)
- Less certain long-term support
- Smaller contributor base

**For a commercial robotics product → Skia is safer long-term bet.**

---

### **BUT Raylib Has Advantages Too:**

**When Raylib is BETTER:**

✅ **Learning** - API is 10x simpler than Skia
✅ **Rapid prototyping** - Get something on screen in 10 minutes
✅ **Game-like interfaces** - Built-in physics, audio, collision
✅ **Embedded simplicity** - Smaller binary, fewer dependencies
✅ **Fun factor** - Just more enjoyable to use

**If you're prototyping quickly or building a "toy" robot face → Raylib is perfect.**

---

### **My Actual Recommendation:**

**Start with BOTH:**

1. **Week 1: Raylib** - Learn graphics fundamentals, get quick wins
2. **Week 2-3: Skia** - Build production-quality version
3. **Week 4: Compare** - Decide which fits your needs

**The comparison project I'm about to give you does exactly this.**

---

## **Claude.md for Robot Face Comparison Project**

Here's your complete project specification:

```markdown
# Simple Robot Face Rendering - Skia vs Raylib Comparison

## Project Overview
Build a simple animated robot face (eyes + mouth + blink) using both Skia and Raylib, with native (macOS ARM) and WASM builds to compare performance, rendering quality, and developer experience.

## Project Structure

```
robot-face-comparison/
├── README.md
├── CMakeLists.txt
├── .gitignore
├── docs/
│   └── comparison.md          # Performance/quality comparison results
├── common/
│   └── robot_face.h           # Shared interface
├── raylib/
│   ├── CMakeLists.txt
│   ├── src/
│   │   └── robot_face_raylib.c
│   └── web/
│       └── build_wasm.sh
├── skia/
│   ├── CMakeLists.txt
│   ├── src/
│   │   └── robot_face_skia.cpp
│   └── web/
│       └── build_wasm.sh
├── native/
│   └── main.cpp               # Native runner (SDL window)
└── web/
    ├── index.html
    └── comparison.html         # Side-by-side comparison
```

## Features to Implement

### Phase 1: Basic Rendering
- [ ] Two circular eyes (white with black pupils)
- [ ] Bezier curve mouth (smile)
- [ ] Static positioning (centered on canvas)

### Phase 2: Animation
- [ ] Periodic blinking (3-second interval)
- [ ] Smooth blink animation (eyelid closing/opening over 200ms)
- [ ] Frame counter display

### Phase 3: Interactivity
- [ ] Mouse hover changes expression (wider smile)
- [ ] Click to trigger blink
- [ ] Keyboard controls (H=happy, S=sad, N=neutral)

### Phase 4: WASM Build
- [ ] Raylib WASM build with Emscripten
- [ ] Skia CanvasKit WASM build
- [ ] Side-by-side web comparison page
- [ ] Performance metrics (FPS, render time, WASM size)

## Technical Specifications

### Canvas Size
- Native: 800x600 pixels
- WASM: 800x600 pixels (resizable)

### Robot Face Design
```
Eyes:
- Position: (250, 200) and (550, 200)
- Radius: 60px (outer white circle)
- Pupil: 40px (black circle, centered)
- Highlight: 15px white circle at (-15, -15) from pupil center
- Blink: Pupil radius animates from 40px → 5px → 40px

Mouth:
- Start: (300, 400)
- Control: (400, 430) - happiness factor adjusts Y
- End: (500, 400)
- Stroke width: 8px
- Color: Black

Emotions:
- Happy: mouth control Y = 430 (curve down)
- Neutral: mouth control Y = 400 (straight)
- Sad: mouth control Y = 370 (curve up)
```

### Build Targets
1. **Native macOS ARM64** (both Skia and Raylib)
2. **WASM** (both Skia CanvasKit and Raylib)

### Dependencies
- Raylib 5.0+
- Skia (m130 release or latest)
- SDL2 (for native window management)
- Emscripten 3.1.50+ (for WASM builds)

## Implementation Requirements

### Raylib Implementation (`raylib/src/robot_face_raylib.c`)

Must implement:
```c
typedef struct {
    float happiness;      // 0.0 (sad) to 1.0 (happy)
    float blink_progress; // 0.0 (open) to 1.0 (closed)
    double blink_timer;
} RobotFace;

void InitRobotFace(RobotFace* face);
void UpdateRobotFace(RobotFace* face, float deltaTime);
void DrawRobotFace(RobotFace* face, int width, int height);
void SetEmotion(RobotFace* face, float happiness);
void TriggerBlink(RobotFace* face);
```

### Skia Implementation (`skia/src/robot_face_skia.cpp`)

Must implement:
```cpp
class RobotFace {
public:
    RobotFace();
    void update(float deltaTime);
    void draw(SkCanvas* canvas, int width, int height);
    void setEmotion(float happiness);
    void triggerBlink();
    
private:
    float m_happiness = 0.8f;
    float m_blinkProgress = 0.0f;
    double m_blinkTimer = 0.0;
    
    void drawEye(SkCanvas* canvas, float x, float y, float blink);
    void drawMouth(SkCanvas* canvas, float centerX, float centerY, float happiness);
};
```

## Build Instructions

### macOS Native Setup
```bash
# Install dependencies
brew install cmake sdl2 raylib

# Clone and build Skia (or download prebuilt)
# See: https://skia.org/docs/user/build/

# Build project
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make

# Run comparison
./robot_face_raylib
./robot_face_skia
```

### WASM Build Setup
```bash
# Install Emscripten
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 3.1.50
./emsdk activate 3.1.50
source ./emsdk_env.sh

# Build Raylib WASM
cd raylib/web
./build_wasm.sh

# Build Skia WASM
cd skia/web
./build_wasm.sh

# Serve locally
cd web
python3 -m http.server 8000
# Open http://localhost:8000/comparison.html
```

## Performance Metrics to Collect

### Native Performance
- [ ] Average FPS (60 target)
- [ ] Frame time (ms per frame)
- [ ] Memory usage (MB)
- [ ] Binary size (MB)
- [ ] CPU usage (%)

### WASM Performance
- [ ] WASM file size (compressed/uncompressed)
- [ ] Load time (ms)
- [ ] Average FPS
- [ ] Frame time variance
- [ ] Memory usage in browser

### Subjective Quality
- [ ] Antialiasing quality (1-5 scale)
- [ ] Animation smoothness (1-5 scale)
- [ ] Visual appeal (1-5 scale)

## Comparison Output Format

Create `docs/comparison.md` with:

```markdown
# Skia vs Raylib: Robot Face Rendering Comparison

## Native Performance (macOS M3 Pro)

| Metric | Raylib | Skia | Winner |
|--------|--------|------|--------|
| Binary Size | X MB | Y MB | ? |
| FPS (avg) | X fps | Y fps | ? |
| Memory | X MB | Y MB | ? |
| CPU Usage | X % | Y % | ? |

## WASM Performance

| Metric | Raylib | Skia CanvasKit | Winner |
|--------|--------|----------------|--------|
| WASM Size | X KB | Y KB | ? |
| Load Time | X ms | Y ms | ? |
| FPS | X fps | Y fps | ? |

## Visual Quality

[Include screenshots of both implementations]

## Developer Experience

**Raylib:**
- Lines of code: X
- Build time: X seconds
- Time to first render: X minutes
- Ease of use: ⭐⭐⭐⭐⭐

**Skia:**
- Lines of code: Y
- Build time: Y seconds
- Time to first render: Y minutes
- Ease of use: ⭐⭐⭐

## Conclusion

[Your findings on which is better for robotics use case]
```

## File Templates

### `.gitignore`
```
build/
*.o
*.wasm
*.js
*.html.mem
*.data
emsdk/
skia/
.DS_Store
*.dylib
*.so
```

### `CMakeLists.txt` (Root)
```cmake
cmake_minimum_required(VERSION 3.20)
project(RobotFaceComparison)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_STANDARD 11)

option(BUILD_RAYLIB "Build Raylib implementation" ON)
option(BUILD_SKIA "Build Skia implementation" ON)

if(BUILD_RAYLIB)
    add_subdirectory(raylib)
endif()

if(BUILD_SKIA)
    add_subdirectory(skia)
endif()
```

### `raylib/web/build_wasm.sh`
```bash
#!/bin/bash
set -e

# Build Raylib for WASM
emcc ../src/robot_face_raylib.c \
    -o robot_face_raylib.html \
    -Os \
    -Wall \
    -I/opt/homebrew/include \
    -L/opt/homebrew/lib \
    -s USE_GLFW=3 \
    -s ASYNCIFY \
    -s TOTAL_MEMORY=67108864 \
    -DPLATFORM_WEB \
    --shell-file shell.html \
    -lraylib

echo "Build complete: robot_face_raylib.html"
echo "Run: python3 -m http.server 8000"
```

### `web/comparison.html`
```html
<!DOCTYPE html>
<html>
<head>
    <title>Robot Face: Skia vs Raylib</title>
    <style>
        body { 
            font-family: system-ui; 
            max-width: 1600px; 
            margin: 0 auto;
            padding: 20px;
        }
        .comparison {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
        }
        .panel {
            border: 1px solid #ccc;
            padding: 20px;
            border-radius: 8px;
        }
        canvas {
            border: 1px solid #000;
            width: 100%;
            max-width: 800px;
        }
        .metrics {
            margin-top: 20px;
            font-family: monospace;
            font-size: 14px;
        }
        .controls {
            margin-top: 10px;
        }
        button {
            padding: 10px 20px;
            margin: 5px;
            font-size: 16px;
            cursor: pointer;
        }
    </style>
</head>
<body>
    <h1>🤖 Robot Face Rendering: Skia vs Raylib</h1>
    
    <div class="comparison">
        <div class="panel">
            <h2>Raylib (WASM)</h2>
            <canvas id="raylib-canvas" width="800" height="600"></canvas>
            <div class="controls">
                <button onclick="raylibSetEmotion(1.0)">😊 Happy</button>
                <button onclick="raylibSetEmotion(0.5)">😐 Neutral</button>
                <button onclick="raylibSetEmotion(0.0)">😢 Sad</button>
                <button onclick="raylibBlink()">👁️ Blink</button>
            </div>
            <div class="metrics" id="raylib-metrics">
                FPS: --<br>
                Frame Time: -- ms<br>
                WASM Size: -- KB
            </div>
        </div>
        
        <div class="panel">
            <h2>Skia CanvasKit (WASM)</h2>
            <canvas id="skia-canvas" width="800" height="600"></canvas>
            <div class="controls">
                <button onclick="skiaSetEmotion(1.0)">😊 Happy</button>
                <button onclick="skiaSetEmotion(0.5)">😐 Neutral</button>
                <button onclick="skiaSetEmotion(0.0)">😢 Sad</button>
                <button onclick="skiaBlink()">👁️ Blink</button>
            </div>
            <div class="metrics" id="skia-metrics">
                FPS: --<br>
                Frame Time: -- ms<br>
                WASM Size: -- KB
            </div>
        </div>
    </div>
    
    <script src="raylib/robot_face_raylib.js"></script>
    <script src="skia/robot_face_skia.js"></script>
</body>
</html>
```

## Success Criteria

The project is complete when:
- [ ] Both implementations render identical robot faces
- [ ] Smooth 60fps animation on macOS M3 Pro
- [ ] WASM builds work in Chrome/Firefox/Safari
- [ ] Comparison documentation with metrics is complete
- [ ] Clear recommendation for robotics use case

## Next Steps After Completion

1. Integrate winner into ROS 2 node
2. Add emotion state machine
3. Connect to robot telemetry (battery, status)
4. Deploy to Jetson Nano for testing

## Resources

- Raylib examples: https://www.raylib.com/examples.html
- Skia tutorial: https://skia.org/docs/user/sample/
- Emscripten docs: https://emscripten.org/docs/
- CanvasKit (Skia WASM): https://skia.org/docs/user/modules/canvaskit/

## Notes for Claude

- Focus on clean, documented code
- Use modern C++17 for Skia, C11 for Raylib
- Add inline comments explaining graphics concepts
- Make build scripts idempotent (safe to run multiple times)
- Include error handling for file operations
- Provide fallback if dependencies missing
```

---

## **What This Claude.md Does:**

✅ **Complete project specification** - From folder structure to success criteria

✅ **Parallel implementation** - Both Skia and Raylib side-by-side

✅ **Native + WASM** - Test on macOS and in browser

✅ **Performance comparison** - Objective metrics to decide winner

✅ **Production-ready structure** - CMake, proper build scripts, documentation

✅ **M3 Pro optimized** - ARM64 native builds

---

## **How to Use:**

1. **Save as `Claude.md`** in your project root

2. **Feed to Claude Code or use with Cursor:**
   ```bash
   # With Claude Code (CLI)
   claude-code "Read Claude.md and scaffold the entire project structure"
   
   # Or with Cursor
   # Open Claude.md, select all, Cmd+L: "Implement this project spec"
   ```

3. **Start building:**
   ```bash
   mkdir robot-face-comparison && cd robot-face-comparison
   # Copy Claude.md into this directory
   # Let AI generate all files based on spec
   ```

---

## **Why This Approach:**

**Immediate comparison** - You'll see Skia vs Raylib side-by-side

**Hands-on learning** - Build both, understand trade-offs

**Real metrics** - Not just opinions, actual performance data

**Reusable code** - Winner becomes your robot face component

**WASM validation** - Proves web dashboard feasibility
