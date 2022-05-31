use colored::Colorize;
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::Instant;

// const HASH_6MAXENCE: &str =
// "bCrZRP7T31V14qwiWNzeBDKckEr+7q5aWwtYi/xnGSI57DwO4pWc5Ce1axjS3yNhF9wvmA2THtL/lwbIIeF69A==";
const HASH_TEST_USER: &str =
    "6wNblautrN+HX3ugQDgQboUMH4fYimqSezTsymY1w+vQl/uFDiHNqc0pA2BjBHAT6zww8iRX2ntGdqiZijBCYw==";
const NUMBER_OF_LINES: u32 = 160_353_105;
const DATA_PATH: &str = "data/rplace.csv";
// const DATA_PATH: &str = "data/sampleplace.csv";
const IMAGE_PATH: &str = "img/rplace.png";
const HEATMAP_PATH: &str = "img/heatmap.png";
const BACKGROUND_PATH: &str = "img/original.png";
const IMAGE_DIM: (u32, u32) = (2000, 2000);
const PIXEL_SIZE: u32 = 32;

type Coordinates = Vec<Vec<(u32, u32, u32)>>;
type RgbColor = (u8, u8, u8);

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
    let dataset = File::open(DATA_PATH).unwrap();
    let reader = BufReader::new(dataset);
    let mut imgbuf: RgbaImage = ImageBuffer::new(IMAGE_DIM.0, IMAGE_DIM.1);
    let mut coords: Coordinates = gen_vec_coords(IMAGE_DIM.0, IMAGE_DIM.1);
    let mut placed_pixels: Vec<Pixel> = vec![];
    let start = Instant::now();

    set_background(&mut imgbuf);

    for (counter, line) in reader.lines().skip(1).into_iter().enumerate() {
        if counter % 500_000 == 0 {
            io::stdout().flush()?;
            let percentage = counter as f64 / NUMBER_OF_LINES as f64 * 100.0;
            let r = 255.0 * (1.0 - percentage / 100.0);
            let g = 255.0 * (percentage / 100.0);
            print!(
                "\r{}% processed.",
                percentage
                    .round()
                    .to_string()
                    .truecolor(r as u8, g as u8, 0)
            );
        }

        let line = line.unwrap();
        let values = line.split(',').collect::<Vec<&str>>();
        let x = values[3].to_string().replace('\"', "").parse().unwrap_or(0);
        let y = values[4].to_string().replace('\"', "").parse().unwrap_or(0);
        coords[x as usize][y as usize].2 += 1;

        if line.contains(HASH_TEST_USER) {
            let color = values[2].to_string();
            placed_pixels.push(Pixel::new(x, y, color));
        }
    }
    io::stdout().flush()?;
    println!("\r{}% processed.", "100".truecolor(0, 255, 0));
    println!("Here are the placed pixels:");
    place_pixels(&placed_pixels, &mut imgbuf);
    println!("Placed {} pixels.", &placed_pixels.len());
    imgbuf.save(IMAGE_PATH).unwrap();
    create_heatmap(&mut coords);
    println!("Result Image at : {IMAGE_PATH} | Heatmap at : {HEATMAP_PATH}");
    println!("Process took {:?} to execute.", start.elapsed());

    Ok(())
}

fn set_background(imgbuf: &mut RgbaImage) {
    let placebuf = image::open(BACKGROUND_PATH).unwrap();
    for i in 0..IMAGE_DIM.0 {
        for j in 0..IMAGE_DIM.1 {
            let pixel = placebuf.get_pixel(i, j);
            imgbuf.put_pixel(
                i,
                j,
                Rgba([pixel.0[0], pixel.0[1], pixel.0[2], 20]),
            );
        }
    }
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn place_pixel(imgbuf: &mut RgbaImage, pixel: &Pixel, mut size: u32) {
    if size == 0 {
        size = 1;
    }
    let (r, g, b) = hex_to_rgb(&pixel.color);
    let paddingleft = ((size as f32 - 1.0) / 2.0).ceil() as u32;
    let paddingright = ((size as f32 - 1.0) / 2.0).floor() as u32;

    let startx = pixel.x as i32 - paddingleft as i32;
    let endx = pixel.x as i32 + paddingright as i32;
    let starty = pixel.y as i32 - paddingleft as i32;
    let endy = pixel.y as i32 + paddingright as i32;

    for i in startx..=endx {
        for j in starty..=endy {
            if i >= IMAGE_DIM.0 as i32
                || j >= IMAGE_DIM.1 as i32
                || i < 0
                || j < 0
            {
                continue;
            }
            imgbuf.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]));
        }
    }
}

fn create_heatmap(coords: &mut Coordinates) {
    let mut imgbuf: RgbImage = ImageBuffer::new(IMAGE_DIM.0, IMAGE_DIM.1);
    let max = 1000;
    // let max = {
    //     let mut curr = 0;
    //     for i in coords.iter() {
    //         for coord in i {
    //             if coord.2 > curr {
    //                 curr = coord.2
    //             }
    //         }
    //     }
    //     curr
    // };

    for i in coords.iter() {
        for coord in i {
            let (r, g, b) = get_heatmap_color(coord.2, max);
            imgbuf.put_pixel(coord.0, coord.1, Rgb([r, g, b]));
        }
    }

    imgbuf.save(HEATMAP_PATH).unwrap();
}

// Blue -> Green -> Red
// MAYBE: Use the piecewise-linear crate, or implement a same functionality?
fn get_heatmap_color(value: u32, max: u32) -> RgbColor {
    fn round_to_u8(num: f32) -> u8 {
        num.round() as u8
    }

    let factor = value as f32 / max as f32;

    if value <= max / 2 {
        (
            0,
            round_to_u8((2.0 * factor) * 255.0),
            round_to_u8((-2.0 * factor + 1.0) * 255.0),
        )
    } else if value <= max {
        (
            round_to_u8((2.0 * factor - 1.0) * 255.0),
            round_to_u8((-2.0 * factor + 2.0) * 255.0),
            0,
        )
    } else {
        (255, 0, 0)
    }
}

// White -> Blue -> Orange -> Red
#[allow(dead_code)]
// TODO: Refactor this function
fn get_heatmap_color_white_to_red(n: u32, max: u32) -> RgbColor {
    let (r, g, b);
    let factor = n as f32 / max as f32;

    if n <= max / 3 {
        r = ((-3.0 * factor + 1.0) * 255.0).round() as u8;
        g = ((-factor + 1.0) * 255.0).round() as u8;
        b = 255
    } else if n <= 2 * (max / 3) {
        r = ((3.0 * factor - 1.0) * 255.0).round() as u8;
        g = ((-factor + 1.0) * 255.0).round() as u8;
        b = ((-3.0 * factor + 2.0) * 255.0).round() as u8;
    } else if n <= max {
        r = 255;
        g = ((-factor + 1.0) * 255.0).round() as u8;
        b = 0;
    } else {
        r = 255;
        g = 0;
        b = 0;
    }

    (r, g, b)
}

fn place_pixels(pixels: &[Pixel], imgbuf: &mut RgbaImage) {
    for pixel in pixels.iter() {
        let (r, g, b) = hex_to_rgb(&pixel.color);
        println!("  x: {}, y: {}", pixel.x, pixel.y);
        println!(
            "  color: {} ({})",
            pixel.color.truecolor(r, g, b),
            "  ".on_truecolor(r, g, b)
        );
        place_pixel(imgbuf, pixel, PIXEL_SIZE);
    }
}

fn gen_vec_coords(width: u32, height: u32) -> Coordinates {
    let mut coords: Coordinates = vec![];

    for i in 0..width {
        coords.push(vec![]);
        for j in 0..height {
            coords[i as usize].push((i, j, 0))
        }
    }

    coords
}
