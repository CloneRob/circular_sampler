use ::candidates;
use ::argparse;
use ::argparse::SplitType;
use image;
use image::{DynamicImage, GenericImage};
use rayon::prelude::*;

use std::path::{Path, PathBuf, Component};
use std::cmp::min;
use std::{thread, time};
use std::fs::{File, DirBuilder};


pub fn parallel_split(config: &argparse::ParamConfig) {
    let _ = DirBuilder::new().recursive(true).create(config.target.clone()).unwrap();

    config.files.par_iter().for_each(|path| {
        match config.split_type {
            SplitType::Circular { sample_size, threshold, scaling } => {
                if let Some(patches) = crop(path,
                                            config.split_size,
                                            sample_size,
                                            scaling,
                                            threshold) {
                    save_patches(patches, &config.prefix, path, &config.target);
                }
            }
            SplitType::Center => {
                unimplemented!();
            }
        }
    });
}

fn save_patches(patches: Vec<((u32, u32), DynamicImage)>,
                prefix: &PathBuf,
                origin_path: &PathBuf,
                target_path: &PathBuf) {
    let target = PathBuf::from(target_path);
    if let Ok(path_in_folder) = origin_path.strip_prefix(prefix) {
        let stem = path_in_folder.file_stem().unwrap();
        let parent = path_in_folder.parent().unwrap();
        let comps: Vec<_> = parent.components().collect();

        let mut identifier = String::new();
        for c in comps {
            match c {
                Component::Normal(ident) => {
                    identifier.push_str(ident.to_str().unwrap());
                    identifier.push_str("|");
                }
                _ => {}
            }
        }
        identifier.push_str(&stem.to_str().unwrap());


        for (coord, patch) in patches {
            let fmt = format!("{}|{}_{}.jpg", identifier.clone(), coord.0, coord.1);
            let mut patch_target = target.clone();
            patch_target.push(fmt);
            let ref mut fout = File::create(patch_target).unwrap();
            let _ = image::ImageRgb8(patch.to_rgb()).save(fout, image::JPEG);
        }
    }
}
pub fn crop(path: &PathBuf,
            crop_resolution: (u32, u32),
            samplesize: usize,
            mut scaling: f64,
            threshold: Option<f64>)
            -> Option<Vec<((u32, u32), DynamicImage)>> {
    let image = image::open(&path);
    if let Ok(mut img) = image {
        let (xdim, ydim) = img.dimensions();

        let min_dim = min(xdim, ydim) as f64;
        if scaling > min_dim / 2.0 {
            scaling = min_dim / 2.0
        }

        let half_crop = (crop_resolution.0 / 2) as f64;
        scaling = scaling - half_crop;

        let points = candidates::generate_candidates(samplesize, scaling, threshold);
        let transformer = candidates::Point::new((xdim / 2) as f64 - half_crop,
                                                 (ydim / 2) as f64 - half_crop);
        let coords = candidates::to_coords(points, Some(transformer));

        let mut patches = Vec::with_capacity(coords.len());

        for centroid in coords.iter() {
            let patch = img.crop(centroid.0, centroid.1, crop_resolution.0, crop_resolution.1);
            patches.push((*centroid, patch.clone()));
        }
        Some(patches)
    } else {
        None
    }
}

pub fn cropv2(path: &PathBuf,
             crop_resolution: (u32, u32),
             split_type: SplitType)
             -> Option<Vec<((u32, u32), DynamicImage)>> {
    let image = image::open(&path);
    if let Ok(mut img) = image {
        let (xdim, ydim) = img.dimensions();
        let min_dim = min(xdim, ydim);
        if crop_resolution.0 > min_dim {
            None
        } else {
            match split_type {
                SplitType::Center => {

                },
                SplitType::Circular { sample_size, threshold, scaling } => {

                }
            }
            None
        }
    } else {
        None
    }
}
