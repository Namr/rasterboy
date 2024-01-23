use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod math;
mod mesh;
mod rasterizer;
mod scene;

#[cfg(test)]
mod test;

use math::*;
use mesh::*;
use rasterizer::*;
use scene::*;

const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = 1080;
const NUM_PIXELS: usize = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
const NEAR: f32 = 0.1;
const FAR: f32 = 100.0;

fn main() {
    ///////////////////////////////////////////////////
    // Load scene from disk
    ///////////////////////////////////////////////////
    let teapot = Mesh::from_obj_file(Path::new("data/teapot.obj")).expect("Couldn't load obj file");
    let light1 = Light {
        position: Vector3 {
            x: 20.0,
            y: -10.0,
            z: 2.0,
        },
        color: Color {
            r: 230,
            g: 230,
            b: 230,
        },
        ambient_strength: 0.3,
    };

    let light2 = Light {
        position: Vector3 {
            x: -20.0,
            y: 10.0,
            z: 2.0,
        },
        color: Color { r: 50, g: 0, b: 0 },
        ambient_strength: 0.3,
    };
    let lights = vec![light1, light2];

    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, 54_f32.to_radians(), NEAR, FAR);

    ///////////////////////////////////////////////////
    // Create a PPM file to contain our image output //
    //////////////////////////////////////////////////
    let path = Path::new("output.ppm");
    let display = path.display();

    let mut output_file = match File::create(path) {
        Err(why) => panic!("Couldn't create output file {}: {}", display, why),
        Ok(file) => file,
    };

    let ppm_header = format!("P3 {IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    match output_file.write_all(ppm_header.as_bytes()) {
        Err(why) => panic!("Failed to write to output file {}: {}", display, why),
        Ok(_) => (),
    }

    let mut pixel_buffer = vec![Color::default(); NUM_PIXELS];
    let mut depth_buffer = vec![f32::MAX; NUM_PIXELS];

    let model_mat = Mat4::euler_angles(0.0, 0.4, 0.0) * Mat4::translation(0.0, -3.0, -40.0);
    let model_mat2 = Mat4::euler_angles(1.0, 0.4, 0.0) * Mat4::translation(0.0, 1.6, -40.0);

    draw_mesh(
        &teapot,
        model_mat,
        &lights,
        camera,
        &mut pixel_buffer,
        &mut depth_buffer,
    );
    draw_mesh(
        &teapot,
        model_mat2,
        &lights,
        camera,
        &mut pixel_buffer,
        &mut depth_buffer,
    );

    //////////////////////////////////////
    // Write framebuffer to image file //
    /////////////////////////////////////
    let mut output_str: String = String::default();
    for i in 0..NUM_PIXELS {
        output_str.push_str(&format!(
            "{} {} {}\n",
            pixel_buffer[i].r, pixel_buffer[i].g, pixel_buffer[i].b
        ));
    }
    match output_file.write_all(output_str.as_bytes()) {
        Err(why) => panic!("Failed to write pixel to output file {}: {}", display, why),
        Ok(_) => (),
    }
}

/*
 * This function determines which side of the line defined by v0 and v1 the the given point is on.
 * returns true if left of the line. v0 and v1 are intended to be provided in counter-clockwise order.
 */
pub fn triangle_edge(point: ScreenCoordinate, v0: ScreenCoordinate, v1: ScreenCoordinate) -> f32 {
    ((point.x - v0.x) * (v0.y - v1.y) - (point.y - v0.y) * (v0.x - v1.x)) as f32
}

/*
 * Expects an NDC vertex
 */
pub fn is_on_screen(point: Vector3, near: f32, far: f32) -> bool {
    point.z > near
        && point.z < far
        && point.x >= -1.0
        && point.x <= 1.0
        && point.y >= -1.0
        && point.y <= 1.0
}
