use bytemuck::{Pod, Zeroable};

/// GPU-facing uniform data (maps 1:1 to WGSL struct FaceUniforms)
/// Naturally 16-byte aligned (4 x f32) — no padding needed.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FaceUniforms {
    pub time: f32,
    pub blink_progress: f32, // 0.0 = open, 1.0 = closed
    pub happiness: f32,      // 0.0 = sad, 0.5 = neutral, 1.0 = happy
    pub aspect: f32,         // viewport width / height
}

/// CPU-side animation state
pub struct RobotFace {
    pub happiness: f32,
    pub blink_progress: f32,
    pub blink_timer: f32,
    is_blinking: bool,
    blink_phase: f32, // 0..1 within current blink
}

impl RobotFace {
    const BLINK_INTERVAL: f32 = 3.0; // seconds between auto-blinks
    const BLINK_DURATION: f32 = 0.25; // seconds per blink cycle

    pub fn new() -> Self {
        Self {
            happiness: 0.8,
            blink_progress: 0.0,
            blink_timer: 0.0,
            is_blinking: false,
            blink_phase: 0.0,
        }
    }

    pub fn update(&mut self, delta_secs: f32) {
        self.blink_timer += delta_secs;

        if self.is_blinking {
            self.blink_phase += delta_secs / Self::BLINK_DURATION;
            if self.blink_phase >= 1.0 {
                self.is_blinking = false;
                self.blink_phase = 0.0;
                self.blink_progress = 0.0;
                self.blink_timer = 0.0;
            } else {
                // Sine-smoothed: 0→1→0 over blink_phase 0→1
                self.blink_progress = (self.blink_phase * std::f32::consts::PI).sin();
            }
        } else if self.blink_timer >= Self::BLINK_INTERVAL {
            self.trigger_blink();
        }
    }

    pub fn trigger_blink(&mut self) {
        if !self.is_blinking {
            self.is_blinking = true;
            self.blink_phase = 0.0;
        }
    }

    pub fn set_happiness(&mut self, value: f32) {
        self.happiness = value.clamp(0.0, 1.0);
    }

    pub fn uniforms(&self, time: f32, aspect: f32) -> FaceUniforms {
        FaceUniforms {
            time,
            blink_progress: self.blink_progress,
            happiness: self.happiness,
            aspect,
        }
    }
}
