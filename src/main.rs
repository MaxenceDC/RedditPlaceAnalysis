#![allow(dead_code)]

extern crate colored;
extern crate image;
extern crate regex;

use colored::Colorize;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::Instant;
use regex::Regex;

const LINE_NUMBER: f64 = 160_353_105.0;
const HASH_6MAXENCE: &str =
    "bCrZRP7T31V14qwiWNzeBDKckEr+7q5aWwtYi/xnGSI57DwO4pWc5Ce1axjS3yNhF9wvmA2THtL/lwbIIeF69A==";
const DATA_PATH: &str = "data/rplace.csv";
// const DATA_PATH: &str = "data/sampleplace.csv";
const IMAGE_PATH: &str = "img/rplace.png";
const IMAGE_DIM: (u32, u32) = (2000, 2000);

struct Pixel {
    x: u32,
    y: u32,
    color: String,
}

impl Pixel {
    fn new(x: u32, y: u32, color: String) -> Pixel {
        let reghex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
        if !reghex.is_match(&color) {
            panic!("Invalid color: {}", color);
        }
        Pixel { x, y, color }
    }
}

fn main() -> io::Result<()> {
    let file = File::open(DATA_PATH).unwrap();
    let reader = BufReader::new(file);
    let mut imgbuf: RgbaImage = ImageBuffer::new(IMAGE_DIM.0, IMAGE_DIM.1);
    let mut placed_pixels: Vec<Pixel> = Vec::new();
    let mut line_number = 0;
    let start = Instant::now();
    
    for i in 0..IMAGE_DIM.0 {
        for j in 0..IMAGE_DIM.1 {
            imgbuf.put_pixel(i, j, Rgba([0, 0, 0, 0]));
        }
    }
    imgbuf.save(IMAGE_PATH).unwrap();

    for (counter, line) in reader.lines().into_iter().enumerate() {
        if counter % 500_000 == 0 {
            io::stdout().flush()?;
            let percentage = counter as f64 / LINE_NUMBER * 100.0;
            let r = 255.0 * (1.0 - percentage / 100.0);
            let g = 255.0 * (percentage / 100.0);
            print!(
                "\r{}% processed.",
                percentage.round().to_string().truecolor(
                    r as u8,
                    g as u8,
                    0
                )
            );
        }

        if line.as_ref().unwrap().contains(HASH_6MAXENCE) {
            let line = line.unwrap();
            let values = line.split(",").collect::<Vec<&str>>();
            let x = values[3].to_string().replace("\"", "").parse().unwrap();
            let y = values[4].to_string().replace("\"", "").parse().unwrap();
            let color = values[2].to_string();
            placed_pixels.push(Pixel::new(x, y, color));
        }

        line_number += 1;
    }
    io::stdout().flush()?;
    print!("\r{}% processed.", "100".green());

    println!("\nHere are the placed pixels:");
    for pixel in &placed_pixels {
        println!("  x: {}, y: {}", pixel.x, pixel.y);
        let (r, g, b) = hex_to_rgb(&pixel.color);
        println!("  color: {}", pixel.color.truecolor(r, g, b));
        for i in pixel.x-3..=pixel.x+3 {
            for j in pixel.y-3..=pixel.y+3 {
                imgbuf.put_pixel(i, j, Rgba([r, g, b, 255]));
            }
        }
        // imgbuf.put_pixel(pixel.x, pixel.y, Rgba([r, g, b, 255]));
    }
    println!("Placed {} pixels.", &placed_pixels.len());
    println!("There are {} pixels in this file.", line_number - 1);

    imgbuf.save(IMAGE_PATH).unwrap();
    println!("Saved image to {}.", IMAGE_PATH);

    let end = Instant::now();
    println!("Process took {:?} to execute.", end.duration_since(start));

    Ok(())
}

// fn place_background(imgbuf: &mut RgbaImage) {
//
// }

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}
