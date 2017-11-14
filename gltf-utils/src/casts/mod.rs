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

#[inline]
fn norm_i8_as_f32(x: i8) -> f32 {
    (x as f32 * (i8::max_value() as f32).recip()).max(-1.0)
}

#[inline]
fn norm_u8_as_u16(x: u8) -> u16 {
    x as u16 * 0x100
}

#[inline]
fn norm_u8_as_f32(x: u8) -> f32 {
    x as f32 * (u8::max_value() as f32).recip()
}

#[inline]
fn norm_i16_as_f32(x: i16) -> f32 {
    (x as f32 * (i16::max_value() as f32).recip()).max(-1.0)
}

#[inline]
fn norm_u16_as_u8(x: u16) -> u8 {
    (x / 0x100) as u8
}

#[inline]
fn norm_u16_as_f32(x: u16) -> f32 {
    x as f32 * (u16::max_value() as f32).recip()
}

#[inline]
fn norm_f32_as_u8(x: f32) -> u8 {
    let hi = u8::max_value() as f32;
    let lo = 0.0;
    (x * hi).max(lo).min(hi) as u8
}

#[inline]
fn norm_f32_as_u16(x: f32) -> u16 {
    let hi = u16::max_value() as f32;
    let lo = 0.0;
    (x * hi).max(lo).min(hi) as u16
}
