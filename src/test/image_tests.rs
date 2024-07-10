use crate::image::Image;
use crate::math::Color;

#[test]
fn test_nearest_neighbor_sample() {
    let mut texture = Image::new(4, 4);
    texture.data[0] = Color { r: 0, g: 0, b: 0 };
    texture.data[1] = Color { r: 1, g: 1, b: 1 };
    texture.data[2] = Color { r: 2, g: 2, b: 2 };
    texture.data[3] = Color { r: 3, g: 3, b: 3 };
    texture.data[4] = Color { r: 4, g: 4, b: 4 };
    texture.data[5] = Color { r: 5, g: 5, b: 5 };
    texture.data[6] = Color { r: 6, g: 6, b: 6 };
    texture.data[7] = Color { r: 7, g: 7, b: 7 };
    texture.data[8] = Color { r: 8, g: 8, b: 8 };
    texture.data[9] = Color { r: 9, g: 9, b: 9 };
    texture.data[10] = Color {
        r: 10,
        g: 10,
        b: 10,
    };
    texture.data[11] = Color {
        r: 11,
        g: 11,
        b: 11,
    };
    texture.data[12] = Color {
        r: 12,
        g: 12,
        b: 12,
    };
    texture.data[13] = Color {
        r: 13,
        g: 13,
        b: 13,
    };
    texture.data[14] = Color {
        r: 14,
        g: 14,
        b: 14,
    };
    texture.data[15] = Color {
        r: 15,
        g: 15,
        b: 15,
    };

    assert_eq!(
        texture.sample_nearest_neighbor(0.0, 0.0),
        Color {
            r: 12,
            g: 12,
            b: 12
        }
    );
    assert_eq!(
        texture.sample_nearest_neighbor(0.01, 0.01),
        Color {
            r: 12,
            g: 12,
            b: 12
        }
    );
    assert_eq!(
        texture.sample_nearest_neighbor(0.5, 0.5),
        Color {
            r: 10,
            g: 10,
            b: 10
        }
    );
    assert_eq!(
        texture.sample_nearest_neighbor(0.8, 0.8),
        Color { r: 6, g: 6, b: 6 }
    );
    assert_eq!(
        texture.sample_nearest_neighbor(1.0, 0.0),
        Color {
            r: 15,
            g: 15,
            b: 15
        }
    );
    assert_eq!(
        texture.sample_nearest_neighbor(0.0, 1.0),
        Color { r: 0, g: 0, b: 0 }
    );
}

#[test]
fn test_bilinear_sample() {
    let mut texture = Image::new(2, 2);
    texture.data[0] = Color { r: 0, g: 0, b: 0 };
    texture.data[1] = Color { r: 255, g: 0, b: 0 };
    texture.data[2] = Color { r: 0, g: 255, b: 0 };
    texture.data[3] = Color { r: 0, g: 0, b: 255 };

    // no interpolation
    assert_eq!(
        texture.sample_bilinear(0.0, 0.0),
        Color { r: 0, g: 255, b: 0 }
    );
    assert_eq!(
        texture.sample_bilinear(0.0, 1.0),
        Color { r: 0, g: 0, b: 0 }
    );
    assert_eq!(
        texture.sample_bilinear(1.0, 1.0),
        Color { r: 255, g: 0, b: 0 }
    );
    assert_eq!(
        texture.sample_bilinear(1.0, 0.0),
        Color { r: 0, g: 0, b: 255 }
    );

    // interpolation
    assert_eq!(
        texture.sample_bilinear(0.0, 0.3),
        Color { r: 0, g: 178, b: 0 }
    );
    assert_eq!(
        texture.sample_bilinear(0.3, 0.3),
        Color {
            r: 22,
            g: 124,
            b: 53
        }
    );
}
