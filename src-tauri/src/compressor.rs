use bitvec::prelude::BitVec;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

pub trait Compressor {
    fn compress(&self, filepath: PathBuf) -> PathBuf;
    fn compress_llm(&self, filepath: PathBuf) -> PathBuf;
}

pub struct VoidCompressor {}

impl Compressor for VoidCompressor {
    fn compress(&self, inpath: PathBuf) -> PathBuf {
        let filetype = inpath.extension().unwrap();
        match filetype {
            txt | org | md => self.compress_llm(&inpath),
        }
    }

    // Use ts_zip and pipe output to mut progress
    // Output code from https://stackoverflow.com/questions/31576555/unable-to-pipe-to-or-from-spawned-child-process-more-than-once/31577297#31577297
    fn compress_llm(&self, inpath: PathBuf) -> PathBuf {
        let filename = inpath.file_stem().unwrap();
        let outpath = Path::new("/path/to/data/dir").join(filename).join(".bin");
        let mut process = Command::new("/path/to/ts_zip")
            .arg("-m path/to/model")
            .arg(format!("c {} {}", inpath.display(), outpath.display()))
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
}
