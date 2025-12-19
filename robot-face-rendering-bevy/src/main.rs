//! # Robot Face Rendering with Bevy
//!
//! A real-time robot face renderer showcasing proper ECS (Entity Component System) architecture.
//!
//! ## Features
//! - Emotional expressions (happy, neutral, sad)
//! - Animated blinking
//! - Pupil movement
//! - Smooth Bezier curve mouth
//! - WASM support
//!
//! ## Architecture
//! The project follows Bevy's ECS paradigm with clear separation:
//! - **Components**: Pure data structures (`components.rs`)
//! - **Resources**: Global state (`resources.rs`)
//! - **Systems**: Game logic (`systems/`)
//! - **Constants**: Configuration values (`constants.rs`)
//! - **Utils**: Helper functions (`utils.rs`)

use bevy::prelude::*;

// Module declarations
mod components;
mod resources;
mod constants;
mod utils;
mod systems;

// Import what we need for the app setup
use resources::{Emotion, AutoBlinkTimer};
use systems::*;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Robot Face Rendering".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .init_resource::<Emotion>()
        .init_resource::<AutoBlinkTimer>()
        // Startup systems
        .add_systems(Startup, (setup_camera, setup_robot_face, setup_ui))
        // Update systems
        .add_systems(
            Update,
            (
                keyboard_input_system,
                auto_blink_system,
                blink_animation_system,
                emotion_update_system,
                pupil_look_system,
                update_ui_system,
            ),
        )
        .run();
}
