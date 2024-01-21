use std::ops;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub data: [f32; 16],
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
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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
        let c1 = x.cos();
        let c2 = y.cos();
        let c3 = z.cos();
        let s1 = x.sin();
        let s2 = y.sin();
        let s3 = z.sin();
        ret.data[0] = c1 * c3 - c2 * s1 * s3;
        ret.data[1] = -c1 * s3 - c2 * c3 * s1;
        ret.data[2] = s1 * s2;
        ret.data[4] = c3 * s1 + c1 * c2 * s3;
        ret.data[5] = c1 * c2 * c3 - s1 * s3;
        ret.data[6] = -c1 * s2;
        ret.data[8] = s2 * s3;
        ret.data[9] = c3 * s2;
        ret.data[10] = c2;

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

    pub fn translation_part(&self) -> Vector3 {
        Vector3 {
            x: self.data[(3 * 4) + 0],
            y: self.data[(3 * 4) + 1],
            z: self.data[(3 * 4) + 2],
        }
    }
}

impl Vector3 {
    pub const ORIGIN: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn ndc_to_pixel(&self, screen_width: i32, screen_height: i32) -> ScreenCoordinate {
        ScreenCoordinate {
            x: ((self.x + 1.0) * 0.5 * (screen_width as f32)) as i32,
            y: ((1.0 - self.y) * 0.5 * (screen_height as f32)) as i32,
        }
    }
}

// TODO: the operator overloads here copy the entire mat, which seems super expensive
// somehow it is not trivial to pass by reference in operator overloads
impl ops::Mul<Mat4> for Mat4 {
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

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
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
