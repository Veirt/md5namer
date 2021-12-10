use md5::{Digest, Md5};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, process};

struct FileInfo {
    input_file: File,
    input_path: PathBuf,
}

impl FileInfo {
    fn new(mut args: env::Args) -> Self {
        args.next();

        if let Some(file) = args.next() {
            let path = Path::new(&file);

            Self {
                input_file: File::open(path).expect("Cannot open the file."),
                input_path: path.to_path_buf(),
            }
        } else {
            eprintln!("Please input a file name.");
            process::exit(1)
        }
    }

    fn name_to_hash(&mut self) {
        let mut buffer = Vec::new();
        self.input_file
            .read_to_end(&mut buffer)
            .expect("Cannot read the file.");

        let digest = Md5::digest(&buffer);
        let output_file = match self.input_path.extension() {
            Some(extension) => {
                format!("{:x}.{}", digest, extension.to_str().unwrap())
            }
            None => {
                format!("{:x}", digest)
            }
        };

        fs::copy(&self.input_path, output_file).expect("Cannot copy and rename the file.");
    }
}

fn main() {
    let mut file = FileInfo::new(env::args());
    file.name_to_hash();
}
