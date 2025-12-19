//! Resource definitions for global state
//!
//! Resources represent global state that isn't tied to specific entities.

use bevy::prelude::*;

/// Current emotion state
#[derive(Resource)]
pub struct Emotion {
    /// 0.0 = sad, 0.5 = neutral, 1.0 = happy
    pub happiness: f32,
}

impl Default for Emotion {
    fn default() -> Self {
        Self { happiness: 0.8 }
    }
}

/// Timer for automatic blinking
#[derive(Resource)]
pub struct AutoBlinkTimer {
    pub timer: Timer,
}

impl Default for AutoBlinkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}
