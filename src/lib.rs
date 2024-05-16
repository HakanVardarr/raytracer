use std::fs::OpenOptions;

mod error;
mod vec;

use error::Error;

pub fn run() -> Result<(), Error> {
    let _ = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("image.ppm")
        .map_err(|_| Error::CannotOpenFile(String::from("image.ppm")))?;

    Ok(())
}
