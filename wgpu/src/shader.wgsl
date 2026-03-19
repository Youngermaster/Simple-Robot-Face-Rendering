// Robot Face WGSL Shader — SDF-based 2D rendering
// All shapes are rendered via Signed Distance Fields for clean, resolution-independent graphics.

struct FaceUniforms {
    time:           f32,
    blink_progress: f32,  // 0.0 = open, 1.0 = closed
    happiness:      f32,  // 0.0 = sad, 0.5 = neutral, 1.0 = happy
    aspect:         f32,  // width / height
}

@group(0) @binding(0)
var<uniform> u: FaceUniforms;

// ---------------------------------------------------------------------------
// Vertex shader — fullscreen triangle (no vertex buffer required)
// Three vertices cover the entire clip space:  (-1,-1), (3,-1), (-1,3)
// ---------------------------------------------------------------------------
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0)       uv:       vec2<f32>,  // NDC-space UV, range [-1, 1]
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    let pos = positions[vid];
    var out: VertexOutput;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = pos; // [-1,1] in both axes before aspect correction
    return out;
}

// ---------------------------------------------------------------------------
// SDF helpers
// ---------------------------------------------------------------------------

// Inigo Quilez rounded-box SDF
// p = point, b = half-extents, r = corner radius
fn sdf_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r;
}

// Soft glow falloff starting at d=0, fading to 0 over `width`
fn glow(d: f32, width: f32) -> f32 {
    return clamp(1.0 - d / width, 0.0, 1.0);
}

// 8x8 LED dot grid — returns intensity 0..1
fn led_grid(p: vec2<f32>, density: f32) -> f32 {
    let cell = fract(p * density) - 0.5; // center of each cell
    let dot_r = 0.3;                     // dot radius within cell (relative)
    let dist = length(cell);
    return 1.0 - smoothstep(dot_r - 0.05, dot_r + 0.05, dist);
}

// ---------------------------------------------------------------------------
// Fragment shader
// ---------------------------------------------------------------------------
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Correct for aspect ratio so shapes aren't stretched
    var uv = vec2<f32>(in.uv.x * u.aspect, in.uv.y);

    // -----------------------------------------------------------------------
    // 1. Screen background — large rounded rectangle, near-black
    // -----------------------------------------------------------------------
    let screen_half = vec2<f32>(u.aspect * 0.88, 0.88);
    let d_screen = sdf_rounded_box(uv, screen_half, 0.12);

    let bg_color    = vec3<f32>(0.03, 0.03, 0.06);
    let border_color = vec3<f32>(0.0, 0.45, 0.48);

    // Start with pitch-black outside the screen
    var color = vec3<f32>(0.0, 0.0, 0.0);

    // Fill inside screen
    if d_screen < 0.0 {
        color = bg_color;
    }

    // Subtle border glow on screen edge
    let screen_border = glow(abs(d_screen), 0.03) * 0.4;
    color += border_color * screen_border;

    // -----------------------------------------------------------------------
    // 2. Eyes — two rounded rectangles, landscape orientation
    //    Eye centers at (±0.42 * aspect, +0.18) in aspect-corrected space
    // -----------------------------------------------------------------------
    let eye_offset_x = 0.42 * u.aspect;
    let eye_offset_y = 0.18;

    // Eye dimensions — wider than tall for a "LED panel" look
    let eye_half_base = vec2<f32>(0.28 * u.aspect, 0.18);
    let eye_radius    = 0.10;

    // Cyan LED colors
    let dim_cyan    = vec3<f32>(0.0,  0.40, 0.45);
    let bright_cyan = vec3<f32>(0.05, 0.90, 0.95);
    let glow_cyan   = vec3<f32>(0.0,  0.70, 0.80);

    // Blink squishes the eye vertically (Y half-extent → ~0 when closed)
    let blink_scale = 1.0 - u.blink_progress * 0.97;
    let eye_half    = vec2<f32>(eye_half_base.x, eye_half_base.y * blink_scale);

    // Process both eyes in a loop (Rust-side; WGSL unrolled manually)
    for (var side = 0; side < 2; side++) {
        let sign_x   = select(-1.0, 1.0, side == 0);
        let eye_center = vec2<f32>(sign_x * eye_offset_x, eye_offset_y);
        let p_eye    = uv - eye_center;

        let d_eye = sdf_rounded_box(p_eye, eye_half, eye_radius * blink_scale);

        if d_eye < 0.0 {
            // Local UV inside eye box for grid, range ~[-1,1]
            let local = p_eye / eye_half_base;

            // LED grid with subtle time-based shimmer
            let shimmer = 0.85 + 0.15 * sin(u.time * 2.0 + local.x * 3.0 + local.y * 2.0);
            let grid    = led_grid(local, 4.0) * shimmer;

            let eye_fill = mix(dim_cyan, bright_cyan, grid);
            color = mix(color, eye_fill, 1.0); // full coverage inside eye
        }

        // Glow halo around each eye
        if d_eye >= 0.0 && d_eye < 0.12 {
            let g = glow(d_eye, 0.12);
            color += glow_cyan * g * 0.55;
        }
    }

    // -----------------------------------------------------------------------
    // 3. Mouth — parabolic curve SDF
    //    happiness: 0=sad (inverted arc), 0.5=flat, 1=smile (arc down)
    // -----------------------------------------------------------------------
    let mouth_center = vec2<f32>(0.0, -0.30);
    let mouth_width  = 0.34 * u.aspect;
    let mouth_thick  = 0.025;

    // Map happiness to curvature: positive = smile, negative = frown
    let curvature = (u.happiness - 0.5) * 1.2; // range ±0.6

    // For a point p relative to mouth center, compute distance to parabolic arc
    let p_mouth = uv - mouth_center;
    // Parabola: y = curvature * (x/width)^2   →  dist ≈ |y - curve_y| near x axis
    let t       = clamp(p_mouth.x / mouth_width, -1.0, 1.0);
    let curve_y = curvature * t * t - abs(curvature) * 0.5;
    let on_arc  = vec2<f32>(t * mouth_width, curve_y);
    let d_mouth = length(p_mouth - on_arc) - mouth_thick;

    if d_mouth < 0.0 {
        color = mix(color, bright_cyan * 0.9, 1.0);
    } else if d_mouth < 0.04 {
        let g = glow(d_mouth, 0.04);
        color += glow_cyan * g * 0.4;
    }

    // -----------------------------------------------------------------------
    // 4. Vignette — darken corners of the whole frame
    // -----------------------------------------------------------------------
    let vignette = 1.0 - smoothstep(0.7, 1.4, length(uv / vec2<f32>(u.aspect, 1.0)));
    color *= vignette;

    return vec4<f32>(color, 1.0);
}
