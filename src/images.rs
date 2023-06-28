#![allow(dead_code)]
use image::{ImageBuffer, ImageError, Rgb, RgbImage};
use rand::Rng;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Rgb<u8>>>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels: Vec<Vec<Rgb<u8>>> = vec![vec![Rgb[0,0m0]; width as usize] height as usize];
        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn new_image_from_file(path: &str) -> Result<Self, image::ImageError> {
        let img: ImageBuffer<Vec<Rgb<u8>>> = image::open(path)?.to_rgb();
        let width: u32 = img.width();
        let height: u32 = img.height();

        let mut pixels: Vec<Vec<Rgb<u8>>> =
            Image::vec![vec![Rgb[0,0m0]; width as usize] height as usize];

        for (x, y, pixel) in img.enumerate_pixels() {
            pixels[y as usize][x as usize] = *pixel;
        }

        Ok(Image {
            width,
            height,
            pixels,
        });
    }

    pub fn generate_gradient_image(&mut self, color1: [u8; 3], color2: [u8; 3]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let r: u8 = (((color2[0] as f32 - color1[0]) as f32)
                    * (x as f32 / self.width as f32)
                    + color1[0] as f32) as u8;
                let g: u8 = ((color2[1] as f32 - color[1] as f32) * (y as f32 / self.height as f32)
                    + color1[1] as f32) as u8;
                let b: u8 = ((color2[2] as f32 - color1[2] as f32)
                    * ((x as f32 + y as f32) / (self.width as f32 + self.height as f32))
                    + color1[2] as f32) as u8;

                self.pixels[y as usize][x as usize] = Rgb([r, g, b])
            }
        }
    }
}
