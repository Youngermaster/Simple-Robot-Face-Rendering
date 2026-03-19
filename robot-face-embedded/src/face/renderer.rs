//! Generic renderer — compiles for any `DrawTarget<Color = BinaryColor>`.
//!
//! This module never imports `embedded-graphics-simulator`. It is equally
//! valid for a real SH1106 or SSD1306 driver.

use embedded_graphics::{
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Arc, Circle, Line, PrimitiveStyle},
};

use crate::face::{config, emotion::Emotion, RobotFace};

/// Draw the complete robot face onto `target`.
///
/// Returns `Err(D::Error)` if the underlying display driver reports an error.
pub fn draw<D>(face: &RobotFace, target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    // Clear to black (all pixels off).
    target.clear(BinaryColor::Off)?;

    let closure = face.blink.closure_factor();
    let emotion = face.emotion.current;

    draw_eye(target, config::LEFT_EYE, closure, emotion, false)?;
    draw_eye(target, config::RIGHT_EYE, closure, emotion, true)?;
    draw_mouth(target, emotion)?;
    draw_brows(target, emotion)?;

    Ok(())
}

// ── Eye ──────────────────────────────────────────────────────────────────────

fn draw_eye<D>(
    target: &mut D,
    center: Point,
    closure: f32,
    emotion: Emotion,
    is_right: bool,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let stroke_on = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let fill_off = PrimitiveStyle::with_fill(BinaryColor::Off);
    let fill_on = PrimitiveStyle::with_fill(BinaryColor::On);

    if closure > 0.5 {
        // Blinking — draw a horizontal line across the eye centre.
        let half = config::EYE_OUTER_RADIUS as i32;
        Line::new(center - Point::new(half, 0), center + Point::new(half, 0))
            .into_styled(stroke_on)
            .draw(target)?;
        return Ok(());
    }

    match (emotion, is_right) {
        (Emotion::Surprised, _) => {
            // Larger outer circle.
            let r = config::EYE_OUTER_RADIUS + 3;
            circle_centered(center, r)
                .into_styled(stroke_on)
                .draw(target)?;
            circle_centered(center, config::EYE_PUPIL_RADIUS)
                .into_styled(fill_on)
                .draw(target)?;
        }
        (Emotion::Thinking, true) => {
            // Right eye squints — draw only the top arc (180° arc).
            let r = config::EYE_OUTER_RADIUS as i32;
            Arc::new(
                center - Point::new(r, r),
                config::EYE_OUTER_RADIUS * 2,
                180.0.deg(),
                180.0.deg(),
            )
            .into_styled(stroke_on)
            .draw(target)?;
        }
        _ => {
            // Default eye: outer ring → black pupil fill → white highlight.
            circle_centered(center, config::EYE_OUTER_RADIUS)
                .into_styled(stroke_on)
                .draw(target)?;

            // Pupil (pixels OFF = black on OLED).
            circle_centered(center, config::EYE_PUPIL_RADIUS)
                .into_styled(fill_off)
                .draw(target)?;
            // Re-draw pupil border so it's visible against the black background.
            circle_centered(center, config::EYE_PUPIL_RADIUS)
                .into_styled(stroke_on)
                .draw(target)?;

            // Highlight (pixels ON = bright spot).
            let hl_center = center + Point::new(-3, -3);
            circle_centered(hl_center, config::EYE_HIGHLIGHT_RADIUS)
                .into_styled(fill_on)
                .draw(target)?;
        }
    }

    Ok(())
}

// ── Mouth ─────────────────────────────────────────────────────────────────────

fn draw_mouth<D>(target: &mut D, emotion: Emotion) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let stroke_on = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let c = config::MOUTH_CENTER;
    let r = (config::MOUTH_DIAMETER / 2) as i32;

    match emotion {
        Emotion::Happy => {
            // Bottom half arc → smile.
            Arc::new(
                c - Point::new(r, r),
                config::MOUTH_DIAMETER,
                0.0.deg(),
                180.0.deg(),
            )
            .into_styled(stroke_on)
            .draw(target)?;
        }
        Emotion::Neutral => {
            // Straight horizontal line.
            Line::new(c - Point::new(r, 0), c + Point::new(r, 0))
                .into_styled(stroke_on)
                .draw(target)?;
        }
        Emotion::Sad => {
            // Top half arc → frown.
            Arc::new(
                c - Point::new(r, r),
                config::MOUTH_DIAMETER,
                180.0.deg(),
                180.0.deg(),
            )
            .into_styled(stroke_on)
            .draw(target)?;
        }
        Emotion::Angry => {
            // Two diagonal lines meeting in the middle — scowl.
            Line::new(c - Point::new(r, -2), c + Point::new(0, 2))
                .into_styled(stroke_on)
                .draw(target)?;
            Line::new(c + Point::new(r, -2), c + Point::new(0, 2))
                .into_styled(stroke_on)
                .draw(target)?;
        }
        Emotion::Surprised => {
            // Small 'O' mouth.
            circle_centered(c, 5)
                .into_styled(stroke_on)
                .draw(target)?;
        }
        Emotion::Thinking => {
            // Short diagonal line offset to one side.
            Line::new(c + Point::new(0, 0), c + Point::new(r, -3))
                .into_styled(stroke_on)
                .draw(target)?;
        }
    }

    Ok(())
}

// ── Eyebrows ──────────────────────────────────────────────────────────────────

fn draw_brows<D>(target: &mut D, emotion: Emotion) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let stroke_on = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let hw = config::BROW_HALF_WIDTH;
    let oy = config::BROW_OFFSET_Y;

    match emotion {
        Emotion::Sad => {
            // \ / — inner corners raised.
            let left = config::LEFT_EYE;
            Line::new(
                left + Point::new(-hw, -oy),
                left + Point::new(hw, -oy - 4),
            )
            .into_styled(stroke_on)
            .draw(target)?;

            let right = config::RIGHT_EYE;
            Line::new(
                right + Point::new(-hw, -oy - 4),
                right + Point::new(hw, -oy),
            )
            .into_styled(stroke_on)
            .draw(target)?;
        }
        Emotion::Angry => {
            // / \ — inner corners lowered sharply.
            let left = config::LEFT_EYE;
            Line::new(
                left + Point::new(-hw, -oy - 4),
                left + Point::new(hw, -oy),
            )
            .into_styled(stroke_on)
            .draw(target)?;

            let right = config::RIGHT_EYE;
            Line::new(
                right + Point::new(-hw, -oy),
                right + Point::new(hw, -oy - 4),
            )
            .into_styled(stroke_on)
            .draw(target)?;
        }
        _ => {}
    }

    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Build a `Circle` centred at `center` with the given radius.
fn circle_centered(center: Point, radius: u32) -> Circle {
    let r = radius as i32;
    Circle::new(center - Point::new(r, r), radius * 2)
}
