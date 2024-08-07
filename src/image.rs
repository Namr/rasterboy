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
        let split_size_line: Vec<&str> = size_line.split_whitespace().collect();
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
        let mut idx: usize = 0;
        for maybe_line in lines {
            let line = maybe_line?;
            let split_line: Vec<&str> = line.split_whitespace().collect();
            if split_line.len() % 3 != 0 {
                return Err(Box::new(PPMLoadError{msg: "the number of values in the PPM file is not a multiple of three (cannot create colors)".to_string()}));
            }

            for color_str in split_line.chunks(3) {
                data[idx].r = ((color_str[0].parse::<f32>()? / max_value) * 255.0) as u8;
                data[idx].g = ((color_str[1].parse::<f32>()? / max_value) * 255.0) as u8;
                data[idx].b = ((color_str[2].parse::<f32>()? / max_value) * 255.0) as u8;
                idx += 1;
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

    pub fn sample_bilinear(&self, u: f32, v: f32) -> Color {
        let max_x = self.width - 1;
        let max_y = self.height - 1;
        let v = 1.0 - v;

        let x_low_idx = ((u * max_x as f32).floor() as usize).clamp(0, max_x);
        let x_high_idx = ((u * max_x as f32).ceil() as usize).clamp(0, max_x);
        let y_low_idx = ((v * max_y as f32).floor() as usize).clamp(0, max_y);
        let y_high_idx = ((v * max_y as f32).ceil() as usize).clamp(0, max_y);

        // (note: amoussa) we need to add epsilon here to avoid a divide by zero in the case that
        // one axis is not being interpolated
        let x1 = (x_low_idx as f32 / max_x as f32) - f32::EPSILON;
        let x2 = (x_high_idx as f32 / max_x as f32) + f32::EPSILON;
        let y1 = (y_low_idx as f32 / max_y as f32) - f32::EPSILON;
        let y2 = (y_high_idx as f32 / max_y as f32) + f32::EPSILON;

        let q11 = self.data[(y_low_idx * self.width) + x_low_idx].to_vector3();
        let q21 = self.data[(y_low_idx * self.width) + x_high_idx].to_vector3();
        let q12 = self.data[(y_high_idx * self.width) + x_low_idx].to_vector3();
        let q22 = self.data[(y_high_idx * self.width) + x_high_idx].to_vector3();

        // (note: amoussa) these names are stupid
        let range = (x2 - x1) * (y2 - y1);
        let temp1 = (x2 - u) / range;
        let temp2 = (u - x1) / range;
        let temp3 = q11 * temp1 + q21 * temp2;
        let temp4 = q12 * temp1 + q22 * temp2;

        (temp3 * (y2 - v) + temp4 * (v - y1)).to_color()
    }

    #[allow(dead_code)]
    pub fn sample_nearest_neighbor(&self, u: f32, v: f32) -> Color {
        let max_x = self.width - 1;
        let max_y = self.height - 1;
        let v = 1.0 - v;

        let nearest_x = ((u * max_x as f32).round() as usize).clamp(0, max_x);
        let nearest_y = ((v * max_y as f32).round() as usize).clamp(0, max_y);
        self.data[(nearest_y * self.width) + nearest_x]
    }
}
