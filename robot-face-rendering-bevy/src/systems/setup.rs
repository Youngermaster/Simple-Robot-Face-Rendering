//! Setup systems for initializing the robot face
//!
//! These systems run once at startup to create the initial scene.

use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::utils::create_bezier_curve_mesh;

/// Spawns the 2D camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Spawns all robot face entities (eyes, pupils, mouth)
pub fn setup_robot_face(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the robot face container
    commands
        .spawn((RobotFace, Transform::default(), Visibility::default()))
        .with_children(|parent| {
            // Left Eye
            let eye_mesh = meshes.add(Circle::new(EYE_RADIUS));
            let pupil_mesh = meshes.add(Circle::new(PUPIL_RADIUS));
            let highlight_mesh = meshes.add(Circle::new(HIGHLIGHT_RADIUS));

            parent
                .spawn((
                    EyeWhite {
                        side: Eye::Left,
                        original_scale: Vec3::ONE,
                    },
                    Blinking {
                        progress: 0.0,
                        closing: false,
                    },
                    Mesh2d(eye_mesh.clone()),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(-EYE_SPACING / 2.0, EYE_Y, 0.0),
                ))
                .with_children(|eye_parent| {
                    eye_parent
                        .spawn((
                            Pupil {
                                side: Eye::Left,
                                look_offset: Vec2::ZERO,
                            },
                            Mesh2d(pupil_mesh.clone()),
                            MeshMaterial2d(materials.add(Color::BLACK)),
                            Transform::from_xyz(0.0, 0.0, 1.0),
                        ))
                        .with_children(|pupil_parent| {
                            pupil_parent.spawn((
                                Highlight,
                                Mesh2d(highlight_mesh.clone()),
                                MeshMaterial2d(materials.add(Color::WHITE)),
                                Transform::from_xyz(-12.0, 12.0, 1.0),
                            ));
                        });
                });

            // Right Eye
            let eye_mesh = meshes.add(Circle::new(EYE_RADIUS));
            let pupil_mesh = meshes.add(Circle::new(PUPIL_RADIUS));
            let highlight_mesh = meshes.add(Circle::new(HIGHLIGHT_RADIUS));

            parent
                .spawn((
                    EyeWhite {
                        side: Eye::Right,
                        original_scale: Vec3::ONE,
                    },
                    Blinking {
                        progress: 0.0,
                        closing: false,
                    },
                    Mesh2d(eye_mesh),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(EYE_SPACING / 2.0, EYE_Y, 0.0),
                ))
                .with_children(|eye_parent| {
                    eye_parent
                        .spawn((
                            Pupil {
                                side: Eye::Right,
                                look_offset: Vec2::ZERO,
                            },
                            Mesh2d(pupil_mesh),
                            MeshMaterial2d(materials.add(Color::BLACK)),
                            Transform::from_xyz(0.0, 0.0, 1.0),
                        ))
                        .with_children(|pupil_parent| {
                            pupil_parent.spawn((
                                Highlight,
                                Mesh2d(highlight_mesh),
                                MeshMaterial2d(materials.add(Color::WHITE)),
                                Transform::from_xyz(-12.0, 12.0, 1.0),
                            ));
                        });
                });

            // Mouth - Bezier curve for smooth smile/frown
            let start = Vec2::new(-MOUTH_WIDTH / 2.0, 0.0);
            let end = Vec2::new(MOUTH_WIDTH / 2.0, 0.0);
            let control = Vec2::new(0.0, -40.0); // Negative = smile (curve down)

            let mouth_mesh = meshes.add(create_bezier_curve_mesh(start, control, end));

            parent.spawn((
                Mouth {
                    curve_offset: -40.0, // Start happy
                },
                Mesh2d(mouth_mesh),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::BLACK,
                    ..default()
                })),
                Transform::from_xyz(0.0, MOUTH_Y, 0.0),
            ));
        });
}

/// Sets up the UI controls text
pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        ControlsText,
        Text::new(
            "Controls:\n\
             H - Happy  S - Sad  N - Neutral\n\
             B - Blink  L - Look Around\n\
             ESC - Quit",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
}
