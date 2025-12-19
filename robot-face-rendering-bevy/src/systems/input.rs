//! Input handling systems
//!
//! Processes keyboard input for controlling emotions and gestures.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

/// Handles keyboard input for emotion changes, blinking, and looking around
pub fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut emotion: ResMut<Emotion>,
    mut blink_query: Query<&mut Blinking>,
    mut pupil_query: Query<&mut Pupil>,
) {
    // Emotion controls
    if keyboard.just_pressed(KeyCode::KeyH) {
        emotion.happiness = 1.0; // Happy
    }
    if keyboard.just_pressed(KeyCode::KeyN) {
        emotion.happiness = 0.5; // Neutral
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        emotion.happiness = 0.0; // Sad
    }

    // Manual blink trigger
    if keyboard.just_pressed(KeyCode::KeyB) {
        // Trigger blink on all eyes that aren't already blinking
        for mut blinking in blink_query.iter_mut() {
            if blinking.progress == 0.0 {
                blinking.closing = true;
            }
        }
    }

    // Look around (move pupils randomly)
    if keyboard.just_pressed(KeyCode::KeyL) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_offset = Vec2::new(rng.gen_range(-15.0..15.0), rng.gen_range(-15.0..15.0));

        for mut pupil in pupil_query.iter_mut() {
            pupil.look_offset = random_offset;
        }
    }

    // Quit - Use immediate exit to avoid macOS hang issues
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("ESC pressed - exiting application");
        std::process::exit(0);
    }
}
