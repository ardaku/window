// Window
// Copyright © 2019-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

/// A 4x4 transformation matrix.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Transform {
    mat: [[f32; 4]; 4],
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    /// Create a new identity matrix (transform that does nothing).
    pub fn new() -> Self {
        Self::from_mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Create a new Transform from a 4x4 matrix.
    pub fn from_mat4(mat: [[f32; 4]; 4]) -> Self {
        Self { mat }
    }

    /// Scale transformation (make biggger or smaller).
    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.mat[0][0] *= x;
        self.mat[1][1] *= y;
        self.mat[2][2] *= z;
        self
    }

    /// Translate (move) transformation.
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.mat[3][0] += x;
        self.mat[3][1] += y;
        self.mat[3][2] += z;
        self
    }

    /// Rotate transformation.  Parameters are quaternion in axis-angle form.
    /// - `x`: axis-vector x.
    /// - `y`: axis-vector y.
    /// - `z`: axis-vector z.
    /// - `c`: angle in cycles.
    pub fn rotate(self, x: f32, y: f32, z: f32, cycles: f32) -> Self {
        // Step 1. Normalize xyz rotation vector.
        let length = ((x * x) + (y * y) + (z * z)).sqrt();
        let (x, y, z) = (x / length, y / length, z / length);

        // Step 2. Get quaternion vector.
        let angle = cycles * std::f32::consts::PI;
        let scalar = angle.sin();
        let (x, y, z) = (x * scalar, y * scalar, z * scalar);

        // Step 3. Get quaternion scalar.
        let scalar = angle.cos();

        // Step 4. Convert quaternion into matrix.
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx2 = x2 * x;
        let xy2 = x2 * y;
        let xz2 = x2 * z;

        let yy2 = y2 * y;
        let yz2 = y2 * z;
        let zz2 = z2 * z;

        let sy2 = y2 * scalar;
        let sz2 = z2 * scalar;
        let sx2 = x2 * scalar;

        #[rustfmt::skip]
        let a = Self {
            mat: [
                [1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.0],
                [xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2, 0.0],
                [xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        self * a
    }

    pub(crate) fn as_ptr(&self) -> *const f32 {
        self.mat[0].as_ptr()
    }
}

impl std::ops::Mul<[f32; 3]> for Transform {
    type Output = [f32; 3];

    fn mul(self, vertex: [f32; 3]) -> Self::Output {
        [
            self.mat[0][0] * vertex[0]
                + self.mat[1][0] * vertex[1]
                + self.mat[2][0] * vertex[2]
                + self.mat[3][0],
            self.mat[0][1] * vertex[0]
                + self.mat[1][1] * vertex[1]
                + self.mat[2][1] * vertex[2]
                + self.mat[3][1],
            self.mat[0][2] * vertex[0]
                + self.mat[1][2] * vertex[1]
                + self.mat[2][2] * vertex[2]
                + self.mat[3][2],
        ]
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            mat: [
                [
                    (self.mat[0][0] * rhs.mat[0][0])
                        + (self.mat[0][1] * rhs.mat[1][0])
                        + (self.mat[0][2] * rhs.mat[2][0])
                        + (self.mat[0][3] * rhs.mat[3][0]),
                    (self.mat[0][0] * rhs.mat[0][1])
                        + (self.mat[0][1] * rhs.mat[1][1])
                        + (self.mat[0][2] * rhs.mat[2][1])
                        + (self.mat[0][3] * rhs.mat[3][1]),
                    (self.mat[0][0] * rhs.mat[0][2])
                        + (self.mat[0][1] * rhs.mat[1][2])
                        + (self.mat[0][2] * rhs.mat[2][2])
                        + (self.mat[0][3] * rhs.mat[3][2]),
                    (self.mat[0][0] * rhs.mat[0][3])
                        + (self.mat[0][1] * rhs.mat[1][3])
                        + (self.mat[0][2] * rhs.mat[2][3])
                        + (self.mat[0][3] * rhs.mat[3][3]),
                ],
                [
                    (self.mat[1][0] * rhs.mat[0][0])
                        + (self.mat[1][1] * rhs.mat[1][0])
                        + (self.mat[1][2] * rhs.mat[2][0])
                        + (self.mat[1][3] * rhs.mat[3][0]),
                    (self.mat[1][0] * rhs.mat[0][1])
                        + (self.mat[1][1] * rhs.mat[1][1])
                        + (self.mat[1][2] * rhs.mat[2][1])
                        + (self.mat[1][3] * rhs.mat[3][1]),
                    (self.mat[1][0] * rhs.mat[0][2])
                        + (self.mat[1][1] * rhs.mat[1][2])
                        + (self.mat[1][2] * rhs.mat[2][2])
                        + (self.mat[1][3] * rhs.mat[3][2]),
                    (self.mat[1][0] * rhs.mat[0][3])
                        + (self.mat[1][1] * rhs.mat[1][3])
                        + (self.mat[1][2] * rhs.mat[2][3])
                        + (self.mat[1][3] * rhs.mat[3][3]),
                ],
                [
                    (self.mat[2][0] * rhs.mat[0][0])
                        + (self.mat[2][1] * rhs.mat[1][0])
                        + (self.mat[2][2] * rhs.mat[2][0])
                        + (self.mat[2][3] * rhs.mat[3][0]),
                    (self.mat[2][0] * rhs.mat[0][1])
                        + (self.mat[2][1] * rhs.mat[1][1])
                        + (self.mat[2][2] * rhs.mat[2][1])
                        + (self.mat[2][3] * rhs.mat[3][1]),
                    (self.mat[2][0] * rhs.mat[0][2])
                        + (self.mat[2][1] * rhs.mat[1][2])
                        + (self.mat[2][2] * rhs.mat[2][2])
                        + (self.mat[2][3] * rhs.mat[3][2]),
                    (self.mat[2][0] * rhs.mat[0][3])
                        + (self.mat[2][1] * rhs.mat[1][3])
                        + (self.mat[2][2] * rhs.mat[2][3])
                        + (self.mat[2][3] * rhs.mat[3][3]),
                ],
                [
                    (self.mat[3][0] * rhs.mat[0][0])
                        + (self.mat[3][1] * rhs.mat[1][0])
                        + (self.mat[3][2] * rhs.mat[2][0])
                        + (self.mat[3][3] * rhs.mat[3][0]),
                    (self.mat[3][0] * rhs.mat[0][1])
                        + (self.mat[3][1] * rhs.mat[1][1])
                        + (self.mat[3][2] * rhs.mat[2][1])
                        + (self.mat[3][3] * rhs.mat[3][1]),
                    (self.mat[3][0] * rhs.mat[0][2])
                        + (self.mat[3][1] * rhs.mat[1][2])
                        + (self.mat[3][2] * rhs.mat[2][2])
                        + (self.mat[3][3] * rhs.mat[3][2]),
                    (self.mat[3][0] * rhs.mat[0][3])
                        + (self.mat[3][1] * rhs.mat[1][3])
                        + (self.mat[3][2] * rhs.mat[2][3])
                        + (self.mat[3][3] * rhs.mat[3][3]),
                ],
            ],
        }
    }
}

// SIMD-Multiply (From https://codereview.stackexchange.com/questions/101144/simd-matrix-multiplication)
/* void dotFourByFourMatrix(const Mat4* left, const Mat4* right, Mat4* result) {
    const __m128 BCx = _mm_load_ps((float*)&B.Row0);
    const __m128 BCy = _mm_load_ps((float*)&B.Row1);
    const __m128 BCz = _mm_load_ps((float*)&B.Row2);
    const __m128 BCw = _mm_load_ps((float*)&B.Row3);

    float* leftRowPointer = &left->Row0;
    float* resultRowPointer = &result->Row0;

    for (unsigned int i = 0; i < 4; ++i, leftRowPointer += 4, resultRowPointer += 4) {
        __m128 ARx = _mm_set1_ps(leftRowPointer[0]);
        __m128 ARy = _mm_set1_ps(leftRowPointer[1]);
        __m128 ARz = _mm_set1_ps(leftRowPointer[2]);
        __m128 ARw = _mm_set1_ps(leftRowPointer[3]);

        __m128 X = ARx * BCx;
        __m128 Y = ARy * BCy;
        __m128 Z = ARz * BCz;
        __m128 W = ARw * BCw;

        __m128 R = X + Y + Z + W;
        _mm_store_ps(resultRowPointer, R);
    }
}*/
