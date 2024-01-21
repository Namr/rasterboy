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
const VERT_SQUARE_SIZE: i32 = 4;

fn main() {
    ///////////////////////////////////////////////////
    // Load scene from disk
    ///////////////////////////////////////////////////
    let teapot = Mesh::from_obj_file(Path::new("data/teapot.obj"));

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

    let mut pixel_buffer: [Pixel; NUM_PIXELS] = [Pixel::default(); NUM_PIXELS];

    let model_mat = Mat4::translation(0.0, -1.0, -20.0);
    let projection_mat = Mat4::perspective(
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32,
        54_f32.to_radians(),
        NEAR,
        FAR,
    );
    for v in teapot.verticies {
        let v_prime = model_mat * projection_mat * v;

        // plane clipping
        if v_prime.z > NEAR && v_prime.z < FAR {
            let v_s = v_prime.ndc_to_pixel(IMAGE_WIDTH, IMAGE_HEIGHT);
            // ensure we are on screen
            if v_s.x > 0 && v_s.x < IMAGE_WIDTH && v_s.y > 0 && v_s.y < IMAGE_HEIGHT {
                let x_start = std::cmp::max(v_s.x - VERT_SQUARE_SIZE, 0);
                let x_end = std::cmp::min(v_s.x + VERT_SQUARE_SIZE, IMAGE_WIDTH - 1);
                let y_start = std::cmp::max(v_s.y - VERT_SQUARE_SIZE, 0);
                let y_end = std::cmp::min(v_s.y + VERT_SQUARE_SIZE, IMAGE_HEIGHT - 1);

                for x in x_start..x_end {
                    for y in y_start..y_end {
                        pixel_buffer[((y * IMAGE_WIDTH) + x) as usize].r = 255;
                        pixel_buffer[((y * IMAGE_WIDTH) + x) as usize].g = 255;
                        pixel_buffer[((y * IMAGE_WIDTH) + x) as usize].b = 255;
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
