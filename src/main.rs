use std::collections::HashMap;

pub use crate::prelude::*;

mod ray;
mod utility;

mod prelude {
    pub use crate::ray::*;
    pub use crate::utility::*;
    pub use nalgebra::Vector3;
}

use image::{ImageBuffer, Rgb, RgbImage};
use rayon::{
    iter::Map,
    prelude::{
        IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator,
    },
};

fn main() {
    // Image data
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;

    // Camera data
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    // lower left corner in screen space
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);
    let mut pixels_on_screen = Vec::new();
    buffer.enumerate_pixels().into_iter().for_each(|(x, y, _)| {
        pixels_on_screen.push((x, y));
    });
    let pixels: HashMap<(u32, u32), Rgb<u8>> = pixels_on_screen
        .par_iter()
        .map(|a| {
            // Convert to uv coord's
            let u = a.0 as f64 / (image_width - 1) as f64;
            let v = a.1 as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                (lower_left_corner + u * horizontal + v * vertical) - origin,
            );
            let pixel_color = ray_color(&r);
            let png_color = to_png_color(&pixel_color);
            ((a.0, a.1), Rgb([png_color.0, png_color.1, png_color.2]))
        })
        .collect();

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let value = pixels.get(&(x, y));
        if let Some(value) = value {
            *pixel = value.clone();
        }
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done. image.png created"),
    }
}
