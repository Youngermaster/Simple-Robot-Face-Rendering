//! Blink animation implemented as an explicit state machine.
//!
//! Using an enum makes impossible states unrepresentable — you cannot
//! have `closing = true` AND `opening = true` simultaneously.

/// Blink animation state machine.
///
/// Transitions:
/// ```text
/// Open ──(timer expires or trigger())──► Closing
/// Closing ──(phase reaches 1.0)──────► Closed
/// Closed ──(timer expires)───────────► Opening
/// Opening ──(phase reaches 1.0)──────► Open
/// ```
pub enum BlinkState {
    /// Eyes fully open; counting down seconds until the next auto-blink.
    Open { timer: f32 },

    /// Eyes closing: `phase` advances from 0.0 → 1.0 over `BLINK_CLOSE_SECS`.
    Closing { phase: f32 },

    /// Eyes fully closed; brief pause before reopening.
    Closed { timer: f32 },

    /// Eyes opening: `phase` advances from 0.0 → 1.0 over `BLINK_OPEN_SECS`.
    Opening { phase: f32 },
}

impl BlinkState {
    /// Seconds between automatic blinks.
    const AUTO_INTERVAL: f32 = 3.0;
    /// Duration of the closing phase (eyelid sweeps down).
    const BLINK_CLOSE_SECS: f32 = 0.12;
    /// Duration the eye stays fully closed.
    const BLINK_HOLD_SECS: f32 = 0.06;
    /// Duration of the opening phase (eyelid sweeps up).
    const BLINK_OPEN_SECS: f32 = 0.12;

    /// Start in the open state.
    pub fn new() -> Self {
        BlinkState::Open {
            timer: Self::AUTO_INTERVAL,
        }
    }

    /// Advance the state machine by `dt` seconds. Call once per frame.
    pub fn update(&mut self, dt: f32) {
        *self = match self {
            BlinkState::Open { timer } => {
                let remaining = *timer - dt;
                if remaining <= 0.0 {
                    BlinkState::Closing { phase: 0.0 }
                } else {
                    BlinkState::Open { timer: remaining }
                }
            }
            BlinkState::Closing { phase } => {
                let new_phase = *phase + dt / Self::BLINK_CLOSE_SECS;
                if new_phase >= 1.0 {
                    BlinkState::Closed {
                        timer: Self::BLINK_HOLD_SECS,
                    }
                } else {
                    BlinkState::Closing { phase: new_phase }
                }
            }
            BlinkState::Closed { timer } => {
                let remaining = *timer - dt;
                if remaining <= 0.0 {
                    BlinkState::Opening { phase: 0.0 }
                } else {
                    BlinkState::Closed { timer: remaining }
                }
            }
            BlinkState::Opening { phase } => {
                let new_phase = *phase + dt / Self::BLINK_OPEN_SECS;
                if new_phase >= 1.0 {
                    BlinkState::Open {
                        timer: Self::AUTO_INTERVAL,
                    }
                } else {
                    BlinkState::Opening { phase: new_phase }
                }
            }
        };
    }

    /// Returns a closure factor in `[0.0, 1.0]`:
    /// - `0.0` = eyes fully open
    /// - `1.0` = eyes fully closed
    pub fn closure_factor(&self) -> f32 {
        match self {
            BlinkState::Open { .. } => 0.0,
            BlinkState::Closing { phase } => *phase,
            BlinkState::Closed { .. } => 1.0,
            BlinkState::Opening { phase } => 1.0 - phase,
        }
    }

    /// Request an immediate blink (triggered by keyboard shortcut or external event).
    /// If the eyes are already mid-blink, this is a no-op.
    pub fn trigger(&mut self) {
        if let BlinkState::Open { .. } = self {
            *self = BlinkState::Closing { phase: 0.0 };
        }
    }
}

impl Default for BlinkState {
    fn default() -> Self {
        Self::new()
    }
}
