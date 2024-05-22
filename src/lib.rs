use std::{
    fs::{File, OpenOptions},
    io::Write,
};

mod error;
#[cfg(test)]
mod test;
mod vec;

const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGTH: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

use error::Error;
use vec::Vec3;

type Color = Vec3<f64>;

pub fn run() -> Result<(), Error> {
    let mut file = open_file("image.ppm")?;
    let mut contents = String::new();
    contents.push_str(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGTH}\n255\n").as_str());

    for j in 0..IMAGE_HEIGTH {
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGTH - 1) as f64,
                0.0,
            ) * 255.999;

            contents.push_str(format!("{} {} {}\n", color.x, color.y, color.z).as_str());
        }
    }

    file.write(contents.as_bytes())
        .map_err(|_| Error::CannotWriteToTheFile)?;
    Ok(())
}

fn open_file(path: &str) -> Result<File, Error> {
    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_| Error::CannotOpenFile(String::from(path)))?)
}
