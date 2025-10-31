//! Shader utilities that work on BOTH CPU and GPU!
//!
//! This is the power of rust-gpu: write functions once, test them on CPU,
//! then use them in your GPU shaders with confidence.

#![cfg_attr(target_arch = "spirv", no_std)]

use shared::glam::{Vec2, Vec3, Vec4, vec3, vec4};

// When compiling for GPU (spirv target), we need the Float trait for sin, cos, etc.
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

// Re-export shared utilities so users can access them
pub use shared::{saturate, smoothstep, pow, exp};

/// Create a color that oscillates over time
/// This function works on BOTH CPU (for tests) and GPU (in shaders)!
pub fn animated_color(time: f32) -> Vec4 {
    let r = (time).sin() * 0.5 + 0.5;
    let g = (time + 2.0).sin() * 0.5 + 0.5;
    let b = (time + 4.0).sin() * 0.5 + 0.5;
    vec4(r, g, b, 1.0)
}

/// Create a gradient based on UV coordinates (0.0 to 1.0)
pub fn uv_gradient(uv: Vec2) -> Vec4 {
    vec4(uv.x, uv.y, 1.0 - uv.x, 1.0)
}

/// Create a circular pattern
/// Returns 1.0 at the center, fading to 0.0 at the edges
pub fn circle_pattern(uv: Vec2, center: Vec2, radius: f32) -> f32 {
    let dist = (uv - center).length();
    let edge_smoothness = 0.01;

    // Use smoothstep from shared crate for smooth falloff
    1.0 - smoothstep(radius - edge_smoothness, radius + edge_smoothness, dist)
}

/// Combine two colors with a blend factor
pub fn blend_colors(color_a: Vec4, color_b: Vec4, factor: f32) -> Vec4 {
    let t = saturate(factor); // Use saturate from shared crate
    color_a * (1.0 - t) + color_b * t
}

/// Create a wave pattern that can be used for effects
pub fn wave_pattern(pos: f32, time: f32, frequency: f32, amplitude: f32) -> f32 {
    (pos * frequency + time).sin() * amplitude
}

/// Convert a direction to a color (useful for debugging normals, etc.)
pub fn direction_to_color(dir: Vec3) -> Vec4 {
    // Map from [-1, 1] to [0, 1]
    let color = (dir + vec3(1.0, 1.0, 1.0)) * 0.5;
    vec4(color.x, color.y, color.z, 1.0)
}

/// Create a checkerboard pattern
pub fn checkerboard(uv: Vec2, scale: f32) -> f32 {
    let scaled_uv = uv * scale;
    let checker = ((scaled_uv.x.floor() + scaled_uv.y.floor()) % 2.0).abs();
    checker
}
