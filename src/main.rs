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
use scene::*;

fn main() {
    ///////////////////////////
    // Load scene from disk //
    //////////////////////////
    let scene = Scene::load_from_file("data/example.xml").expect("could not load scene file");

    ///////////////////////////////////////////////////
    // Create a PPM file to contain our image output //
    //////////////////////////////////////////////////
    let image_width = scene.camera.canvas_width as usize;
    let image_height = scene.camera.canvas_height as usize;
    let num_pixels = image_width * image_height;

    let path = Path::new("output.ppm");
    let display = path.display();

    let mut output_file = match File::create(path) {
        Err(why) => panic!("Couldn't create output file {}: {}", display, why),
        Ok(file) => file,
    };

    let ppm_header = format!("P3 {image_width} {image_height}\n255\n");

    if let Err(why) = output_file.write_all(ppm_header.as_bytes()) {
        panic!("Failed to write to output file {}: {}", display, why)
    }

    let mut pixel_buffer = vec![Color::default(); num_pixels as usize];
    let mut depth_buffer = vec![f32::MAX; num_pixels as usize];

    scene.render(&mut pixel_buffer, &mut depth_buffer);

    //////////////////////////////////////
    // Write framebuffer to image file //
    /////////////////////////////////////
    let mut output_str: String = String::default();
    for pixel in pixel_buffer.iter() {
        output_str.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
    }

    // write to file and catch error
    if let Err(why) = output_file.write_all(output_str.as_bytes()) {
        panic!("Failed to write pixel to output file {}: {}", display, why);
    }
}
