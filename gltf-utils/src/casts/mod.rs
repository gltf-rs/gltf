//! This module provides casting iterators for certain types of accessors.

/// Casting iterator and accompanying types for `Colors`.
pub mod colors;
/// Casting iterator and accompanying types for `Indices`.
pub mod indices;
/// Casting iterator and accompanying types for `Joints`.
pub mod joints;
/// Casting iterator and accompanying types for `TexCoords`.
pub mod tex_coords;
/// Casting iterator and accompanying types for `Weights`.
pub mod weights;

fn norm_f32_as_u8(x: f32) -> u8 {
    (x * u8::max_value() as f32) as u8
}

fn norm_f32_as_u16(x: f32) -> u16 {
    (x * u16::max_value() as f32) as u16
}

fn u8_as_norm_f32(x: u8) -> f32 {
    x as f32 * (1.0 / u8::max_value() as f32)
}

fn u16_as_norm_f32(x: u16) -> f32 {
    x as f32 * (1.0 / u16::max_value() as f32)
}

fn u8_as_u16_norm(x: u8) -> u16 {
    x as u16 * 0x100
}

fn u16_as_u8_norm(x: u16) -> u8 {
    (x / 0x100) as u8
}
