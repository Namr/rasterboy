use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use mesh::Mesh;

mod math;
mod mesh;
mod test;

static IMAGE_WIDTH: i32 = 1920;
static IMAGE_HEIGHT: i32 = 1080;
static OUTER_CIRCLE_RADIUS: i32 = 500;
static INNER_CIRCLE_RADIUS: i32 = 450;
static CENTER_X: i32 = IMAGE_WIDTH / 2;
static CENTER_Y: i32 = IMAGE_HEIGHT / 2;

fn main() {
    let teapot = Mesh::from_obj_file(Path::new("data/teapot.obj"));
    println!("Teapot loaded: {:?}", teapot);

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

    ////////////////////////////////////////////////
    // Compute and commit the color of each pixel //
    ///////////////////////////////////////////////
    for y in 0..IMAGE_WIDTH {
        for x in 0..IMAGE_HEIGHT {
            let square = i32::pow(x - CENTER_X, 2) + i32::pow(y - CENTER_Y, 2);
            let pixel = if square < i32::pow(OUTER_CIRCLE_RADIUS, 2)
                && square > i32::pow(INNER_CIRCLE_RADIUS, 2)
            {
                "0 0 0\n"
            } else {
                "255 255 255\n"
            };

            match output_file.write_all(pixel.as_bytes()) {
                Err(why) => panic!("Failed to write pixel to output file {}: {}", display, why),
                Ok(_) => (),
            }
        }
    }
}
