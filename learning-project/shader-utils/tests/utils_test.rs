//! CPU tests for shader utilities
//!
//! This is the power of rust-gpu: test your shader logic on the CPU
//! before deploying to the GPU!

use shader_utils::*;
use glam::{vec2, vec4};

#[test]
fn test_animated_color_range() {
    // Test that animated color stays in valid range [0, 1]
    for i in 0..100 {
        let time = i as f32 * 0.1;
        let color = animated_color(time);

        assert!(color.x >= 0.0 && color.x <= 1.0, "Red channel out of range");
        assert!(color.y >= 0.0 && color.y <= 1.0, "Green channel out of range");
        assert!(color.z >= 0.0 && color.z <= 1.0, "Blue channel out of range");
        assert_eq!(color.w, 1.0, "Alpha should always be 1.0");
    }
}

#[test]
fn test_uv_gradient_corners() {
    // Test gradient at corners
    let bottom_left = uv_gradient(vec2(0.0, 0.0));
    assert_eq!(bottom_left, vec4(0.0, 0.0, 1.0, 1.0));

    let top_right = uv_gradient(vec2(1.0, 1.0));
    assert_eq!(top_right, vec4(1.0, 1.0, 0.0, 1.0));
}

#[test]
fn test_circle_pattern_center() {
    let center = vec2(0.5, 0.5);

    // At center, should be full brightness
    let at_center = circle_pattern(center, center, 0.3);
    assert!(at_center > 0.99, "Circle should be bright at center");

    // Far outside, should be dark
    let far_away = circle_pattern(vec2(2.0, 2.0), center, 0.3);
    assert!(far_away < 0.01, "Circle should be dark far from center");
}

#[test]
fn test_blend_colors() {
    let red = vec4(1.0, 0.0, 0.0, 1.0);
    let blue = vec4(0.0, 0.0, 1.0, 1.0);

    // 0% blend = all red
    let blend_0 = blend_colors(red, blue, 0.0);
    assert_eq!(blend_0, red);

    // 100% blend = all blue
    let blend_100 = blend_colors(red, blue, 1.0);
    assert_eq!(blend_100, blue);

    // 50% blend = purple
    let blend_50 = blend_colors(red, blue, 0.5);
    assert_eq!(blend_50, vec4(0.5, 0.0, 0.5, 1.0));
}

#[test]
fn test_wave_pattern_oscillation() {
    // Wave should oscillate between -amplitude and +amplitude
    let amplitude = 2.0;
    let frequency = 1.0;

    for i in 0..100 {
        let pos = i as f32 * 0.1;
        let time = 0.0;
        let wave = wave_pattern(pos, time, frequency, amplitude);

        assert!(wave >= -amplitude && wave <= amplitude,
            "Wave should stay within amplitude bounds");
    }
}

#[test]
fn test_checkerboard_alternates() {
    // Test that checkerboard alternates correctly
    let scale = 4.0;

    // Test two squares that should definitely be different colors
    let check_00 = checkerboard(vec2(0.1, 0.1), scale); // Square (0,0)
    let check_10 = checkerboard(vec2(0.3, 0.1), scale); // Square (1,0)

    // Adjacent squares should have different values
    assert_ne!(check_00, check_10, "Adjacent checkerboard squares should differ");

    // Also test that same square has same value
    let check_00_again = checkerboard(vec2(0.15, 0.15), scale);
    assert_eq!(check_00, check_00_again, "Same square should have same value");
}

#[test]
fn test_saturate_clamps() {
    // Test that saturate properly clamps values
    assert_eq!(saturate(-1.0), 0.0);
    assert_eq!(saturate(0.5), 0.5);
    assert_eq!(saturate(2.0), 1.0);
}
