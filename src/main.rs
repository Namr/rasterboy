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
const FAR: f32 = 100.0;
const AMBIENT_STRENGTH: f32 = 0.3;

fn main() {
    ///////////////////////////////////////////////////
    // Load scene from disk
    ///////////////////////////////////////////////////
    let teapot = Mesh::from_obj_file(Path::new("data/teapot.obj"));
    let light = Vector3 {
        x: 20.0,
        y: -10.0,
        z: 2.0,
    };
    let light_color = Vector3 {
        x: 0.8,
        y: 0.8,
        z: 0.8,
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

    let model_mat = Mat4::euler_angles(0.0, 0.4, 0.0) * Mat4::translation(0.0, -1.5, -30.0);
    let projection_mat = Mat4::perspective(
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
        54_f32.to_radians(),
        NEAR,
        FAR,
    );

    for t in teapot.face_indicies {
        let world_to_v0 = model_mat * teapot.verticies[t.a];
        let world_to_v1 = model_mat * teapot.verticies[t.b];
        let world_to_v2 = model_mat * teapot.verticies[t.c];

        let mut ndc_v0 = projection_mat * world_to_v0;
        let mut ndc_v1 = projection_mat * world_to_v1;
        let mut ndc_v2 = projection_mat * world_to_v2;

        let normal =
            Vector3::cross(&(world_to_v2 - world_to_v0), &(world_to_v1 - world_to_v0)).normalized();

        // if any points are on screen, lets rasterize, we also perform back-face culling here
        if Vector3::dot(&world_to_v0, &normal) <= 0.0
            && (is_on_screen(&ndc_v0, NEAR, FAR)
                || is_on_screen(&ndc_v1, NEAR, FAR)
                || is_on_screen(&ndc_v2, NEAR, FAR))
        {
            // screen coords
            let pixel_v0 = ndc_v0.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);
            let pixel_v1 = ndc_v1.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);
            let pixel_v2 = ndc_v2.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);

            // pre-compute inverse depth before loop
            ndc_v0.z = 1.0 / ndc_v0.z;
            ndc_v1.z = 1.0 / ndc_v1.z;
            ndc_v2.z = 1.0 / ndc_v2.z;

            // compute color of each vertex
            // calc direction to light source
            let v0_to_light = (light - world_to_v0).normalized();
            let v1_to_light = (light - world_to_v1).normalized();
            let v2_to_light = (light - world_to_v2).normalized();

            let ambient = light_color * AMBIENT_STRENGTH;
            let c0 = (light_color * f32::max(Vector3::dot(&normal, &v0_to_light), 0.0)) + ambient;
            let c1 = (light_color * f32::max(Vector3::dot(&normal, &v1_to_light), 0.0)) + ambient;
            let c2 = (light_color * f32::max(Vector3::dot(&normal, &v2_to_light), 0.0)) + ambient;

            let area = triangle_edge(&pixel_v2, &pixel_v0, &pixel_v1);

            // axis aligned bounding box of triangle (clipped to match screen)
            let x_start = max(min(min(pixel_v0.x, pixel_v1.x), pixel_v2.x), 0);
            let x_end = min(max(max(pixel_v0.x, pixel_v1.x), pixel_v2.x), IMAGE_WIDTH);
            let y_start = max(min(min(pixel_v0.y, pixel_v1.y), pixel_v2.y), 0);
            let y_end = min(max(max(pixel_v0.y, pixel_v1.y), pixel_v2.y), IMAGE_HEIGHT);

            for x in x_start..x_end {
                for y in y_start..y_end {
                    let current_pixel = ScreenCoordinate { x, y };
                    let mut w0 = triangle_edge(&current_pixel, &pixel_v1, &pixel_v2);
                    let mut w1 = triangle_edge(&current_pixel, &pixel_v2, &pixel_v0);
                    let mut w2 = triangle_edge(&current_pixel, &pixel_v0, &pixel_v1);

                    let edge0 = ndc_v2 - ndc_v1;
                    let edge1 = ndc_v0 - ndc_v2;
                    let edge2 = ndc_v1 - ndc_v0;

                    // are we inside of a triangle? (also does a top left edge rule check)
                    if ((w0 == 0.0 && ((edge0.y == 0.0 && edge0.x > 0.0) || edge0.y > 0.0))
                        || w0 >= 0.0)
                        && ((w1 == 0.0 && ((edge1.y == 0.0 && edge1.x > 0.0) || edge1.y > 0.0))
                            || w1 >= 0.0)
                        && ((w2 == 0.0 && ((edge2.y == 0.0 && edge2.x > 0.0) || edge2.y > 0.0))
                            || w2 >= 0.0)
                    {
                        let buff_idx = ((y * IMAGE_WIDTH) + x) as usize;
                        w0 /= area;
                        w1 /= area;
                        w2 /= area;

                        // (note: amoussa) this is a very unintuitive formula I recommend reading about
                        // it here: https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html
                        let depth = ndc_v0.z * w0 + ndc_v1.z * w1 + ndc_v2.z * w2;

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
    ((point.x - v0.x) * (v0.y - v1.y) - (point.y - v0.y) * (v0.x - v1.x)) as f32
}

/*
 * Expects an NDC vertex
 */
pub fn is_on_screen(point: &Vector3, near: f32, far: f32) -> bool {
    point.z > near
        && point.z < far
        && point.x >= -1.0
        && point.x <= 1.0
        && point.y >= -1.0
        && point.y <= 1.0
}
