extern crate image;

use std::fs;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() {

    fs::create_dir_all("bin/").expect("Coudn't create directory");

    let mut imgbuf : image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> = image::ImageBuffer::new(200, 100);


    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (255.99 * (x as f32 / WIDTH as f32)) as u8;
        let g = (255.99 * ((HEIGHT - y) as f32 / HEIGHT as f32)) as u8;
        *pixel = image::Rgb([r, g, 0]);
    }

    imgbuf.save("bin/image.png").unwrap();
}
