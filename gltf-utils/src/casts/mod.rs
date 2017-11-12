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
/// Casting iterator and accompanying types for `MorphWeights`.
pub mod morph_weights;
/// Casting iterator and accompanying types for `Rotations`.
pub mod rotations;

#[inline(always)]
fn norm_i8_as_f32(x: i8) -> f32 {
    (x as f32 * (1.0 / i8::max_value() as f32)).max(-1.0)
}

#[inline(always)]
fn norm_u8_as_u16(x: u8) -> u16 {
    x as u16 * 0x100
}

#[inline(always)]
fn norm_u8_as_f32(x: u8) -> f32 {
    x as f32 * (1.0 / u8::max_value() as f32)
}

#[inline(always)]
fn norm_i16_as_f32(x: i16) -> f32 {
    (x as f32 * (1.0 / i16::max_value() as f32)).max(-1.0)
}

#[inline(always)]
fn norm_u16_as_u8(x: u16) -> u8 {
    (x / 0x100) as u8
}

#[inline(always)]
fn norm_u16_as_f32(x: u16) -> f32 {
    x as f32 * (1.0 / u16::max_value() as f32)
}

#[inline(always)]
fn norm_f32_as_u8(x: f32) -> u8 {
    (x * u8::max_value() as f32) as u8
}

#[inline(always)]
fn norm_f32_as_u16(x: f32) -> u16 {
    (x * u16::max_value() as f32) as u16
}
