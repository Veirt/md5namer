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
    fn new(file_path: Option<String>) -> Self {
        if let Some(file) = file_path {
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

    fn get_output_file_name(&mut self) -> String {
        let mut buffer = Vec::new();
        self.input_file
            .read_to_end(&mut buffer)
            .expect("Cannot read the file.");

        let digest = Md5::digest(&buffer);

        match self.input_path.extension() {
            Some(extension) => {
                format!("{:x}.{}", digest, extension.to_str().unwrap())
            }
            None => {
                format!("{:x}", digest)
            }
        }
    }

    fn rename_to_hash(&mut self) {
        let output_file = self.get_output_file_name();

        fs::copy(&self.input_path, &output_file).expect("Cannot copy and rename the file.");
        println!("{}", output_file);
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let file_path = args.next();

    let mut file = FileInfo::new(file_path);
    file.rename_to_hash();
}
