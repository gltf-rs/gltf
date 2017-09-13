#![allow(dead_code)]

/// Re-exported with `feature = "mint-support"`.
#[cfg(feature = "mint-support")]
mod with_mint {
    /// 4x4 column-major matrix.
    pub type Mat4 = ::mint::ColumnMatrix4<f32>;

    /// Quaternion.
    pub type Quat = ::mint::Quaternion<f32>;

    /// 2D vector.
    pub type Vec2 = ::mint::Vector2<f32>;

    /// 3D vector.
    pub type Vec3 = ::mint::Vector3<f32>;

    /// 4D vector.
    pub type Vec4 = ::mint::Vector4<f32>;

    /// Constructor for 4x4 column-major matrix.
    pub(crate) fn mat4(m: [[f32; 4]; 4]) -> Mat4 {
        m.into()
    }

    /// Constructor for quaternion.
    pub(crate) fn quat(v: [f32; 3], s: f32) -> Quat {
        Quat {
            v: v.into(),
            s: s,
        }
    }

    /// Constructor for 2D vector.
    pub(crate) fn vec2(v: [f32; 2]) -> Vec2 {
        v.into()
    }

    /// Constructor for 3D vector.
    pub(crate) fn vec3(v: [f32; 3]) -> Vec3 {
        v.into()
    }

    /// Constructor for 4D vector.
    pub(crate) fn vec4(v: [f32; 4]) -> Vec4 {
        v.into()
    }
}

/// Re-exported with `not(feature = "mint-support")`.
#[cfg(not(feature = "mint-support"))]
mod without_mint {
    /// 4x4 column-major matrix.
    pub type Mat4 = [[f32; 4]; 4];

    /// Quaternion in the order `[x, y, z, w]`, where `w` is the scalar.
    pub type Quat = [f32; 4];
    
    /// 2D vector.
    pub type Vec2 = [f32; 2];

    /// 3D vector.
    pub type Vec3 = [f32; 3];

    /// 4D vector.
    pub type Vec4 = [f32; 4];

    /// Constructor for 4x4 column-major matrix.
    pub(crate) fn mat4(m: [[f32; 4]; 4]) -> Mat4 {
        m
    }

    /// Constructor for quaternion.
    pub(crate) fn quat(v: [f32; 3], s: f32) -> Quat {
        [v[0], v[1], v[2], s]
    }

    /// Constructor for 2D vector.
    pub(crate) fn vec2(v: [f32; 2]) -> Vec2 {
        v
    }

    /// Constructor for 3D vector.
    pub(crate) fn vec3(v: [f32; 3]) -> Vec3 {
        v
    }

    /// Constructor for 4D vector.
    pub(crate) fn vec4(v: [f32; 4]) -> Vec4 {
        v
    }
}

#[cfg(feature = "mint-support")]
pub use self::with_mint::*;
#[cfg(not(feature = "mint-support"))]
pub use self::without_mint::*;
