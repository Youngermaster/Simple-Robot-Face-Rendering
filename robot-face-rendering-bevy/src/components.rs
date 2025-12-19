//! Component definitions for the robot face
//!
//! Components are pure data structures with no logic, following ECS principles.

use bevy::prelude::*;

/// Marker component for the robot face container
#[derive(Component)]
pub struct RobotFace;

/// Which eye this is
#[derive(Component, Clone, Copy, PartialEq)]
pub enum Eye {
    Left,
    Right,
}

/// The outer white part of the eye
#[derive(Component)]
pub struct EyeWhite {
    pub side: Eye,
    pub original_scale: Vec3,
}

/// The black pupil
#[derive(Component)]
pub struct Pupil {
    pub side: Eye,
    /// Offset from eye center for "looking around"
    pub look_offset: Vec2,
}

/// White highlight on pupil
#[derive(Component)]
pub struct Highlight;

/// The mouth curve - stores control point offset for Bezier curve
#[derive(Component)]
pub struct Mouth {
    /// Vertical offset of control point (positive = smile, negative = frown)
    pub curve_offset: f32,
}

/// Blink animation state
#[derive(Component)]
pub struct Blinking {
    /// 0.0 = fully open, 1.0 = fully closed
    pub progress: f32,
    /// Whether we're closing or opening
    pub closing: bool,
}

/// UI text showing controls
#[derive(Component)]
pub struct ControlsText;
