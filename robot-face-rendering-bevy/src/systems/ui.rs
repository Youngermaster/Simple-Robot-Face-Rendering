//! UI update systems
//!
//! Handles updating UI elements based on game state.

use bevy::prelude::*;
use crate::components::ControlsText;
use crate::resources::Emotion;

/// Updates the UI text to show current emotion
pub fn update_ui_system(emotion: Res<Emotion>, mut query: Query<&mut Text, With<ControlsText>>) {
    if !emotion.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        let emotion_name = if emotion.happiness > 0.8 {
            "Happy 😊"
        } else if emotion.happiness > 0.3 {
            "Neutral 😐"
        } else {
            "Sad 😢"
        };

        text.0 = format!(
            "Controls:\n\
             H - Happy  S - Sad  N - Neutral\n\
             B - Blink  L - Look Around\n\
             ESC - Quit\n\n\
             Current: {}",
            emotion_name
        );
    }
}
