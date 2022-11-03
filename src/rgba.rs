use crate::utils::average;

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }
    pub fn blend(&self, other: &Rgba) -> Rgba {
        Rgba::new(
            average(self.r, other.r),
            average(self.g, other.g),
            average(self.b, other.b),
            average(self.a, other.a),
        )
    }
}

#[cfg(test)]
mod test_rgba {
    use super::*;

    #[test]
    fn new() {
        let pixel = Rgba::new(1, 2, 3, 4);
        assert_eq!(pixel.r, 1);
        assert_eq!(pixel.g, 2);
        assert_eq!(pixel.b, 3);
        assert_eq!(pixel.a, 4);
    }

    #[test]
    fn blend() {
        let pixel1 = Rgba::new(10, 5, 8, 9);
        let pixel2 = Rgba::new(2, 7, 100, 255);
        let blended = pixel1.blend(&pixel2);
        assert_eq!(blended.r, 6);
        assert_eq!(blended.g, 6);
        assert_eq!(blended.b, 54);
        assert_eq!(blended.a, 132);
    }
}
