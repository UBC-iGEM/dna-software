use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub trait Compressor {
    fn compress(&self, inpath: PathBuf) -> PathBuf;
    fn compress_llm(&self, inpath: PathBuf) -> PathBuf;
    fn compress_lz4(&self, inpath: PathBuf) -> PathBuf;
    fn decompress(&self, inpath: PathBuf) -> PathBuf;
    fn decompress_llm(&self, inpath: PathBuf) -> PathBuf;
    fn decompress_lz4(&self, inpath: PathBuf) -> PathBuf;
}

pub struct VoidCompressor {}

impl Compressor for VoidCompressor {
    fn compress(&self, inpath: PathBuf) -> PathBuf {
        // Access the extension and compare it to string literals
        match inpath.extension().and_then(|os_str| os_str.to_str()) {
            Some("txt") | Some("org") | Some("md") => self.compress_llm(inpath),
            _ => self.compress_lz4(inpath),
        }
    }

    // Use ts_zip and pipe output to mut progress
    // Output code from https://stackoverflow.com/questions/31576555/unable-to-pipe-to-or-from-spawned-child-process-more-than-once/31577297#31577297
    fn compress_llm(&self, inpath: PathBuf) -> PathBuf {
        let filename = inpath.file_stem().unwrap();
        let outpath = Path::new("/path/to/data/dir")
            .join(filename)
            .with_extension("bin");
        let mut process = Command::new("/path/to/ts_zip")
            .arg("-m")
            .arg("path/to/model")
            .arg("c")
            .arg(inpath.as_path().to_str().unwrap()) 
            .arg(outpath.as_path().to_str().unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("compression process");
        let stdout = process.stdout.take().unwrap();
        let mut reader = BufReader::new(stdout);
        let mut progress_display = String::new();
        let mut buffer = String::new();

        loop {
            match reader.read_line(&mut buffer) {
                Ok(0) => break, // End of stream
                Ok(_) => {
                    // Check if the line contains "ratio" before updating progress
                    if buffer.contains("ratio") {
                        break;
                    }
                    progress_display = buffer.clone();
                    println!("{}", progress_display);
                    buffer.clear(); // Clear the buffer for the next line
                }
                Err(e) => {
                    eprintln!("Error reading from ts_zip: {}", e);
                    break;
                }
            }
        }

        // Wait for the process to finish and handle potential errors
        if let Err(e) = process.wait() {
            eprintln!("Error during compression: {}", e);
        }

        outpath
    }

    fn compress_lz4(&self, inpath: PathBuf) -> PathBuf {
        compress(&inpath, &inpath.with_extension("lz4")).unwrap();
        inpath.with_extension("lz4")
    }

    fn decompress(&self, inpath: PathBuf) -> PathBuf {
        // Access the extension and compare it to string literals
        match inpath.extension().and_then(|os_str| os_str.to_str()) {
            Some("bin") => self.decompress_llm(inpath),
            _ => self.decompress_lz4(inpath),
        }
    }

    // Use ts_zip and pipe output to mut progress
    // Output code from https://stackoverflow.com/questions/31576555/unable-to-pipe-to-or-from-spawned-child-process-more-than-once/31577297#31577297
    fn decompress_llm(&self, inpath: PathBuf) -> PathBuf {
        let filename = inpath.file_stem().unwrap();
        let outpath = Path::new("/path/to/data/dir")
            .join(filename)
            .with_extension("txt");
        let mut process = Command::new("/path/to/ts_zip")
            // Assuming ts_zip takes arguments this way, otherwise adjust accordingly
            .arg("-m")
            .arg("path/to/model")
            .arg("d")
            .arg(inpath.as_path().to_str().unwrap()) // Convert PathBuf to &str for display
            .arg(outpath.as_path().to_str().unwrap())
            .output()
            .expect("decompression process");
        outpath
    }

    fn decompress_lz4(&self, inpath: PathBuf) -> PathBuf {
        decompress(&inpath, &inpath.with_extension("")).unwrap();
    }
}
