# robot-face-embedded

Animated robot face renderer for **128x64 OLED displays** (SH1106 / SSD1306),
built with [`embedded-graphics`](https://github.com/embedded-graphics/embedded-graphics).

The same drawing code runs on the desktop simulator today and on a microcontroller
SPI display driver tomorrow — zero changes required.

## Prerequisites

The simulator depends on **SDL2**. Install it before running `cargo run`.

### macOS (Homebrew)

```bash
brew install sdl2
```

Homebrew on Apple Silicon installs to `/opt/homebrew` instead of `/usr/local`, so
the linker needs a hint. The `.cargo/config.toml` in this directory already handles
this — no extra steps required.

If you installed SDL2 to a non-standard path, update `.cargo/config.toml`:

```toml
[build]
rustflags = ["-L", "/your/custom/path/to/sdl2/lib"]
```

### Linux (apt)

```bash
sudo apt install libsdl2-dev
```

### Windows

Download the SDL2 development libraries from https://www.libsdl.org/ and follow
the [sdl2 crate setup guide](https://github.com/Rust-SDL2/rust-sdl2#windows).

---

## Quick start (simulator)

```bash
cd robot-face-embedded
cargo run
```

A 512x256 window opens (4x scaled 128x64 OLED with blue pixel theme).

### Keyboard controls

| Key     | Action             |
| ------- | ------------------ |
| H       | Happy              |
| N       | Neutral            |
| S       | Sad                |
| A       | Angry              |
| T       | Thinking           |
| P       | surPrised          |
| B       | Trigger blink      |
| Tab     | Cycle all emotions |
| Esc / Q | Quit               |

Auto-blink fires every 3 seconds.

## Architecture

```bash
src/
├── main.rs              # Simulator entry point (feature-gated)
└── face/
    ├── mod.rs           # RobotFace struct + public API
    ├── config.rs        # All OLED layout constants (128x64)
    ├── emotion.rs       # Emotion enum + EmotionController
    ├── animation.rs     # BlinkState enum (proper state machine)
    └── renderer.rs      # draw<D: DrawTarget<Color=BinaryColor>>()
```

### Key design principles

- **No heap allocation** — `RobotFace`, `BlinkState`, and `EmotionController` live
  entirely on the stack. No `Vec`, `Box`, or `String`. `no_std`-ready.
- **Enum state machine** for blink animation — impossible states are unrepresentable.
  You cannot have `closing = true` AND `opening = true` at the same time.
- **`DrawTarget` abstraction** — `renderer.rs` never imports the simulator. It
  compiles equally for AVR, ARM Cortex-M, RISC-V, or macOS.
- **Config centralisation** — every coordinate and radius is a named constant in
  `config.rs`. Porting to a 128x32 display means editing one file.

## Deploying to real hardware

1. Add your display driver crate (e.g. `ssd1306`, `sh1106`) to `Cargo.toml`.
2. Add your HAL crate (e.g. `esp-idf-hal`, `stm32f4xx-hal`).
3. Build without the simulator feature:

```bash
cargo build --release --no-default-features
```

4. In your `#[entry]` function, create the display and call:

```rust
let mut face = RobotFace::new();

loop {
    let dt = /* measure elapsed seconds */;
    face.update(dt);
    face.draw(&mut display).unwrap();
}
```

The `face` module compiles identically — only the display target changes.

## Supported emotions

| Emotion   | Eyes                 | Mouth           | Brows               |
| --------- | -------------------- | --------------- | ------------------- |
| Happy     | Normal               | Smile arc       | —                   |
| Neutral   | Normal               | Straight line   | —                   |
| Sad       | Normal               | Frown arc       | `\ /` raised inner  |
| Angry     | Normal               | Scowl lines     | `/ \` lowered inner |
| Surprised | Wide (larger radius) | Small 'O'       | —                   |
| Thinking  | Right eye squints    | Diagonal offset | —                   |

## Display layout

```
0                   64                  127
┌───────────────────────────────────────┐  0
│         ╭────╮           ╭────╮       │
│         │ ●  │           │ ●  │       │  22
│         ╰────╯           ╰────╯       │
│                                       │
│              ╭──────╮                 │  48
└───────────────────────────────────────┘  63
```
