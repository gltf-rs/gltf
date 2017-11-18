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

trait Normalizable<T> {
    fn normalize(self) -> T;
}

impl Normalizable<i8> for i8 {
    fn normalize(self) -> i8 { self }
}

impl Normalizable<u8> for i8 {
    fn normalize(self) -> u8 { self.max(0) as u8 * 2 }
}

impl Normalizable<i16> for i8 {
    fn normalize(self) -> i16 { self as i16 * 0x100 }
}

impl Normalizable<u16> for i8 {
    fn normalize(self) -> u16 { self.max(0) as u16 * 0x200 }
}

impl Normalizable<f32> for i8 {
    fn normalize(self) -> f32 { (self as f32 * 127.0_f32.recip()).max(-1.0) }
}

impl Normalizable<i8> for u8 {
    fn normalize(self) -> i8 { (self / 2) as i8 }
}

impl Normalizable<u8> for u8 {
    fn normalize(self) -> u8 { self }
}

impl Normalizable<i16> for u8 {
    fn normalize(self) -> i16 { self as i16 * 0x80 }
}

impl Normalizable<u16> for u8 {
    fn normalize(self) -> u16 { self.max(0) as u16 * 2 }
}

impl Normalizable<f32> for u8 {
    fn normalize(self) -> f32 { (self as f32 * 32767.0_f32.recip()).max(-1.0) }
}

impl Normalizable<i8> for i16 {
    fn normalize(self) -> i8 { (self / 0x100) as i8 }
}

impl Normalizable<u8> for i16 {
    fn normalize(self) -> u8 { (self.max(0) / 0x80) as u8 }
}

impl Normalizable<i16> for i16 {
    fn normalize(self) -> i16 { self }
}

impl Normalizable<u16> for i16 {
    fn normalize(self) -> u16 { self.max(0) as u16 * 2 }
}

impl Normalizable<f32> for i16 {
    fn normalize(self) -> f32 { (self as f32 * 32767.0_f32.recip()).max(-1.0) }
}

impl Normalizable<i8> for u16 {
    fn normalize(self) -> i8 { (self / 0x200) as i8 }
}

impl Normalizable<u8> for u16 {
    fn normalize(self) -> u8 { (self / 0x100) as u8 }
}

impl Normalizable<i16> for u16 {
    fn normalize(self) -> i16 { (self / 2) as i16 }
}

impl Normalizable<u16> for u16 {
    fn normalize(self) -> u16 { self }
}

impl Normalizable<f32> for u16 {
    fn normalize(self) -> f32 { self as f32 * 65535.0_f32.recip() }
}

impl Normalizable<i8> for f32 {
    fn normalize(self) -> i8 { (self * 127.0) as i8 }
}

impl Normalizable<u8> for f32 {
    fn normalize(self) -> u8 { (self.max(0.0) * 255.0) as u8 }
}

impl Normalizable<i16> for f32 {
    fn normalize(self) -> i16 { (self * 32767.0) as i16 }
}

impl Normalizable<u16> for f32 {
    fn normalize(self) -> u16 { (self.max(0.0) * 65535.0) as u16 }
}

impl Normalizable<f32> for f32 {
    fn normalize(self) -> f32 { self }
}

impl<U, T> Normalizable<[T; 2]> for [U; 2] where U: Normalizable<T> + Copy {
    fn normalize(self) -> [T; 2] {
        [self[0].normalize(), self[1].normalize()]
    }
}

impl<U, T> Normalizable<[T; 3]> for [U; 3] where U: Normalizable<T> + Copy {
    fn normalize(self) -> [T; 3] {
        [self[0].normalize(), self[1].normalize(), self[2].normalize()]
    }
}

impl<U, T> Normalizable<[T; 4]> for [U; 4] where U: Normalizable<T> + Copy {
    fn normalize(self) -> [T; 4] {
        [self[0].normalize(), self[1].normalize(), self[2].normalize(), self[3].normalize()]
    }
}
