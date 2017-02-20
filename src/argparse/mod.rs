use getopts::Options;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io};


pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} SOURCE_DIR DEST_DIR [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
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
