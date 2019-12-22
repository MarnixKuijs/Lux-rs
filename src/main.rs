use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use cgmath::Vector3;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() {

    fs::create_dir_all("bin/").expect("Coudn't create directory");

    let path = Path::new("bin/image.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    file.write_all((format!("P3\n{} {}\n255\n", WIDTH, HEIGHT)).as_bytes())
        .expect(&format!("couldn't write to {}", display));

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut color = Vector3 {
                x: (i as f32 / WIDTH as f32),
                y: (j as f32 / HEIGHT as f32),
                z: 0.0,
            };

            color *= 255.99;

            let color = format!("{} {} {}\n", color.x as u32, color.y as u32, color.z as u32);
            file.write_all(color.as_bytes())
                .expect(&format!("couldn't write to {}", display));
        }
    }
}
