use crate::math::*;

static EPSILON: f32 = 0.00001;

#[test]
fn test_matrix_mul() {
    // I X I = I
    let a = Mat4::identity();
    let b = Mat4::identity();
    let c = a * b;

    assert_eq!(c, Mat4::identity());

    let a = Mat4::translation(1.0, 1.0, 1.0);
    let b = Mat4::translation(1.0, 1.0, 1.0);
    let c = a * b;
    let tp = c.translation_part();

    assert_eq!(
        tp,
        Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0
        }
    );
}

#[test]
fn test_point_transformations() {
    let t = Mat4::translation(1.0, 1.0, 1.0);
    let p = Vector3::ORIGIN;
    let tp = t * p;
    assert_eq!(
        tp,
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    );

    let t = Mat4::euler_angles(0.0, 0.0, 90_f32.to_radians());
    let p = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let tp = t * p;

    assert!(tp.x - 0.0 < EPSILON);
    assert!(tp.y - 1.0 < EPSILON);
    assert!(tp.z - 0.0 < EPSILON);

    let t = Mat4::translation(1.0, 0.0, 0.0)
        * Mat4::euler_angles(0.0, 0.0, 90_f32.to_radians())
        * Mat4::scale(2.0, 2.0, 2.0);
    let p = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let tp = t * p;

    assert!(tp.x - 1.0 < EPSILON);
    assert!(tp.y - 2.0 < EPSILON);
    assert!(tp.z - 0.0 < EPSILON);
}
