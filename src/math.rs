use std::ops;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
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
    pub fn identity() -> Mat4 {
        let mut ret = Mat4 { data: [0.0; 16] };
        for i in 0..4 {
            ret.data[(i * 4) + i] = 1.0;
        }
        ret
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        let mut ret = Mat4::identity();
        ret.data[(3 * 4) + 0] = x;
        ret.data[(3 * 4) + 1] = y;
        ret.data[(3 * 4) + 2] = z;
        ret
    }

    pub fn euler_angles(x: f32, y: f32, z: f32) -> Mat4 {
        let mut ret = Mat4::identity();
        let cx = x.cos();
        let cy = y.cos();
        let cz = z.cos();
        let sx = x.sin();
        let sy = y.sin();
        let sz = z.sin();
        ret.data[0] = cy * cz;
        ret.data[1] = sx * sy * cz - cx * sz;
        ret.data[2] = sx * sz + cx * sy * cz;
        ret.data[4] = cy * cz;
        ret.data[5] = cx * cz + sx * sy * sz;
        ret.data[6] = cx * sy * sz - sx * cz;
        ret.data[8] = -sy;
        ret.data[9] = sx * cy;
        ret.data[10] = cx * cy;

        ret
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        let mut ret = Mat4 { data: [0.0; 16] };
        ret.data[(0 * 4) + 0] = x;
        ret.data[(1 * 4) + 1] = y;
        ret.data[(2 * 4) + 2] = z;
        ret.data[(3 * 4) + 3] = 1.0;
        ret
    }

    pub fn perspective(aspect_ratio: f32, fov: f32, near_plane: f32, far_plane: f32) -> Mat4 {
        let mut ret = Mat4 { data: [0.0; 16] };
        ret.data[0] = 1.0 / (aspect_ratio * (fov / 2.0).tan());
        ret.data[(1 * 4) + 1] = 1.0 / (fov / 2.0).tan();
        ret.data[(2 * 4) + 2] = -1.0 * (far_plane + near_plane) / (far_plane - near_plane);
        ret.data[(2 * 4) + 3] = (-2.0 * far_plane * near_plane) / (far_plane - near_plane);
        ret.data[(3 * 4) + 2] = -1.0;
        ret
    }

    pub fn translation_part(self) -> Vector3 {
        Vector3 {
            x: self.data[(3 * 4) + 0],
            y: self.data[(3 * 4) + 1],
            z: self.data[(3 * 4) + 2],
        }
    }

    // (note: amoussa) this was uh "adapted" from GLU :)
    pub fn inverse(self) -> Option<Mat4> {
        let mut ret = Mat4::default();

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
                ret.data[(i * 4) + j] = self.data[(j * 4) + i];
            }
        }
        ret
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
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
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
        let mut c = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                c.data[(i * 4) + j] = (0..4)
                    .map(|k| self.data[(i * 4) + k] * rhs.data[(k * 4) + j])
                    .sum();
            }
        }
        c
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
            vec4_out[i] = (0..4).map(|k| self.data[(k * 4) + i] * vec4_rhs[k]).sum();
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
                ret.data[(i * 3) + j] = item.data[(i * 4) + j];
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
            out[i] = (0..3).map(|k| self.data[(k * 3) + i] * rhs_data[k]).sum();
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
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const RED: Color = Color { r: 255, g: 0, b: 0 };

    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };

    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

    pub fn to_vector3(self) -> Vector3 {
        Vector3 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
        }
    }
}
