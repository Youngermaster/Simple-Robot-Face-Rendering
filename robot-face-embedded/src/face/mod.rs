//! Public API for the robot face.
//!
//! `RobotFace` owns the animation and emotion state.
//! `draw()` is generic over any `DrawTarget<Color = BinaryColor>`,
//! so the same struct works with the simulator, SH1106, SSD1306, etc.

pub mod animation;
pub mod config;
pub mod emotion;
pub mod renderer;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

pub use animation::BlinkState;
pub use emotion::{Emotion, EmotionController};

/// The robot face — owns all animation and emotion state.
///
/// No heap allocation: everything lives on the stack.
/// Compatible with `no_std` environments (no `Vec`, `Box`, or `String`).
pub struct RobotFace {
    pub emotion: EmotionController,
    pub blink: BlinkState,
}

impl RobotFace {
    /// Create a new robot face in the Neutral / eyes-open state.
    pub fn new() -> Self {
        Self {
            emotion: EmotionController::new(),
            blink: BlinkState::new(),
        }
    }

    /// Advance all animations by `dt` seconds. Call once per frame.
    pub fn update(&mut self, dt: f32) {
        self.blink.update(dt);
    }

    /// Change the displayed emotion.
    pub fn set_emotion(&mut self, e: Emotion) {
        self.emotion.set(e);
    }

    /// Immediately start a blink animation (if eyes are currently open).
    pub fn trigger_blink(&mut self) {
        self.blink.trigger();
    }

    /// Draw the face onto any compatible display target.
    pub fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        renderer::draw(self, target)
    }
}

impl Default for RobotFace {
    fn default() -> Self {
        Self::new()
    }
}
