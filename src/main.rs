use colored::Colorize;
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::time::Instant;

const NUMBER_OF_LINES: u32 = 160_353_105;

type Coordinates = Vec<Vec<(u32, u32, u32)>>;
type RgbColor = (u8, u8, u8);

struct Pixel {
  x: u32,
  y: u32,
  color: String,
}

struct Dimensions {
  width: u32,
  height: u32,
}

impl Pixel {
  fn new(x: u32, y: u32, color: String) -> Pixel {
    let reghex = Regex::new(r"^#?[0-9a-fA-F]{6}$").unwrap();
    if !reghex.is_match(&color) {
      panic!("Invalid color: {}", color);
    }
    Pixel { x, y, color }
  }
}

fn main() -> io::Result<()> {
  let now = Instant::now();

  #[allow(unused_variables)]
    let my_hash = "bCrZRP7T31V14qwiWNzeBDKckEr+7q5aWwtYi/xnGSI57DwO4pWc5Ce1axjS3yNhF9wvmA2THtL/lwbIIeF69A==";
  let test_hash = "6wNblautrN+HX3ugQDgQboUMH4fYimqSezTsymY1w+vQl/uFDiHNqc0pA2BjBHAT6zww8iRX2ntGdqiZijBCYw==";

  let pixel_size = 16;

  let original_image_path = Path::new("img/original.png");
  let result_image_path = Path::new("img/rplace.png");
  let dataset_path = Path::new("data/rplace.csv");
  let heatmap_path = Path::new("img/heatmap.png");
  let img_dim = Dimensions {
    width: 2000,
    height: 2000,
  };

  let dataset = File::open(dataset_path).unwrap();
  let bufreader = BufReader::new(dataset);
  let mut img_buf: RgbaImage = ImageBuffer::new(img_dim.width, img_dim.height);
  let mut coords: Coordinates = coords_vec(img_dim.width, img_dim.height);
  let mut placed_pixels: Vec<Pixel> = vec![];

  set_background(&mut img_buf, original_image_path);

  for (counter, line) in bufreader.lines().skip(1).into_iter().enumerate() {
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
    let x = values[3].to_string().replace('"', "").parse().unwrap_or(0);
    let y = values[4].to_string().replace('"', "").parse().unwrap_or(0);
    coords[x as usize][y as usize].2 += 1;

    if line.contains(test_hash) {
      let color = values[2].to_string();
      placed_pixels.push(Pixel::new(x, y, color));
    }
  }
  io::stdout().flush()?;
  println!("\r{}% processed.", "100".truecolor(0, 255, 0));
  println!("Here are the placed pixels:");
  place_pixels(&placed_pixels, pixel_size, &mut img_buf);
  println!("Placed {} pixels.", &placed_pixels.len());
  img_buf.save(result_image_path).unwrap();
  create_heatmap(&mut coords, heatmap_path, img_dim);
  println!(
    "Result Image at : {} | Heatmap at : {}",
    result_image_path.display(),
    heatmap_path.display()
  );
  println!("Process took {:?} to execute.", now.elapsed());

  Ok(())
}

fn set_background(img_buf: &mut RgbaImage, source_img: &Path) {
  let background_img = image::open(source_img).unwrap();
  for i in 0..img_buf.width() {
    for j in 0..img_buf.height() {
      let pixel = background_img.get_pixel(i, j);
      img_buf.put_pixel(i, j, Rgba([pixel.0[0], pixel.0[1], pixel.0[2], 20]));
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

fn place_pixel(img_buf: &mut RgbaImage, pixel: &Pixel, mut size: u32) {
  if size == 0 {
    size = 1;
  }
  let (r, g, b) = hex_to_rgb(&pixel.color);
  let paddingleft = ((size as f32 - 1.0) / 2.0).ceil() as u32;
  let paddingright = ((size as f32 - 1.0) / 2.0).floor() as u32;

  let startx = (pixel.x as i32 - paddingleft as i32).max(0) as u32;
  let endx = (pixel.x as i32 + paddingright as i32).min(img_buf.width()) as u32;
  let starty = (pixel.y as i32 - paddingleft as i32).max(0) as u32;
  let endy = (pixel.y as i32 + paddingright as i32).min(img_buf.height()) as u32;

  for i in startx..=endx {
    for j in starty..=endy {
      img_buf.put_pixel(i, j, Rgba([r, g, b, 255]));
    }
  }
}

fn create_heatmap(
  coords: &mut Coordinates,
  heatmap_img: &Path,
  img_dim: Dimensions,
) {
  let mut img_buf: RgbImage = ImageBuffer::new(img_dim.width, img_dim.height);
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
      img_buf.put_pixel(coord.0, coord.1, Rgb([r, g, b]));
    }
  }

  img_buf.save(heatmap_img).unwrap();
}

// Blue -> Green -> Red
// ? Use the piecewise-linear crate, or implement a same functionality?
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

// TODO: Refactor this function
fn place_pixels(pixels: &[Pixel], pixel_size: u32, img_buf: &mut RgbaImage) {
  for pixel in pixels.iter() {
    let (r, g, b) = hex_to_rgb(&pixel.color);
    println!("  x: {}, y: {}", pixel.x, pixel.y);
    println!(
      "  color: {} ({})",
      pixel.color.truecolor(r, g, b),
      "  ".on_truecolor(r, g, b)
    );
    place_pixel(img_buf, pixel, pixel_size);
  }
}

fn coords_vec(width: u32, height: u32) -> Coordinates {
  let mut coords: Coordinates = vec![];

  for i in 0..width {
    coords.push(vec![]);
    for j in 0..height {
      coords[i as usize].push((i, j, 0))
    }
  }

  coords
}
