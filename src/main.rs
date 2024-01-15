use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

static IMAGE_WIDTH: i32 = 1920;
static IMAGE_HEIGHT: i32 = 1080;
static OUTER_CIRCLE_RADIUS: i32 = 500;
static INNER_CIRCLE_RADIUS: i32 = 450;
static CENTER_X: i32 =  IMAGE_WIDTH / 2;
static CENTER_Y: i32 =  IMAGE_HEIGHT / 2;

fn main() {
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
    
    for y in 0..1080 {
        for x in 0..1920 {
            let square = i32::pow(x - CENTER_X, 2) + i32::pow(y - CENTER_Y, 2);
            let pixel = if square < i32::pow(OUTER_CIRCLE_RADIUS, 2) && square > i32::pow(INNER_CIRCLE_RADIUS, 2) {
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
