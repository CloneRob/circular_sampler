use getopts::Options;
use std::path::{Path, PathBuf};
use std::{fs, io};


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

pub fn parse(args: Vec<String>) -> Option<Vec<PathBuf>> {
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
        return None;
    }

    
    let path_str = matches.opt_str("s");
    if let Some(p) = path_str {
        let path = Path::new(&p);
        if let Ok(files) = visit_dirs(&path) {
            return Some(files);
        } else {
            panic!("could not locate files")
        }
    } else {
        panic!("{:?} not a valid path", path_str)
    }
}
