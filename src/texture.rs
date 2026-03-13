use image::{DynamicImage};
use std::path::Path;

#[derive(Debug)]
pub struct Texture {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0u8; (width * height * 4) as usize],
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| format!("Failed to load image: {}", e))?;
        Self::from_dynamic_image(img)
    }

    pub fn from_dynamic_image(img: DynamicImage) -> Result<Self, String> {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba.into_raw();

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = ((y * self.width + x) * 4) as usize;
        if index + 3 < self.pixels.len() {
            Some((
                self.pixels[index],
                self.pixels[index + 1],
                self.pixels[index + 2],
                self.pixels[index + 3],
            ))
        } else {
            None
        }
    }

    pub fn get_pixel_clamped(&self, x: i32, y: i32) -> (u8, u8, u8, u8) {
        let x = x.clamp(0, self.width as i32 - 1) as u32;
        let y = y.clamp(0, self.height as i32 - 1) as u32;
        self.get_pixel(x, y).unwrap_or((0, 0, 0, 255))
    }

    pub fn get_pixel_wrapped(&self, x: i32, y: i32) -> (u8, u8, u8, u8) {
        let x = ((x % self.width as i32) + self.width as i32) % self.width as i32;
        let y = ((y % self.height as i32) + self.height as i32) % self.height as i32;
        self.get_pixel(x as u32, y as u32).unwrap_or((0, 0, 0, 255))
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = ((y * self.width + x) * 4) as usize;
        if index + 3 < self.pixels.len() {
            self.pixels[index] = r;
            self.pixels[index + 1] = g;
            self.pixels[index + 2] = b;
            self.pixels[index + 3] = a;
        }
    }

    pub fn get_raw_pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn get_raw_pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let texture = Texture::new(100, 100);
        assert_eq!(texture.width(), 100);
        assert_eq!(texture.height(), 100);
    }

    #[test]
    fn test_set_get_pixel() {
        let mut texture = Texture::new(10, 10);
        texture.set_pixel(5, 5, 255, 128, 64, 255);
        let pixel = texture.get_pixel(5, 5).unwrap();
        assert_eq!(pixel, (255, 128, 64, 255));
    }

    #[test]
    fn test_get_pixel_out_of_bounds() {
        let texture = Texture::new(10, 10);
        assert!(texture.get_pixel(10, 10).is_none());
        assert!(texture.get_pixel(100, 100).is_none());
    }

    #[test]
    fn test_get_pixel_clamped() {
        let mut texture = Texture::new(10, 10);
        texture.set_pixel(9, 9, 255, 0, 0, 255);
        let pixel = texture.get_pixel_clamped(15, 15);
        assert_eq!(pixel, (255, 0, 0, 255));
    }

    #[test]
    fn test_get_pixel_wrapped() {
        let mut texture = Texture::new(10, 10);
        texture.set_pixel(0, 0, 255, 0, 0, 255);
        let pixel = texture.get_pixel_wrapped(10, 10);
        assert_eq!(pixel, (255, 0, 0, 255));
    }
}
