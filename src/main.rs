use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod math;
mod mesh;
mod test;

use math::*;
use mesh::*;

const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = 1080;
const NUM_PIXELS: usize = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
const NEAR: f32 = 0.1;
const FAR: f32 = 150.0;
const AMBIENT_STRENGTH: f32 = 0.1;

fn main() {
    ///////////////////////////////////////////////////
    // Load scene from disk
    ///////////////////////////////////////////////////
    let teapot = Mesh::from_obj_file(Path::new("data/bunny.obj"));
    let light = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 3.0,
    };
    let light_color = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    ///////////////////////////////////////////////////
    // Create a PPM file to contain our image output //
    //////////////////////////////////////////////////
    let path = Path::new("output.ppm");
    let display = path.display();

    let mut output_file = match File::create(&path) {
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

    let model_mat = Mat4::euler_angles(0.0, 1.3, 0.0) * Mat4::translation(0.0, -0.0, -1.4);
    let projection_mat = Mat4::perspective(
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
        54_f32.to_radians(),
        NEAR,
        FAR,
    );

    for t in teapot.face_indicies {
        let v0 = model_mat * projection_mat * teapot.verticies[t.a];
        let v1 = model_mat * projection_mat * teapot.verticies[t.b];
        let v2 = model_mat * projection_mat * teapot.verticies[t.c];

        let normal = Vector3::cross(&(v2 - v0), &(v1 - v0)).normalized();

        // if any points are on screen, lets rasterize
        if normal.z >= 0.0
            && (is_on_screen(&v0, NEAR, FAR)
                || is_on_screen(&v1, NEAR, FAR)
                || is_on_screen(&v2, NEAR, FAR))
        {
            // compute color of each vertex
            let ambient = light_color * AMBIENT_STRENGTH;
            let c0 = (light_color
                * f32::max(Vector3::dot(&normal, &((light - v0).normalized())), 0.0))
                + ambient;
            let c1 = (light_color
                * f32::max(Vector3::dot(&normal, &((light - v1).normalized())), 0.0))
                + ambient;
            let c2 = (light_color
                * f32::max(Vector3::dot(&normal, &((light - v2).normalized())), 0.0))
                + ambient;

            // screen coords
            let v0_s = v0.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);
            let v1_s = v1.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);
            let v2_s = v2.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);
            let area = triangle_edge(&v1_s, &v2_s, &v0_s);

            // axis aligned bounding box of triangle (clipped to match screen)
            let x_start = max(min(min(v0_s.x, v1_s.x), v2_s.x), 0);
            let x_end = min(max(max(v0_s.x, v1_s.x), v2_s.x), IMAGE_WIDTH);
            let y_start = max(min(min(v0_s.y, v1_s.y), v2_s.y), 0);
            let y_end = min(max(max(v0_s.y, v1_s.y), v2_s.y), IMAGE_HEIGHT);

            for x in x_start..x_end {
                for y in y_start..y_end {
                    let current_pixel = ScreenCoordinate { x, y };
                    let mut w0 = triangle_edge(&current_pixel, &v0_s, &v1_s);
                    let mut w1 = triangle_edge(&current_pixel, &v1_s, &v2_s);
                    let mut w2 = triangle_edge(&current_pixel, &v2_s, &v0_s);

                    // are we inside of a triangle?
                    if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                        let buff_idx = ((y * IMAGE_WIDTH) + x) as usize;
                        w0 /= area;
                        w1 /= area;
                        w2 /= area;

                        // (note: amoussa) this is a very unintuitive formula I recommend reading about
                        // it here: https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html
                        let depth = 1.0 / (v0.z * w0 + v1.z * w1 + v2.z + w2);

                        // depth test
                        if depth < depth_buffer[buff_idx] {
                            depth_buffer[buff_idx] = depth;
                            let color = (c0 * w0 + c1 * w1 + c2 * w2).to_color();
                            pixel_buffer[buff_idx] = color;
                        }
                    }
                }
            }
        }
    }

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
pub fn triangle_edge(
    point: &ScreenCoordinate,
    v0: &ScreenCoordinate,
    v1: &ScreenCoordinate,
) -> f32 {
    return ((point.x - v0.x) * (v0.y - v1.y) - (point.y - v0.y) * (v0.x - v1.x)) as f32;
}

/*
 * Expects an NDC vertex
 */
pub fn is_on_screen(point: &Vector3, near: f32, far: f32) -> bool {
    return point.z > near
        && point.z < far
        && point.x > -1.0
        && point.x < 1.0
        && point.y > -1.0
        && point.y < 1.0;
}
