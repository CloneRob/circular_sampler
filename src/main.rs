#[allow(unused_imports, dead_code)]
extern crate getopts;
extern crate rand;

mod argparse;
mod candidates;
mod crop;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let files = argparse::parse(args);
    if let Some(files) = files {
        println!("{}", files.len());
        println!("{:?}", files[1]);
    };
    let points = candidates::generate_candidates(6000, 550f64, 35f64);
    let transformer = candidates::Point::new(540f64, 670f64);
    let coords = candidates::transform_points(points, transformer);
    println!("{}", coords.len());
}
