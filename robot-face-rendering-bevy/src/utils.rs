//! Utility functions for mesh generation and helpers
//!
//! These are reusable functions that don't fit into systems or components.

use bevy::prelude::*;
use crate::constants::{MOUTH_SEGMENTS, MOUTH_THICKNESS};

/// Creates a mesh for a quadratic Bezier curve with thickness
///
/// # Arguments
/// * `start` - Starting point of the curve
/// * `control` - Control point that defines the curve's shape
/// * `end` - Ending point of the curve
///
/// # Returns
/// A `Mesh` representing a thick stroke along the Bezier curve
pub fn create_bezier_curve_mesh(start: Vec2, control: Vec2, end: Vec2) -> Mesh {
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
