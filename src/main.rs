use std::path::Path;

mod image;
mod math;
mod mesh;
mod rasterizer;
mod scene;

#[cfg(test)]
mod test;

use image::*;
use scene::*;

fn main() {
    // load scene from disk
    let scene = Scene::load_from_file("data/example.xml").expect("could not load scene file");

    // create color and depth buffers
    let image_width = scene.camera.canvas_width as usize;
    let image_height = scene.camera.canvas_height as usize;
    let num_pixels = image_width * image_height;
    let output_path = Path::new("output.ppm");
    let mut output_image = Image::new(image_width, image_height);
    let mut depth_buffer = vec![f32::MAX; num_pixels as usize];

    // render
    scene.render(&mut output_image.data, &mut depth_buffer);

    // write image to disk
    if let Err(why) = output_image.save_to_ppm(output_path) {
        panic!(
            "Could not write output image to disk because of error: {}",
            why
        );
    }
}
