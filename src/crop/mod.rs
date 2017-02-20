extern crate image;

use self::image::DynamicImage;
use std::path::PathBuf;

struct SplitConfig {
    path_list: Vec<PathBuf>,
    crop_resolution: (u32, u32),
}

impl SplitConfig {
    fn new(path_list: Vec<PathBuf>, crop_res: (u32,u32)) -> SplitConfig {
        SplitConfig {
            path_list: path_list,
            crop_resolution: crop_res,
        }
    }
}

pub fn crop(path: PathBuf) {
    let image = image::open(&path);
    unimplemented!();
}
