//! Layout constants for a 128×64 OLED display (SH1106 / SSD1306).
//!
//! All coordinates and radii are in physical pixels.
//! Edit this file to port the face to a different display size.
//!
//! ```text
//! 0                   64                  127
//! ┌───────────────────────────────────────┐  0
//! │         ╭────╮           ╭────╮       │
//! │         │ ●  │           │ ●  │       │  22  ← EYE_Y
//! │         ╰────╯           ╰────╯       │
//! │                                       │
//! │              ╭──────╮                 │  48  ← MOUTH_Y
//! └───────────────────────────────────────┘  63
//!
//! LEFT_EYE  = (32, 22)      RIGHT_EYE = (96, 22)
//! ```

use embedded_graphics::geometry::Point;

pub const DISPLAY_W: i32 = 128;
pub const DISPLAY_H: i32 = 64;

pub const LEFT_EYE: Point = Point::new(32, 22);
pub const RIGHT_EYE: Point = Point::new(96, 22);

pub const EYE_OUTER_RADIUS: u32 = 12;
pub const EYE_PUPIL_RADIUS: u32 = 6;
pub const EYE_HIGHLIGHT_RADIUS: u32 = 2;

pub const MOUTH_CENTER: Point = Point::new(64, 48);
pub const MOUTH_DIAMETER: u32 = 28;

pub const BROW_HALF_WIDTH: i32 = 10;
pub const BROW_OFFSET_Y: i32 = 8;
