//! Animation systems for robot face behaviors
//!
//! These systems handle blinking, emotion changes, and pupil movement.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::utils::create_bezier_curve_mesh;

/// Triggers automatic blinks at regular intervals
pub fn auto_blink_system(
    time: Res<Time>,
    mut timer: ResMut<AutoBlinkTimer>,
    mut query: Query<&mut Blinking>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        // Trigger blink on all eyes
        for mut blinking in query.iter_mut() {
            if blinking.progress == 0.0 {
                blinking.closing = true;
            }
        }
    }
}

/// Animates the blinking behavior by scaling eyes vertically
pub fn blink_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Blinking, &mut Transform, &EyeWhite)>,
) {
    for (mut blinking, mut transform, eye_white) in query.iter_mut() {
        if blinking.closing {
            // Close the eye
            blinking.progress += time.delta_secs() * BLINK_SPEED;
            if blinking.progress >= 1.0 {
                blinking.progress = 1.0;
                blinking.closing = false; // Start opening
            }
        } else if blinking.progress > 0.0 {
            // Open the eye
            blinking.progress -= time.delta_secs() * BLINK_SPEED;
            if blinking.progress <= 0.0 {
                blinking.progress = 0.0;
            }
        }

        // Animate by scaling Y (squishing the eye vertically)
        let scale_y = 1.0 - blinking.progress; // 1.0 (open) -> 0.0 (closed)
        transform.scale = eye_white.original_scale * Vec3::new(1.0, scale_y.max(0.1), 1.0);
    }
}

/// Updates the mouth shape based on current emotion
pub fn emotion_update_system(
    emotion: Res<Emotion>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Mouth, &Mesh2d)>,
) {
    if !emotion.is_changed() {
        return;
    }

    for (mut mouth, mesh_handle) in query.iter_mut() {
        // Map happiness to curve offset
        // Happy (1.0) = smile (curve down) = -40
        // Neutral (0.5) = straight = 0
        // Sad (0.0) = frown (curve up) = 40
        let target_offset = -80.0 * (emotion.happiness - 0.5);

        // Only update if changed significantly
        if (mouth.curve_offset - target_offset).abs() > 0.5 {
            mouth.curve_offset = target_offset;

            // Regenerate the Bezier curve mesh with new control point
            let start = Vec2::new(-MOUTH_WIDTH / 2.0, 0.0);
            let end = Vec2::new(MOUTH_WIDTH / 2.0, 0.0);
            let control = Vec2::new(0.0, mouth.curve_offset);

            let new_mesh = create_bezier_curve_mesh(start, control, end);

            // Update the mesh asset
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                *mesh = new_mesh;
            }
        }
    }
}

/// Smoothly moves pupils to their target look offset
pub fn pupil_look_system(mut query: Query<(&Pupil, &mut Transform)>) {
    for (pupil, mut transform) in query.iter_mut() {
        // Smoothly move pupil to the look offset
        let target = Vec3::new(pupil.look_offset.x, pupil.look_offset.y, 1.0);
        transform.translation = transform.translation.lerp(target, 0.1);
    }
}
