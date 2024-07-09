/*
 * The following conventions are to be adhered to in this file:
 * Matrices are stored in column-major order
 * The coordinate system is right handed with +Y as up
 */
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub data: [f32; 16],
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mat3 {
    pub data: [f32; 9],
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ScreenCoordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(clippy::identity_op)]
#[allow(clippy::erasing_op)]
impl Mat4 {
    pub fn at(&self, col: usize, row: usize) -> &f32 {
        return &self.data[(row * 4) + col];
    }

    pub fn mut_at(&mut self, col: usize, row: usize) -> &mut f32 {
        return &mut self.data[(row * 4) + col];
    }

    pub fn identity() -> Mat4 {
        let mut ret = Mat4 { data: [0.0; 16] };
        for i in 0..4 {
            *ret.mut_at(i, i) = 1.0;
        }
        ret
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        let mut ret = Mat4::identity();
        *ret.mut_at(3, 0) = x;
        *ret.mut_at(3, 1) = y;
        *ret.mut_at(3, 2) = z;
        ret
    }

    pub fn euler_angles(roll: f32, pitch: f32, yaw: f32) -> Mat4 {
        let mut ret = Mat4::identity();
        let cb = roll.cos();
        let cp = pitch.cos();
        let ch = yaw.cos();
        let sb = roll.sin();
        let sp = pitch.sin();
        let sh = yaw.sin();

        *ret.mut_at(0, 0) = ch * cb + sh * sp * sb;
        *ret.mut_at(0, 1) = sb * cp;
        *ret.mut_at(0, 2) = -sh * cb + ch * sp * sb;

        *ret.mut_at(1, 0) = -ch * sb + sh * sp * cb;
        *ret.mut_at(1, 1) = cb * cp;
        *ret.mut_at(1, 2) = sb * sh + ch * sp * cb;

        *ret.mut_at(2, 0) = sh * cp;
        *ret.mut_at(2, 1) = -sp;
        *ret.mut_at(2, 2) = ch * cp;
        ret
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        let mut ret = Mat4 { data: [0.0; 16] };
        *ret.mut_at(0, 0) = x;
        *ret.mut_at(1, 1) = y;
        *ret.mut_at(2, 2) = z;
        *ret.mut_at(3, 3) = 1.0;
        ret
    }

    pub fn perspective(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Mat4 {
        let mut ret = Mat4 { data: [0.0; 16] };
        let tangent = (fov / 2.0).tan();

        *ret.mut_at(0, 0) = 1.0 / (aspect_ratio * tangent);
        *ret.mut_at(1, 1) = 1.0 / tangent;
        *ret.mut_at(2, 2) = -(near_plane + far_plane) / (far_plane - near_plane);
        *ret.mut_at(2, 3) = -1.0;
        *ret.mut_at(3, 2) = -(2.0 * far_plane * near_plane) / (far_plane - near_plane);
        ret
    }

    pub fn translation_part(self) -> Vector3 {
        Vector3 {
            x: *self.at(3, 0),
            y: *self.at(3, 1),
            z: *self.at(3, 2),
        }
    }

    // (note: amoussa) this was uh "adapted" from GLU :)
    pub fn inverse(self) -> Option<Mat4> {
        let mut ret = Mat4 { data: [0.0; 16] };

        ret.data[0] = self.data[5] * self.data[10] * self.data[15]
            - self.data[5] * self.data[11] * self.data[14]
            - self.data[9] * self.data[6] * self.data[15]
            + self.data[9] * self.data[7] * self.data[14]
            + self.data[13] * self.data[6] * self.data[11]
            - self.data[13] * self.data[7] * self.data[10];

        ret.data[4] = -self.data[4] * self.data[10] * self.data[15]
            + self.data[4] * self.data[11] * self.data[14]
            + self.data[8] * self.data[6] * self.data[15]
            - self.data[8] * self.data[7] * self.data[14]
            - self.data[12] * self.data[6] * self.data[11]
            + self.data[12] * self.data[7] * self.data[10];

        ret.data[8] = self.data[4] * self.data[9] * self.data[15]
            - self.data[4] * self.data[11] * self.data[13]
            - self.data[8] * self.data[5] * self.data[15]
            + self.data[8] * self.data[7] * self.data[13]
            + self.data[12] * self.data[5] * self.data[11]
            - self.data[12] * self.data[7] * self.data[9];

        ret.data[12] = -self.data[4] * self.data[9] * self.data[14]
            + self.data[4] * self.data[10] * self.data[13]
            + self.data[8] * self.data[5] * self.data[14]
            - self.data[8] * self.data[6] * self.data[13]
            - self.data[12] * self.data[5] * self.data[10]
            + self.data[12] * self.data[6] * self.data[9];

        ret.data[1] = -self.data[1] * self.data[10] * self.data[15]
            + self.data[1] * self.data[11] * self.data[14]
            + self.data[9] * self.data[2] * self.data[15]
            - self.data[9] * self.data[3] * self.data[14]
            - self.data[13] * self.data[2] * self.data[11]
            + self.data[13] * self.data[3] * self.data[10];

        ret.data[5] = self.data[0] * self.data[10] * self.data[15]
            - self.data[0] * self.data[11] * self.data[14]
            - self.data[8] * self.data[2] * self.data[15]
            + self.data[8] * self.data[3] * self.data[14]
            + self.data[12] * self.data[2] * self.data[11]
            - self.data[12] * self.data[3] * self.data[10];

        ret.data[9] = -self.data[0] * self.data[9] * self.data[15]
            + self.data[0] * self.data[11] * self.data[13]
            + self.data[8] * self.data[1] * self.data[15]
            - self.data[8] * self.data[3] * self.data[13]
            - self.data[12] * self.data[1] * self.data[11]
            + self.data[12] * self.data[3] * self.data[9];

        ret.data[13] = self.data[0] * self.data[9] * self.data[14]
            - self.data[0] * self.data[10] * self.data[13]
            - self.data[8] * self.data[1] * self.data[14]
            + self.data[8] * self.data[2] * self.data[13]
            + self.data[12] * self.data[1] * self.data[10]
            - self.data[12] * self.data[2] * self.data[9];

        ret.data[2] = self.data[1] * self.data[6] * self.data[15]
            - self.data[1] * self.data[7] * self.data[14]
            - self.data[5] * self.data[2] * self.data[15]
            + self.data[5] * self.data[3] * self.data[14]
            + self.data[13] * self.data[2] * self.data[7]
            - self.data[13] * self.data[3] * self.data[6];

        ret.data[6] = -self.data[0] * self.data[6] * self.data[15]
            + self.data[0] * self.data[7] * self.data[14]
            + self.data[4] * self.data[2] * self.data[15]
            - self.data[4] * self.data[3] * self.data[14]
            - self.data[12] * self.data[2] * self.data[7]
            + self.data[12] * self.data[3] * self.data[6];

        ret.data[10] = self.data[0] * self.data[5] * self.data[15]
            - self.data[0] * self.data[7] * self.data[13]
            - self.data[4] * self.data[1] * self.data[15]
            + self.data[4] * self.data[3] * self.data[13]
            + self.data[12] * self.data[1] * self.data[7]
            - self.data[12] * self.data[3] * self.data[5];

        ret.data[14] = -self.data[0] * self.data[5] * self.data[14]
            + self.data[0] * self.data[6] * self.data[13]
            + self.data[4] * self.data[1] * self.data[14]
            - self.data[4] * self.data[2] * self.data[13]
            - self.data[12] * self.data[1] * self.data[6]
            + self.data[12] * self.data[2] * self.data[5];

        ret.data[3] = -self.data[1] * self.data[6] * self.data[11]
            + self.data[1] * self.data[7] * self.data[10]
            + self.data[5] * self.data[2] * self.data[11]
            - self.data[5] * self.data[3] * self.data[10]
            - self.data[9] * self.data[2] * self.data[7]
            + self.data[9] * self.data[3] * self.data[6];

        ret.data[7] = self.data[0] * self.data[6] * self.data[11]
            - self.data[0] * self.data[7] * self.data[10]
            - self.data[4] * self.data[2] * self.data[11]
            + self.data[4] * self.data[3] * self.data[10]
            + self.data[8] * self.data[2] * self.data[7]
            - self.data[8] * self.data[3] * self.data[6];

        ret.data[11] = -self.data[0] * self.data[5] * self.data[11]
            + self.data[0] * self.data[7] * self.data[9]
            + self.data[4] * self.data[1] * self.data[11]
            - self.data[4] * self.data[3] * self.data[9]
            - self.data[8] * self.data[1] * self.data[7]
            + self.data[8] * self.data[3] * self.data[5];

        ret.data[15] = self.data[0] * self.data[5] * self.data[10]
            - self.data[0] * self.data[6] * self.data[9]
            - self.data[4] * self.data[1] * self.data[10]
            + self.data[4] * self.data[2] * self.data[9]
            + self.data[8] * self.data[1] * self.data[6]
            - self.data[8] * self.data[2] * self.data[5];

        let det = self.data[0] * ret.data[0]
            + self.data[1] * ret.data[4]
            + self.data[2] * ret.data[8]
            + self.data[3] * ret.data[12];

        if det == 0.0 {
            return None;
        }

        let det = 1.0 / det;

        for i in 0..16 {
            ret.data[i] *= det;
        }

        Some(ret)
    }

    pub fn transpose(self) -> Mat4 {
        let mut ret = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                *ret.mut_at(i, j) = *self.at(j, i);
            }
        }
        ret
    }

    pub fn look_at(eye: Vector3, center: Vector3, up: Vector3) -> Mat4 {
        let mut ret = Mat4::identity();

        let f = (center - eye).normalized();
        let s = Vector3::cross(f, up).normalized();
        let u = Vector3::cross(s, f);

        *ret.mut_at(0, 0) = s.x;
        *ret.mut_at(1, 0) = s.y;
        *ret.mut_at(2, 0) = s.z;

        *ret.mut_at(0, 1) = u.x;
        *ret.mut_at(1, 1) = u.y;
        *ret.mut_at(2, 1) = u.z;

        *ret.mut_at(0, 2) = f.x;
        *ret.mut_at(1, 2) = f.y;
        *ret.mut_at(2, 2) = f.z;

        *ret.mut_at(3, 0) = Vector3::dot(s, eye);
        *ret.mut_at(3, 1) = Vector3::dot(u, eye);
        *ret.mut_at(3, 2) = Vector3::dot(f, eye);

        ret
    }
}

impl Default for Mat4 {
    fn default() -> Mat4 {
        Mat4::identity()
    }
}

impl Mat3 {
    pub fn at(&self, col: usize, row: usize) -> &f32 {
        return &self.data[(row * 3) + col];
    }

    pub fn mut_at(&mut self, col: usize, row: usize) -> &mut f32 {
        return &mut self.data[(row * 3) + col];
    }
}

impl Vector3 {
    pub const ORIGIN: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn ndc_to_pixel(self, screen_width: i32, screen_height: i32) -> ScreenCoordinate {
        ScreenCoordinate {
            x: ((self.x + 1.0) * 0.5 * (screen_width as f32)) as i32,
            y: ((1.0 - self.y) * 0.5 * (screen_height as f32)) as i32,
        }
    }

    pub fn to_color(self) -> Color {
        Color {
            r: (self.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (self.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (self.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Vector3 {
        let mag = self.magnitude();
        if mag.abs() <= f32::EPSILON {
            Vector3::ORIGIN
        } else {
            Vector3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl ops::Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut c = Mat4 { data: [0.0; 16] };
        for i in 0..4 {
            for j in 0..4 {
                *c.mut_at(j, i) = (0..4).map(|k| *self.at(k, i) * *rhs.at(j, k)).sum();
            }
        }
        c
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// (note: amoussa) this is a bit sketchy because we internally promote the vec3 into a vec4
// since it makes sense in the context of geometric transformations.
// Perhaps Mat4 and Vector3 should be Transformation and Point respectively
#[allow(clippy::needless_range_loop)]
impl ops::Mul<Vector3> for Mat4 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        let vec4_rhs = [rhs.x, rhs.y, rhs.z, 1.0];
        let mut vec4_out = [0.0; 4];

        // 4x4 * 4x1
        for i in 0..4 {
            vec4_out[i] = (0..4).map(|k| *self.at(k, i) * vec4_rhs[k]).sum();
        }

        Vector3 {
            x: vec4_out[0] / vec4_out[3],
            y: vec4_out[1] / vec4_out[3],
            z: vec4_out[2] / vec4_out[3],
        }
    }
}

#[allow(clippy::needless_range_loop)]
impl From<Mat4> for Mat3 {
    fn from(item: Mat4) -> Mat3 {
        let mut ret = Mat3::default();

        for i in 0..3 {
            for j in 0..3 {
                *ret.mut_at(j, i) = *item.at(j, i);
            }
        }
        ret
    }
}

// TODO: probably worthwhile to add a Mat3 x Mat3 operator overload for completeness
// but it is unlikely to ever be used
#[allow(clippy::needless_range_loop)]
impl ops::Mul<Vector3> for Mat3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        let rhs_data = [rhs.x, rhs.y, rhs.z];
        let mut out = [0.0; 3];
        // 3x3 * 3x1
        for i in 0..3 {
            out[i] = (0..3).map(|k| *self.at(k, i) * rhs_data[k]).sum();
        }
        Vector3 {
            x: out[0],
            y: out[1],
            z: out[2],
        }
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Color {
    pub fn to_vector3(self) -> Vector3 {
        Vector3 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
        }
    }
}
