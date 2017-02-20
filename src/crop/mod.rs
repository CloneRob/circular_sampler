extern crate image;

use self::image::DynamicImage;
use std::path::PathBuf;

fn open_image(path: PathBuf) -> Option<DynamicImage> {
    let img = image::open(&path);
    if let Ok(img) = img {
        Some(img)
    } else {
        None
    }
}

pub fn crop(path: PathBuf) {
    let image = open_image(path);
    unimplemented!();
}
