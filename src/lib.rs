mod image;
mod rgba;
mod rotate;
mod utils;

use image::Image;
use rotate::rotate_image;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;

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

    pub fn to_canvas(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        canvas.set_width(self.image.width);
        canvas.set_height(self.image.height);
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.image.to_u8_vec()),
            self.image.width,
            self.image.height,
        )
        .unwrap();
        context.put_image_data(&image_data, 0.0, 0.0)
    }
}
