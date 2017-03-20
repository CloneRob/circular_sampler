#[allow(unused_imports, dead_code)]
extern crate getopts;
extern crate rand;
extern crate image;
extern crate rayon;

mod argparse;
mod candidates;
mod crop;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let (Some(param_config), Some(label)) = argparse::parse(args) {
        if let Ok(map) = param_config.label_map(&label) {
            let saver = crop::Saver::build(&param_config, Some(map));
            crop::parallel_split(&param_config, &saver);
        }
    }
}
