use getopts::Options;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct ParamConfig {
    pub files: Vec<PathBuf>,
    pub target: PathBuf,
    pub prefix: PathBuf,
    pub split_size: (u32, u32),
    pub sample_size: usize,
    pub threshold: Option<f64>,
    pub scaling: f64,
}

struct ParamBuilder {
    files: Option<Vec<PathBuf>>,
    target: Option<PathBuf>,
    prefix: Option<PathBuf>,
    split_size: Option<(u32, u32)>,
    sample_size: Option<usize>,
    threshold: Option<f64>,
    scaling: Option<f64>,
}

impl ParamBuilder {
    fn new() -> ParamBuilder {
        ParamBuilder {
            files: None,
            target: None,
            prefix: None,
            split_size: None,
            sample_size: None,
            threshold: None,
            scaling: None,
        }
    }
    fn files(mut self, files: Vec<PathBuf>) -> ParamBuilder {
        self.files = Some(files);
        self
    }
    fn target(mut self, target: PathBuf) -> ParamBuilder {
        self.target = Some(target);
        self
    }
    fn prefix(mut self, prefix: PathBuf) -> ParamBuilder {
        self.prefix = Some(prefix);
        self
    }
    fn split_size(mut self, split_size: (u32, u32)) -> ParamBuilder {
        self.split_size = Some(split_size);
        self
    }
    fn sample_size(mut self, sample_size: usize) -> ParamBuilder {
        self.sample_size = Some(sample_size);
        self
    }
    fn threshold(mut self, threshold: f64) -> ParamBuilder {
        self.threshold = Some(threshold);
        self
    }
    fn scaling(mut self, scaling: f64) -> ParamBuilder {
        self.scaling = Some(scaling);
        self
    }

    fn build(self) -> ParamConfig {
        ParamConfig::new(self.files.expect("How did this go through"),
                         self.prefix.expect(""),
                         self.target.expect(""),
                         self.split_size.unwrap_or((96, 96)),
                         self.sample_size.unwrap_or(6000),
                         self.threshold,
                         self.scaling.unwrap_or(520f64))
    }
}

impl ParamConfig {
    fn new(files: Vec<PathBuf>,
           prefix: PathBuf,
           target: PathBuf,
           split_size: (u32, u32),
           sample_size: usize,
           threshold: Option<f64>,
           scaling: f64)
           -> ParamConfig {
        ParamConfig {
            files: files,
            target: target,
            prefix: prefix,
            split_size: split_size,
            sample_size: sample_size,
            threshold: threshold,
            scaling: scaling,
        }
    }
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
                    Some("jpg") => {
                        files.push(PathBuf::from(&path));
                    }
                    Some("png") => {
                        files.push(PathBuf::from(&path));
                    }
                    _ => {}
                };
            }
        }
    }
    Ok(files)
}

pub fn parse(args: Vec<String>) -> Option<ParamConfig> {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("s", "source", "Source folder of the image files", "DIR");
    opts.reqopt("t",
                "target",
                "Target path were patches will be stored",
                "DIR");
    opts.reqopt("r", "", "split resolution", "Int");
    opts.reqopt("b", "threshold", "Upper bound for centroid merging", "Float");
    opts.optflag("h", "help", "Prints help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }

    let mut param_builder = ParamBuilder::new();

    let source_str = matches.opt_str("s");
    if let Some(p) = source_str {
        let path = PathBuf::from(&p);
        if let Ok(files) = visit_dirs(&path) {
            param_builder = param_builder.prefix(path);
            param_builder = param_builder.files(files);
        } else {
            panic!("could not locate files")
        }
    } else {
        panic!("{:?} not a valid path", source_str)
    }

    let target_str = matches.opt_str("t");
    if let Some(t) = target_str {
        param_builder = param_builder.target(PathBuf::from(t));
    } else {
        panic!("{:?} not a valid path", target_str)
    }

    let resolution_str = matches.opt_str("r");
    if let Some(r) = resolution_str {
        if let Ok(res) = r.parse::<u32>() {
            param_builder = param_builder.split_size((res, res));
        }
    }

    let threshold_str = matches.opt_str("b");
    if let Some(b) = threshold_str {
        if let Ok(thres) = b.parse::<f64>() {
            param_builder = param_builder.threshold(thres);
        }
    }
    
    Some(param_builder.build())
}
