//! Constants for robot face configuration
//!
//! Centralized constants make it easy to tweak the appearance and behavior.

/// Eye dimensions
pub const EYE_RADIUS: f32 = 60.0;
pub const PUPIL_RADIUS: f32 = 40.0;
pub const HIGHLIGHT_RADIUS: f32 = 15.0;

/// Eye positioning
pub const EYE_Y: f32 = 80.0;
pub const EYE_SPACING: f32 = 200.0;

/// Mouth configuration
pub const MOUTH_Y: f32 = -100.0;
pub const MOUTH_WIDTH: f32 = 280.0;
pub const MOUTH_SEGMENTS: usize = 30; // Number of segments for smooth Bezier curve
pub const MOUTH_THICKNESS: f32 = 8.0; // Thickness of the mouth line (stroke width)

/// Animation speeds
pub const BLINK_SPEED: f32 = 8.0; // How fast the blink animation runs
