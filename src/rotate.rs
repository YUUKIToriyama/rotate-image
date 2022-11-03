use crate::{image::Image, rgba::Rgba};

pub fn rotate_image(src: &Image, angle: f32) -> Image {
    let mut dst = Image::new(
        (src.width as f32 * angle.cos() + src.height as f32 * angle.sin()) as u32,
        (src.width as f32 * angle.sin() + src.height as f32 * angle.cos()) as u32,
    );

    for v in 0..dst.height - 1 {
        for u in 0..dst.width - 1 {
            let x = (u as i64 - (dst.width / 2) as i64) as f32 * angle.cos()
                - (v as i64 - (dst.height / 2) as i64) as f32 * angle.sin()
                + (src.width / 2) as f32;
            let y = (u as i64 - (dst.width / 2) as i64) as f32 * angle.sin()
                - (v as i64 - (dst.height / 2) as i64) as f32 * angle.cos()
                + (src.height / 2) as f32;

            if 0.0 < x && x < (src.width - 1) as f32 && 0.0 < y && y < (src.height - 1) as f32 {
                let nw = src.get_pixel(x.floor() as u32, y.floor() as u32);
                let ne = src.get_pixel(x.ceil() as u32, y.floor() as u32);
                let sw = src.get_pixel(x.ceil() as u32, y.ceil() as u32);
                let se = src.get_pixel(x.ceil() as u32, y.ceil() as u32);

                dst.put_pixel(u, v, get_average_color(nw, ne, sw, se));
            } else {
                dst.put_pixel(u, v, Rgba::new(0, 0, 0, 0));
            }
        }
    }
    dst
}

fn get_average_color(a: &Rgba, b: &Rgba, c: &Rgba, d: &Rgba) -> Rgba {
    let ab = a.blend(b);
    let cd = c.blend(d);
    ab.blend(&cd)
}
