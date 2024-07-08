use crate::math::*;
use core::fmt;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct PPMLoadError {
    pub msg: String,
}
impl Error for PPMLoadError {}

impl fmt::Display for PPMLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed Loading PPM Image With Error {}", self.msg,)
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            data: vec![Color::default(); width * height],
            width,
            height,
        }
    }

    pub fn load_ppm(path: &Path) -> Result<Image, Box<dyn Error>> {
        // load in file line by line
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // parse header, assert P3
        if lines.next().ok_or(Box::new(PPMLoadError {
            msg: "PPM file did not contain header".to_string(),
        }))??
            != "P3"
        {
            return Err(Box::new(PPMLoadError {
                msg: "PPM File was not in P3 Format".to_string(),
            }));
        }

        // get width, height, max value from the header
        let size_line: String = lines.next().ok_or(Box::new(PPMLoadError {
            msg: "PPM file did not contain header".to_string(),
        }))??;
        let split_size_line: Vec<&str> = size_line.trim().split_whitespace().collect();
        let max_val_line: String = lines.next().ok_or(Box::new(PPMLoadError {
            msg: "PPM file did not contain header".to_string(),
        }))??;
        if split_size_line.len() != 2 {
            return Err(Box::new(PPMLoadError {
                msg: "PPM File did not contain two numbers to define size in the header"
                    .to_string(),
            }));
        }

        let width = split_size_line[0].parse::<usize>()?;
        let height = split_size_line[1].parse::<usize>()?;
        let max_value = max_val_line.trim().parse::<f32>()?;

        // allocate the pixel buffer
        let mut data = vec![Color::default(); width * height];

        // for all lines read and push data, we enforce that lines are multiples of three numbers
        for maybe_line in lines.skip(3) {
            let line = maybe_line?;
            let split_line: Vec<&str> = line.split_whitespace().collect();
            if split_line.len() % 3 != 0 {
                return Err(Box::new(PPMLoadError{msg: "the number of values in the PPM file is not a multiple of three (cannot create colors)".to_string()}));
            }

            for (i, color_str) in split_line.chunks(3).enumerate() {
                data[i].r = ((color_str[0].parse::<f32>()? / max_value) * 255.0) as u8;
                data[i].g = ((color_str[1].parse::<f32>()? / max_value) * 255.0) as u8;
                data[i].b = ((color_str[2].parse::<f32>()? / max_value) * 255.0) as u8;
            }
        }

        Ok(Image {
            data,
            width,
            height,
        })
    }

    pub fn save_to_ppm(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut output_str: String = String::default();
        for pixel in self.data.iter() {
            output_str.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
        }

        let mut output_file = File::create(path)?;
        let ppm_header = format!("P3 {} {}\n255\n", self.width, self.height);
        output_file.write_all(ppm_header.as_bytes())?;

        let mut output_str: String = String::default();
        for pixel in self.data.iter() {
            output_str.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
        }

        // write to file and catch error
        output_file.write_all(output_str.as_bytes())?;

        Ok(())
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let nearest_x = (u * self.width as f32) as usize;
        let nearest_y = (v * self.height as f32) as usize;

        // TODO: bilinear interpolation
        self.data[(nearest_y * self.width) + nearest_x]
    }
}
