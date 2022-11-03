mod image;
mod rgba;
mod rotate;
mod utils;

use image::Image;
use rotate::rotate_image;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct ImageConverter {
    image: Image,
}

#[wasm_bindgen]
impl ImageConverter {
    pub fn from_canvas(canvas: &HtmlCanvasElement) -> ImageConverter {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        match context.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64) {
            Ok(image_data) => {
                let image = Image::from_vec(canvas.width(), canvas.height(), image_data.data().0);
                ImageConverter { image }
            }
            Err(_) => panic!("Unable to load image from canvas."),
        }
    }

    pub fn rotate(&mut self, radian: f32) {
        self.image = rotate_image(&self.image, radian);
    }
}
