#[allow(unused_imports, dead_code)]
extern crate getopts;
extern crate rand;

mod argparse;
mod candidates;

use getopts::Options;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io};

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
        argparse::print_usage(&program, opts);
        return;
    }

    
    let path_str = matches.opt_str("s");
    if let Some(p) = path_str {
        let path = Path::new(&p);
        if let Ok(files) = argparse::visit_dirs(&path) {
            println!("{}", files.len());
            println!("{:?}", files[1]);
        };
    };

    let points = candidates::generate_candidates(6000, 550f64, 35f64);
    println!("{}", points.len());
}
