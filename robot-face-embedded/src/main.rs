//! Simulator entry point for `robot-face-embedded`.
//!
//! Run with: `cargo run`
//!
//! Controls:
//!   H = Happy      N = Neutral    S = Sad
//!   A = Angry      T = Thinking   P = surPrised
//!   B = Blink      Tab = cycle emotions
//!   Esc / Q = quit

mod face;

#[cfg(feature = "simulator")]
use face::{Emotion, RobotFace};

#[cfg(feature = "simulator")]
fn main() -> Result<(), core::convert::Infallible> {
    use embedded_graphics::pixelcolor::BinaryColor;
    use embedded_graphics::prelude::*;
    use embedded_graphics_simulator::{
        sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
        Window,
    };

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .scale(4)
        .pixel_spacing(1)
        .build();

    let mut window = Window::new(
        "Robot Face — embedded-graphics (128x64 OLED sim)",
        &output_settings,
    );

    let mut face = RobotFace::new();
    let mut last = std::time::Instant::now();

    'main: loop {
        let now = std::time::Instant::now();
        let dt = now.duration_since(last).as_secs_f32();
        last = now;

        face.update(dt);
        face.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'main,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::H => {
                        face.set_emotion(Emotion::Happy);
                        println!("Emotion: Happy");
                    }
                    Keycode::N => {
                        face.set_emotion(Emotion::Neutral);
                        println!("Emotion: Neutral");
                    }
                    Keycode::S => {
                        face.set_emotion(Emotion::Sad);
                        println!("Emotion: Sad");
                    }
                    Keycode::A => {
                        face.set_emotion(Emotion::Angry);
                        println!("Emotion: Angry");
                    }
                    Keycode::T => {
                        face.set_emotion(Emotion::Thinking);
                        println!("Emotion: Thinking");
                    }
                    Keycode::P => {
                        face.set_emotion(Emotion::Surprised);
                        println!("Emotion: Surprised");
                    }
                    Keycode::B => {
                        face.trigger_blink();
                        println!("Blink triggered");
                    }
                    Keycode::Tab => {
                        let next = face.emotion.current.next();
                        face.set_emotion(next);
                        println!("Emotion: {}", face.emotion.current.label());
                    }
                    Keycode::Escape | Keycode::Q => break 'main,
                    _ => {}
                },
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 fps
    }

    Ok(())
}

#[cfg(not(feature = "simulator"))]
fn main() {
    // On real hardware there is no simulator entry point.
    // Link against your HAL crate and call `face.draw(&mut display)` from
    // your `#[entry]` function instead.
    //
    // `cargo check --no-default-features` confirms the face logic compiles
    // for no_std targets without the simulator dependency.
}
