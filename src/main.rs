extern crate getopts;
extern crate rand;

use getopts::Options;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::f32;
use rand::distributions::{IndependentSample, Range};

struct Point<T> {
    x: T, 
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y,
        }
    }
}

fn gen_coords(n: usize, scale: f32) -> Vec<Point<f32>> {
    let mut nums = Vec::with_capacity(n);

    let theta_range = Range::new(0f32, f32::consts::PI * 2f32);
    let rho_range = Range::new(-1f32, 1f32);
    let mut rng = rand::thread_rng();

    for _ in 0..n {
       let t = theta_range.ind_sample(&mut rng);
       let r = f32::sqrt(rho_range.ind_sample(&mut rng));

       let x = f32::cos(t) * r * scale;
       let y = f32::sin(t) * r * scale;
       
       nums.push(Point::new(x, y));
    }
    nums
}

fn centroid_candidates(point_slice: &[Point<f32>]) -> Point<f32> {

    Point::new(0f32, 1f32)
}

fn remove_centroids(mut points: Vec<Point<f32>>) -> Vec<Point<f32>> {
    let mut centroids = Vec::new();
    let mut counter = points.len();

    while counter > 0 {

    }
    centroids
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} SOURCE_DIR DEST_DIR [options]", program);
    print!("{}", opts.usage(&brief));
}

fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();

            if path.is_dir() {
                if let Ok(fl) = visit_dirs(&path) {
                    files.extend_from_slice(&fl[..]);
                }
            } else {
                let ext = path.extension().unwrap().to_str();
                match ext {
                    Some("jpg") => { files.push(PathBuf::from(&path)); },
                    Some("png") => { files.push(PathBuf::from(&path)); },
                    _ => {}
                };
            }
        }
    }
    Ok(files)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("s", "source", "Source folder of the image files", "DIR");
    opts.optflag("h", "help", "Prints help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    
    let path_str = matches.opt_str("s");
    if let Some(p) = path_str {
        let path = Path::new(&p);
        if let Ok(files) = visit_dirs(&path) {
            println!("{}", files.len())
        };
    };
}
