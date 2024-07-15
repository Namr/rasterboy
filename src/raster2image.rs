use rasterboy::image::*;
use rasterboy::scene::*;
use std::env;
use std::path::Path;

fn main() {
    // get path to scene and output file
    let help = "Invalid arguments. Usage is:\nraster2image [FILE...] [OPTION...]\n\nApplication Options:\n-o [OUTPUT_FILE]\t writes output to a file at the given path. Defaults to output.ppm";
    let mut args = env::args();
    if args.len() != 2 && args.len() != 4 {
        println!("{help}");
        return;
    }

    let mut output_file: String = "output.ppm".to_string();
    let mut input_file: String = String::default();
    args.next().expect(help); // skip program name
    loop {
        match args.next() {
            Some(path) => {
                if path == "-o" {
                    output_file = args.next().expect(help);
                } else {
                    input_file = path;
                }
            }
            None => {
                break;
            }
        }
    }

    // load scene from disk
    let scene = Scene::load_from_file(&input_file).expect("could not load scene file");

    // create color and depth buffers
    let image_width = scene.camera.canvas_width as usize;
    let image_height = scene.camera.canvas_height as usize;
    let num_pixels = image_width * image_height;
    let output_path = Path::new(&output_file);
    let mut output_image = Image::new(image_width, image_height);
    let mut depth_buffer = vec![f32::MAX; num_pixels];

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
