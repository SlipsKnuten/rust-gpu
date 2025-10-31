// GPU Shader - Now using the FULL power of rust-gpu!
//
// This shader demonstrates:
// 1. Using functions from shader-utils (tested on CPU!)
// 2. Using utilities from the shared crate
// 3. Creating dynamic, interesting effects
// 4. All written in regular Rust!

#![cfg_attr(target_arch = "spirv", no_std)]

use shared::glam::{Vec2, Vec4, vec2, vec4};
use spirv_std::spirv;

// Import our utilities that work on BOTH CPU and GPU!
use shader_utils::{
    animated_color,
    uv_gradient,
    circle_pattern,
    blend_colors,
    wave_pattern,
    checkerboard,
};

// FRAGMENT SHADER
// Now this does something much more interesting!
#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,  // Pixel position
    output: &mut Vec4,  // Output color
) {
    // Convert fragment coordinates to UV space (0.0 to 1.0)
    // Hardcoded screen size for now - you could pass this as a uniform
    let screen_size = vec2(800.0, 600.0);
    let uv = vec2(frag_coord.x, frag_coord.y) / screen_size;

    // Demo 1: Basic gradient from our utilities
    let gradient = uv_gradient(uv);

    // Demo 2: Animated color using time-based function
    // (In a real app, you'd pass time as a uniform, but we'll fake it)
    let fake_time = uv.x * 10.0; // Use position as fake time for demo
    let animated = animated_color(fake_time);

    // Demo 3: Create a circle in the center
    let center = vec2(0.5, 0.5);
    let circle = circle_pattern(uv, center, 0.3);

    // Demo 4: Create a wave pattern
    let wave = wave_pattern(uv.x, uv.y * 2.0, 10.0, 0.5);

    // Demo 5: Checkerboard pattern
    let checker = checkerboard(uv, 10.0);

    // Combine effects!
    // Let's blend between gradient and animated color based on the circle
    let base_color = blend_colors(gradient, animated, circle);

    // Add some wave distortion for fun
    let wave_color = vec4(wave, wave, wave, 0.0);
    let with_wave = base_color + wave_color * 0.2;

    // Add checkerboard overlay in one corner
    let checker_color = vec4(checker, checker, checker, 1.0);
    let corner_region = if uv.x < 0.3 && uv.y < 0.3 {
        blend_colors(with_wave, checker_color, 0.5)
    } else {
        with_wave
    };

    *output = corner_region;
}

// VERTEX SHADER
// Same triangle setup, but now you understand how it all fits together!
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    // Create a fullscreen triangle
    // This covers the entire screen so our fragment shader runs on every pixel
    *out_pos = vec4(
        (vert_id - 1) as f32,
        ((vert_id & 1) * 2 - 1) as f32,
        0.0,
        1.0,
    );
}

// WHAT'S POWERFUL HERE:
//
// 1. All those functions (animated_color, blend_colors, etc.) are TESTED on CPU
//    Run: `cargo test -p shader-utils` to see them tested!
//
// 2. Same functions work on GPU - no code duplication!
//
// 3. You can experiment with shader logic in CPU tests before deploying to GPU
//
// 4. You're using professional shader development practices from day one
//
// 5. Everything is type-safe, memory-safe Rust!
