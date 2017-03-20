use ::candidates;
use ::argparse;
use ::argparse::SplitType;
use image;
use image::{DynamicImage, GenericImage};
use rayon::prelude::*;
use std::collections::HashMap;

use std::path::{Path, PathBuf, Component};
use std::cmp::min;
use std::{thread, time};
use std::fs::{File, DirBuilder};

pub struct Saver {
    prefix: PathBuf,
    target_path: PathBuf,
    label_map: Option<HashMap<String, String>>,
}

impl Saver {
    fn write(&self, origin_path: &PathBuf, patches: Vec<((u32, u32), DynamicImage)>) {
        let mut target = PathBuf::from(&self.target_path);
        if let Ok(path_in_folder) = origin_path.strip_prefix(&self.prefix) {
            let stem = path_in_folder.file_stem().unwrap();
            let parent = path_in_folder.parent().unwrap();
            let comps: Vec<_> = parent.components().collect();

            let mut identifier = String::new();
            for c in &comps[1..] {
                match *c {
                    Component::Normal(ident) => {
                        identifier.push_str(ident.to_str().unwrap());
                        identifier.push_str("|");
                    }
                    _ => {}
                }
            }
            identifier.push_str(&stem.to_str().unwrap());
            if let Some(ref map) = self.label_map {
                if let Component::Normal(ident) = comps[1] {
                    if let Some(tar) = map.get(ident.to_str().unwrap()) {
                        target.push(tar);
                    }
                }
            }

            for (coord, patch) in patches {
                let fmt = format!("{}|{}_{}.jpg", identifier.clone(), coord.0, coord.1);
                let mut patch_target = target.clone();
                patch_target.push(fmt);
                let ref mut fout = File::create(patch_target).unwrap();
                let _ = image::ImageRgb8(patch.to_rgb()).save(fout, image::JPEG);
            }
        }
    }

    pub fn build(config: &argparse::ParamConfig,
                 label_map: Option<HashMap<String, String>>)
                 -> Saver {
        if let Some(ref map) = label_map {
            for label in map.values() {
                let _ = DirBuilder::new()
                    .recursive(true)
                    .create(Path::join(&config.target, label))
                    .unwrap();


            }
        }
        Saver {
            prefix: config.prefix.clone(),
            target_path: config.target.clone(),
            label_map: label_map,
        }
    }
}

pub fn parallel_split(config: &argparse::ParamConfig, saver: &Saver) {
    let _ = DirBuilder::new().recursive(true).create(config.target.clone()).unwrap();

    config.files.par_iter().for_each(|path| {
        if let Some(patches) = crop(path, config.split_size, &config.split_type) {
            saver.write(path, patches);
        }
    });
}

// pub fn parallel_splitv2(config: &argparse::ParamConfig, saver: &Saver) {
// let _ = DirBuilder::new().recursive(true).create(config.target.clone()).unwrap();
//
// config.files.par_iter().for_each(|path| {
// match config.split_type {
// SplitType::Circular { sample_size, threshold, scaling } => {
// if let Some(patches) = crop(path,
// config.split_size,
// sample_size,
// scaling,
// threshold) {
// saver.write(path, patches);
// }
// }
// SplitType::Center => {
// unimplemented!();
// }
// }
// });
// }
//

pub fn crop(path: &PathBuf,
            crop_resolution: (u32, u32),
            split_type: &SplitType)
            -> Option<Vec<((u32, u32), DynamicImage)>> {
    let image = image::open(&path);
    if let Ok(mut img) = image {
        let (xdim, ydim) = img.dimensions();
        let min_dim = min(xdim, ydim);
        if crop_resolution.0 > min_dim {
            None
        } else {
            let mut patches = Vec::new();
            match *split_type {
                SplitType::Center => {
                    let x_center = xdim / 2;
                    let y_center = ydim / 2;

                    let patch = img.crop(x_center - crop_resolution.0 / 2,
                                         y_center - crop_resolution.1 / 2,
                                         crop_resolution.0,
                                         crop_resolution.1);
                    patches.push(((x_center, y_center), patch));

                }
                SplitType::Circular { sample_size, threshold, mut scaling } => {
                    let min_dim = min(xdim, ydim) as f64;
                    if scaling > min_dim / 2.0 {
                        scaling = min_dim / 2.0
                    }

                    let half_crop = (crop_resolution.0 / 2) as f64;
                    scaling = scaling - half_crop;

                    let points = candidates::generate_candidates(sample_size, scaling, threshold);
                    let transformer = candidates::Point::new((xdim / 2) as f64 - half_crop,
                                                             (ydim / 2) as f64 - half_crop);
                    let coords = candidates::to_coords(points, Some(transformer));

                    let mut patches = Vec::with_capacity(coords.len());

                    for centroid in coords.iter() {
                        let patch = img.crop(centroid.0, centroid.1, crop_resolution.0, crop_resolution.1);
                        patches.push((*centroid, patch.clone()));
                    }
                }
            }
            return Some(patches);
        }
    } else {
        None
    }
}
