//! Systems module
//!
//! All game logic is organized into systems that operate on components and resources.
//! Systems are grouped by functionality for better organization.

pub mod setup;
pub mod animation;
pub mod input;
pub mod ui;

// Re-export all systems for convenience
pub use setup::{setup_camera, setup_robot_face, setup_ui};
pub use animation::{auto_blink_system, blink_animation_system, emotion_update_system, pupil_look_system};
pub use input::keyboard_input_system;
pub use ui::update_ui_system;
