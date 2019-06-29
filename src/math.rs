// Copyright 2013-2014 The CGMath Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Modified for the gltf crate by the gltf library developers.

use std::ops;

#[cfg(test)]
mod test {
    use approx;
    use super::*;

    impl approx::AbsDiffEq for Vector4 {
        type Epsilon = f32;
        fn default_epsilon() -> f32 {
            f32::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Vector4, epsilon: Self::Epsilon) -> bool {
            f32::abs_diff_eq(&self.x, &other.x, epsilon)
                &&
                f32::abs_diff_eq(&self.y, &other.y, epsilon)
                &&
                f32::abs_diff_eq(&self.z, &other.z, epsilon)
                &&
                f32::abs_diff_eq(&self.w, &other.w, epsilon)
        }
    }

    impl approx::RelativeEq for Vector4 {
        fn default_max_relative() -> f32 {
            f32::default_max_relative()
        }

        fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
            f32::relative_eq(&self.x, &other.x, epsilon, max_relative)
                &&
                f32::relative_eq(&self.y, &other.y, epsilon, max_relative)
                &&
                f32::relative_eq(&self.z, &other.z, epsilon, max_relative)
                &&
                f32::relative_eq(&self.w, &other.w, epsilon, max_relative)
        }
    }

    impl approx::UlpsEq for Vector4 {
        fn default_max_ulps() -> u32 {
            f32::default_max_ulps()
        }

        fn ulps_eq(&self, other: &Self, epsilon: f32, max_ulps: u32) -> bool {
            f32::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
                &&
                f32::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
                &&
                f32::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
                &&
                f32::ulps_eq(&self.w, &other.w, epsilon, max_ulps)
        }
    }

    impl approx::AbsDiffEq for Matrix4 {
        type Epsilon = f32;
        fn default_epsilon() -> f32 {
            f32::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Matrix4, epsilon: Self::Epsilon) -> bool {
            Vector4::abs_diff_eq(&self.x, &other.x, epsilon)
                &&
                Vector4::abs_diff_eq(&self.y, &other.y, epsilon)
                &&
                Vector4::abs_diff_eq(&self.z, &other.z, epsilon)
                &&
                Vector4::abs_diff_eq(&self.w, &other.w, epsilon)
        }
    }

    impl approx::RelativeEq for Matrix4 {
        fn default_max_relative() -> f32 {
            f32::default_max_relative()
        }

        fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
            Vector4::relative_eq(&self.x, &other.x, epsilon, max_relative)
                &&
                Vector4::relative_eq(&self.y, &other.y, epsilon, max_relative)
                &&
                Vector4::relative_eq(&self.z, &other.z, epsilon, max_relative)
                &&
                Vector4::relative_eq(&self.w, &other.w, epsilon, max_relative)
        }
    }

    impl approx::UlpsEq for Matrix4 {
        fn default_max_ulps() -> u32 {
            f32::default_max_ulps()
        }

        fn ulps_eq(&self, other: &Self, epsilon: f32, max_ulps: u32) -> bool {
            Vector4::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
                &&
                Vector4::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
                &&
                Vector4::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
                &&
                Vector4::ulps_eq(&self.w, &other.w, epsilon, max_ulps)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn multiply(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    #[cfg(test)]
    pub fn normalize(self) -> Vector3 {
        self * (1.0 / self.magnitude())
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.multiply(rhs);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { x, y, z, w }
    }

    pub fn multiply(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl ops::Add for Vector4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.multiply(rhs);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32,
        c1r0: f32, c1r1: f32, c1r2: f32,
        c2r0: f32, c2r1: f32, c2r2: f32,
    ) -> Matrix3 {
        Matrix3 {
            x: Vector3::new(c0r0, c0r1, c0r2),
            y: Vector3::new(c1r0, c1r1, c1r2),
            z: Vector3::new(c2r0, c2r1, c2r2),
        }
    }

    pub fn determinant(&self) -> f32 {
        self.x.x * (self.y.y * self.z.z - self.z.y * self.y.z)
            - self.y.x * (self.x.y * self.z.z - self.z.y * self.x.z)
            + self.z.x * (self.x.y * self.y.z - self.y.y * self.x.z)
    }

    pub fn trace(&self) -> f32 {
        self.x.x + self.y.y + self.z.z
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix4 {
    pub fn new(
        c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
        c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
        c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
        c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32,
    ) -> Matrix4  {
        Matrix4 {
            x: Vector4::new(c0r0, c0r1, c0r2, c0r3),
            y: Vector4::new(c1r0, c1r1, c1r2, c1r3),
            z: Vector4::new(c2r0, c2r1, c2r2, c2r3),
            w: Vector4::new(c3r0, c3r1, c3r2, c3r3),
        }
    }

    #[cfg(test)]
    pub fn from_array(m: [[f32; 4]; 4]) -> Matrix4 {
        Matrix4::new(
            m[0][0], m[0][1], m[0][2], m[0][3],
            m[1][0], m[1][1], m[1][2], m[1][3],
            m[2][0], m[2][1], m[2][2], m[2][3],
            m[3][0], m[3][1], m[3][2], m[3][3],
        )
    }

    /// Create a homogeneous transformation matrix from a translation vector.
    pub fn from_translation(v: Vector3) -> Matrix4 {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            v.x, v.y, v.z, 1.0,
        )
    }

    /// Create a homogeneous transformation matrix from a set of scale values.
    pub fn from_nonuniform_scale(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::new(
              x, 0.0, 0.0, 0.0,
            0.0,   y, 0.0, 0.0,
            0.0, 0.0,   z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    /// Convert the quaternion to a 4 x 4 rotation matrix.
    pub fn from_quaternion(q: Quaternion) -> Matrix4 {
        let x2 = q.v.x + q.v.x;
        let y2 = q.v.y + q.v.y;
        let z2 = q.v.z + q.v.z;

        let xx2 = x2 * q.v.x;
        let xy2 = x2 * q.v.y;
        let xz2 = x2 * q.v.z;

        let yy2 = y2 * q.v.y;
        let yz2 = y2 * q.v.z;
        let zz2 = z2 * q.v.z;

        let sy2 = y2 * q.s;
        let sz2 = z2 * q.s;
        let sx2 = x2 * q.s;

        Matrix4::new(
            1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.0,
            xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2, 0.0,
            xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn as_array(&self) -> [[f32; 4]; 4] {
        [
            self.x.as_array(),
            self.y.as_array(),
            self.z.as_array(),
            self.w.as_array(),
        ]
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        let a = self.x;
        let b = self.y;
        let c = self.z;
        let d = self.w;
        Matrix4 {
            x: a * rhs.x.x + b * rhs.x.y + c * rhs.x.z + d * rhs.x.w,
            y: a * rhs.y.x + b * rhs.y.y + c * rhs.y.z + d * rhs.y.w,
            z: a * rhs.z.x + b * rhs.z.y + c * rhs.z.z + d * rhs.z.w,
            w: a * rhs.w.x + b * rhs.w.y + c * rhs.w.z + d * rhs.w.w,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Quaternion {
    pub s: f32,
    pub v: Vector3,
}

impl Quaternion {
    pub fn new(w: f32, xi: f32, yj: f32, zk: f32) -> Quaternion {
        Quaternion {
            s: w,
            v: Vector3::new(xi, yj, zk),
        }
    }

    #[cfg(test)]
    pub fn from_axis_angle(axis: Vector3, radians: f32) -> Quaternion {
        Quaternion {
            s: (0.5 * radians).cos(),
            v: axis * (0.5 * radians).sin(),
        }
    }

    /// Convert a rotation matrix to an equivalent quaternion.
    pub fn from_matrix(m: Matrix3) -> Quaternion {
        let trace = m.trace();
        if trace >= 0.0 {
            let s = (1.0 + trace).sqrt();
            let w = 0.5 * s;
            let s = 0.5 / s;
            let x = (m.y.z - m.z.y) * s;
            let y = (m.z.x - m.x.z) * s;
            let z = (m.x.y - m.y.x) * s;
            Quaternion::new(w, x, y, z)
        } else if (m.x.x > m.y.y) && (m.x.x > m.z.z) {
            let s = ((m.x.x - m.y.y - m.z.z) + 1.0).sqrt();
            let x = 0.5 * s;
            let s = 0.5 / s;
            let y = (m.y.x + m.x.y) * s;
            let z = (m.x.z + m.z.x) * s;
            let w = (m.y.z - m.z.y) * s;
            Quaternion::new(w, x, y, z)
        } else if m.y.y > m.z.z {
            let s = ((m.y.y - m.x.x - m.z.z) + 1.0).sqrt();
            let y = 0.5 * s;
            let s = 0.5 / s;
            let z = (m.z.y + m.y.z) * s;
            let x = (m.y.x + m.x.y) * s;
            let w = (m.z.x - m.x.z) * s;
            Quaternion::new(w, x, y, z)
        } else {
            let s = ((m.z.z - m.x.x - m.y.y) + 1.0).sqrt();
            let z = 0.5 * s;
            let s = 0.5 / s;
            let x = (m.x.z + m.z.x) * s;
            let y = (m.z.y + m.y.z) * s;
            let w = (m.x.y - m.y.x) * s;
            Quaternion::new(w, x, y, z)
        }
    }
}
