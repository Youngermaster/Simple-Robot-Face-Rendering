use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Robot Face Rendering".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Emotion>()
        .init_resource::<AutoBlinkTimer>()
        .add_systems(Startup, (setup_camera, setup_robot_face, setup_ui))
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

// ============================================================================
// COMPONENTS - Pure data, no logic
// ============================================================================

/// Marker component for the robot face container
#[derive(Component)]
struct RobotFace;

/// Which eye this is
#[derive(Component, Clone, Copy, PartialEq)]
enum Eye {
    Left,
    Right,
}

/// The outer white part of the eye
#[derive(Component)]
struct EyeWhite {
    side: Eye,
    original_scale: Vec3,
}

/// The black pupil
#[derive(Component)]
struct Pupil {
    side: Eye,
    /// Offset from eye center for "looking around"
    look_offset: Vec2,
}

/// White highlight on pupil
#[derive(Component)]
struct Highlight;

/// The mouth curve - stores control point offset for Bezier curve
#[derive(Component)]
struct Mouth {
    /// Vertical offset of control point (positive = smile, negative = frown)
    curve_offset: f32,
}

/// Blink animation state
#[derive(Component)]
struct Blinking {
    /// 0.0 = fully open, 1.0 = fully closed
    progress: f32,
    /// Whether we're closing or opening
    closing: bool,
}

/// UI text showing controls
#[derive(Component)]
struct ControlsText;

// ============================================================================
// RESOURCES - Global state
// ============================================================================

/// Current emotion state
#[derive(Resource)]
struct Emotion {
    /// 0.0 = sad, 0.5 = neutral, 1.0 = happy
    happiness: f32,
}

impl Default for Emotion {
    fn default() -> Self {
        Self { happiness: 0.8 }
    }
}

/// Timer for automatic blinking
#[derive(Resource)]
struct AutoBlinkTimer {
    timer: Timer,
}

impl Default for AutoBlinkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}

// ============================================================================
// CONSTANTS
// ============================================================================

const EYE_RADIUS: f32 = 60.0;
const PUPIL_RADIUS: f32 = 40.0;
const HIGHLIGHT_RADIUS: f32 = 15.0;
const EYE_Y: f32 = 80.0;
const EYE_SPACING: f32 = 200.0;
const MOUTH_Y: f32 = -100.0;
const MOUTH_WIDTH: f32 = 280.0;

const BLINK_SPEED: f32 = 8.0; // How fast the blink animation runs
const MOUTH_SEGMENTS: usize = 30; // Number of segments for smooth Bezier curve
const MOUTH_THICKNESS: f32 = 8.0; // Thickness of the mouth line (stroke width)

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Creates a mesh for a quadratic Bezier curve with thickness
/// start: starting point, control: control point, end: ending point
fn create_bezier_curve_mesh(start: Vec2, control: Vec2, end: Vec2) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    let half_thickness = MOUTH_THICKNESS / 2.0;

    // Generate points along the Bezier curve and create a thick stroke
    for i in 0..=MOUTH_SEGMENTS {
        let t = i as f32 / MOUTH_SEGMENTS as f32;

        // Quadratic Bezier formula: B(t) = (1-t)²P0 + 2(1-t)tP1 + t²P2
        let point = (1.0 - t).powi(2) * start
                  + 2.0 * (1.0 - t) * t * control
                  + t.powi(2) * end;

        // Calculate tangent for perpendicular offset
        // Derivative of Bezier: B'(t) = 2(1-t)(P1-P0) + 2t(P2-P1)
        let tangent = 2.0 * (1.0 - t) * (control - start)
                    + 2.0 * t * (end - control);

        // Perpendicular to tangent (rotated 90 degrees)
        let perpendicular = Vec2::new(-tangent.y, tangent.x).normalize_or_zero();

        // Create vertices on both sides of the curve
        let offset = perpendicular * half_thickness;
        let top = point + offset;
        let bottom = point - offset;

        positions.push([top.x, top.y, 0.0]);
        positions.push([bottom.x, bottom.y, 0.0]);
    }

    // Create triangle strip indices
    for i in 0..(MOUTH_SEGMENTS as u32) {
        let base = i * 2;

        // First triangle
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);

        // Second triangle
        indices.push(base + 1);
        indices.push(base + 3);
        indices.push(base + 2);
    }

    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(bevy::mesh::Indices::U32(indices));
    mesh
}

// ============================================================================
// STARTUP SYSTEMS
// ============================================================================

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_robot_face(
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
            // Start with a happy smile (control point below the line)
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

fn setup_ui(mut commands: Commands) {
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

// ============================================================================
// UPDATE SYSTEMS
// ============================================================================

fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut emotion: ResMut<Emotion>,
    mut blink_query: Query<&mut Blinking>,
    mut pupil_query: Query<&mut Pupil>,
    mut exit: MessageWriter<AppExit>,
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

    // Quit
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn auto_blink_system(
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

fn blink_animation_system(
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

fn emotion_update_system(
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

fn pupil_look_system(mut query: Query<(&Pupil, &mut Transform)>) {
    for (pupil, mut transform) in query.iter_mut() {
        // Smoothly move pupil to the look offset
        let target = Vec3::new(pupil.look_offset.x, pupil.look_offset.y, 1.0);
        transform.translation = transform.translation.lerp(target, 0.1);
    }
}

fn update_ui_system(emotion: Res<Emotion>, mut query: Query<&mut Text, With<ControlsText>>) {
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
