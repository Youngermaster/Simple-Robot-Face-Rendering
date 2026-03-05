# Robot Face — WGPU (Rust)

A pure-GPU animated robot face built with [wgpu](https://wgpu.rs/) and [winit](https://github.com/rust-windowing/winit). All rendering is done in a single WGSL fragment shader using **Signed Distance Fields (SDF)** — no 2D library, no vertex buffers beyond a fullscreen triangle.

Runs natively on macOS / Linux / Windows and in the browser via **WebGL2 + WebAssembly**.

---

## Visual Design

| Element | Description |
|---------|-------------|
| Background | Near-black rounded rectangle with a subtle cyan border glow |
| Eyes | Landscape rounded rectangles with an 8×4 LED dot grid and animated shimmer |
| Glow | Soft SDF-based halo around each eye |
| Mouth | Parabolic curve SDF — curves up (happy), flat (neutral), curves down (sad) |
| Blink | Y-axis scale of the eye SDF smoothly approaches zero and back |
| Vignette | Radial darkening toward frame corners |

---

## Architecture

```
wgpu/
├── Cargo.toml            Dependencies: wgpu 22, winit 0.30, bytemuck, WASM deps
├── src/
│   ├── main.rs           Native binary entry — env_logger init, then app::run()
│   ├── lib.rs            WASM library entry — #[wasm_bindgen(start)] → app::run()
│   ├── app.rs            winit ApplicationHandler event loop + keyboard input
│   ├── state.rs          WGPU boilerplate: device, queue, pipeline, render loop
│   ├── robot_face.rs     CPU animation state + FaceUniforms (GPU-facing struct)
│   └── shader.wgsl       All visual logic in WGSL — SDF math, glow, LED grid
├── web/
│   ├── index.html        Dark cyberpunk host page (dark bg, cyan accent)
│   └── Trunk.toml        trunk build configuration
└── build_wasm.sh         Manual wasm-bindgen build script
```

### Data Flow

```
winit events
    │
    ▼
app.rs  ──── keyboard ──► robot_face.rs  (happiness, blink_progress)
    │                          │
    │                    FaceUniforms { time, blink_progress, happiness, aspect }
    │                          │
    ▼                          ▼
state.rs ──── queue.write_buffer() ──── WGPU uniform buffer
    │
    ▼
shader.wgsl  (fragment shader reads uniforms, outputs pixel color)
```

---

## Shader: How It Works

The vertex shader emits a **fullscreen triangle** using the `@builtin(vertex_index)` trick — no vertex buffer needed:

```wgsl
// Three vertices cover all clip space: (-1,-1), (3,-1), (-1,3)
var positions = array<vec2<f32>, 3>(
    vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0),
);
```

The fragment shader runs for every pixel and composes layers via SDF:

```
1. sdf_rounded_box(uv, screen_half, r)       → background rect
2. sdf_rounded_box(p_eye, eye_half, r)        → left eye (Y scaled by blink)
   led_grid(local_uv, 4.0)                   → LED dot pattern inside eye
   glow(d_eye, 0.12)                          → soft halo outside eye
3. same for right eye
4. parabolic mouth SDF controlled by happiness uniform
5. radial vignette
```

**Key SDF helper** (Inigo Quilez formulation):
```wgsl
fn sdf_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2(r, r);
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - r;
}
```

---

## Uniform Buffer

`FaceUniforms` is shared between Rust (CPU) and WGSL (GPU) via `bytemuck`:

```rust
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct FaceUniforms {
    pub time:           f32,   // elapsed seconds (shader animations)
    pub blink_progress: f32,   // 0.0 = open, 1.0 = fully closed
    pub happiness:      f32,   // 0.0 = sad, 0.5 = neutral, 1.0 = happy
    pub aspect:         f32,   // viewport width / height
}
```

4 × f32 = 16 bytes, naturally aligned — no padding required for std140.

---

## Building

### Native (macOS / Linux / Windows)

```bash
cd wgpu
cargo run --release
```

Requires a Vulkan / Metal / DX12 capable GPU (any modern machine).

### WASM — with trunk (recommended)

```bash
cargo install trunk                    # one-time
cd wgpu
trunk serve web/index.html             # hot-reload dev server at http://localhost:8080
```

### WASM — with wasm-bindgen manually

```bash
cd wgpu
./build_wasm.sh                        # builds + generates JS bindings
cd web/wasm
python3 -m http.server 8080
# Open http://localhost:8080
```

The WASM build uses `Backends::GL` (WebGL2) and `Limits::downlevel_webgl2_defaults()` so it works in all modern browsers without WebGPU.

---

## Controls

| Key | Action |
|-----|--------|
| `H` | Happy expression (happiness = 1.0) |
| `N` | Neutral expression (happiness = 0.5) |
| `S` | Sad expression (happiness = 0.0) |
| `B` | Trigger manual blink |
| `Esc` | Exit |

Auto-blink fires every **3 seconds** with a 250 ms sine-smoothed animation.

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `wgpu` | 22 | WebGPU/Metal/Vulkan/DX12 abstraction |
| `winit` | 0.30 | Cross-platform window + event loop |
| `bytemuck` | 1 | Safe cast of structs to `&[u8]` for GPU upload |
| `pollster` | 0.3 | Block on async init on native |
| `env_logger` | 0.11 | Native logging |
| `wasm-bindgen` | 0.2 | WASM ↔ JS bridge |
| `wasm-bindgen-futures` | 0.4 | `spawn_local` for WASM async init |
| `console_log` / `console_error_panic_hook` | — | Browser console logging & panic traces |

---

## Technical Notes

**WASM async init**: `request_adapter` and `request_device` are `async`. On native, `pollster::block_on()` blocks the main thread. On WASM (no threads), `wasm_bindgen_futures::spawn_local` runs the init and sends the resulting `State` back through an `EventLoopProxy<AppEvent>`, received in `user_event()`.

**`Arc<Window>`**: `wgpu::Surface<'static>` requires the window to outlive the surface. `Arc<Window>` satisfies this without unsafe lifetime annotation.

**WGSL `select()`**: argument order is `select(false_val, true_val, condition)` — opposite of GLSL ternary `condition ? true_val : false_val`.

**No vertex buffer**: The fullscreen-triangle approach avoids allocating, uploading, and binding a vertex buffer. The three positions are computed entirely from `@builtin(vertex_index)` in the vertex shader.
