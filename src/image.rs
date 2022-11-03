use crate::rgba::Rgba;

pub struct Image {
    pub pixels: Vec<Rgba>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            pixels: vec![Rgba::new(0, 0, 0, 0); (width * height) as usize],
            width,
            height,
        }
    }
    pub fn from_vec(width: u32, height: u32, vector: Vec<u8>) -> Image {
        let mut pixels: Vec<Rgba> = vec![];
        for (index, _) in vector.iter().enumerate() {
            match index % 4 {
                3 => pixels.push(Rgba::new(
                    vector[index - 3],
                    vector[index - 2],
                    vector[index - 1],
                    vector[index],
                )),
                _ => continue,
            }
        }
        Image {
            pixels,
            width,
            height,
        }
    }
    pub fn to_u8_vec(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = vec![];
        for pixel in &self.pixels {
            vector.push(pixel.r);
            vector.push(pixel.g);
            vector.push(pixel.b);
            vector.push(pixel.a);
        }
        vector
    }
    pub fn get_pixel(&self, w: u32, h: u32) -> &Rgba {
        &self.pixels[(self.width * h + w) as usize]
    }
    pub fn put_pixel(&mut self, w: u32, h: u32, pixel: Rgba) {
        self.pixels[(self.width * h + w) as usize] = pixel;
    }
}

#[cfg(test)]
mod test_image {
    use std::vec;

    use super::*;

    #[test]
    fn get_pixel() {
        let width: u32 = 4;
        let height: u32 = 3;
        let pixels: Vec<Rgba> = vec![
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(0, 1, 2, 3),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
        ];
        let image = Image {
            pixels,
            width,
            height,
        };
        let pixel = image.get_pixel(2, 1);
        assert_eq!(pixel.r, 0);
        assert_eq!(pixel.g, 1);
        assert_eq!(pixel.b, 2);
        assert_eq!(pixel.a, 3);
    }

    #[test]
    fn from_vec() {
        let width = 3;
        let height = 1;
        let vector: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 255, 255, 156, 0, 255, 64];
        let image = Image::from_vec(width, height, vector);
        let pixel = image.get_pixel(2, 0);
        assert_eq!(pixel.r, 156);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 255);
        assert_eq!(pixel.a, 64);
    }

    #[test]
    fn to_u8_vec() {
        let width: u32 = 4;
        let height: u32 = 3;
        let pixels: Vec<Rgba> = vec![
            Rgba::new(255, 255, 255, 255),
            Rgba::new(0, 0, 0, 156),
            Rgba::new(156, 255, 0, 0),
        ];
        let image = Image {
            pixels,
            width,
            height,
        };
        assert_eq!(
            image.to_u8_vec(),
            vec![255, 255, 255, 255, 0, 0, 0, 156, 156, 255, 0, 0]
        );
    }

    #[test]
    fn put_pixel() {
        let mut image = Image::new(5, 5);
        image.put_pixel(3, 2, Rgba::new(8, 100, 99, 34));
        let pixel = image.get_pixel(3, 2);
        assert_eq!(pixel.r, 8);
        assert_eq!(pixel.g, 100);
        assert_eq!(pixel.b, 99);
        assert_eq!(pixel.a, 34);
    }
}
