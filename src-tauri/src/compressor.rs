use lz4::{Decoder, EncoderBuilder};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Result, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

// Trait for all compression and decompression functions
pub trait Compressor {
    fn compress(&self, inpath: PathBuf) -> Result<PathBuf>;
    fn compress_llm(&self, inpath: PathBuf) -> Result<PathBuf>;
    fn compress_lz4(&self, inpath: PathBuf) -> Result<PathBuf>;
    fn decompress(&self, inpath: PathBuf, outpath: PathBuf) -> Result<()>;
    fn decompress_llm(&self, inpath: PathBuf, outpath: PathBuf) -> Result<()>;
    fn decompress_lz4(&self, inpath: PathBuf, outpath: PathBuf) -> Result<()>;
}

pub struct VoidCompressor {}

impl Compressor for VoidCompressor {
    fn compress(&self, inpath: PathBuf) -> Result<PathBuf> {
        // Currently, this function just aliases to compress_lz4. May use patterm matching to select a function in future.
        //
        //  match inpath.extension().and_then(|os_str| os_str.to_str()) {
        //      Some("txt") | Some("org") | Some("md") => self.compress_llm(inpath),
        //      _ => self.compress_lz4(inpath),
        //  }
        self.compress_lz4(inpath)
    }

    // Use ts_zip and pipe output to mut progress
    // Progress display code from https://stackoverflow.com/questions/31576555/unable-to-pipe-to-or-from-spawned-child-process-more-than-once/31577297#31577297, untested
    fn compress_llm(&self, inpath: PathBuf) -> Result<PathBuf> {
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
            .spawn()?;

        let stdout = process.stdout.take().unwrap();
        let mut reader = BufReader::new(stdout);
        let mut progress_display = String::new();
        let mut buffer = String::new();

        loop {
            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    if buffer.contains("ratio") {
                        break;
                    }
                    progress_display = buffer.clone();
                    println!("{}", progress_display);
                    buffer.clear();
                }
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error reading from ts_zip: {}", e),
                    ));
                }
            }
        }

        let output = process.wait()?;
        if !output.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ts_zip compression failed",
            ));
        }

        Ok(outpath)
    }

    fn compress_lz4(&self, inpath: PathBuf) -> Result<PathBuf> {
        let mut input_file = File::open(inpath.as_path())?;
        let filename = inpath.file_stem().unwrap();
        let outpath = Path::new("/path/to/data/dir")
            .join(filename)
            .with_extension("lz4");
        let output_file = File::create(outpath.as_path())?;

        println!("Compressing: {} -> {}", inpath.display(), outpath.display());

        let mut encoder = EncoderBuilder::new().level(12).build(output_file)?;
        io::copy(&mut input_file, &mut encoder)?;
        let (mut writer, result) = encoder.finish();
        result?;
        writer.flush()?;

        Ok(outpath)
    }

    fn decompress(&self, inpath: PathBuf, outpath: PathBuf) -> Result<()> {
        //  Currently aliases to decompress_lz4. May use pattern matching in future
        //
        //  match inpath.extension().and_then(|os_str| os_str.to_str()) {
        //      Some("bin") => self.decompress_llm(inpath, outpath),
        //      _ => self.decompress_lz4(inpath, outpath),
        //  }
        self.decompress_lz4(inpath, outpath)
    }

    fn decompress_llm(&self, inpath: PathBuf, outpath: PathBuf) -> Result<()> {
        let mut process = Command::new("/path/to/ts_zip")
            .arg("-m")
            .arg("path/to/model")
            .arg("d")
            .arg(inpath.as_path().to_str().unwrap())
            .arg(outpath.as_path().to_str().unwrap())
            .output()
            .expect("decompression process");

        Ok(())
    }

    fn decompress_lz4(&self, inpath: PathBuf, outpath: PathBuf) -> Result<()> {
        println!(
            "Decompressing: {} -> {}",
            inpath.display(),
            outpath.display()
        );

        let input_file = File::open(inpath.as_path())?;
        let mut decoder = Decoder::new(input_file)?;
        let mut output_file = File::create(outpath.as_path())?;
        io::copy(&mut decoder, &mut output_file)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    use std::{
        fs::{self, File},
        io::Read,
    };
    use tempfile::tempdir;

    #[quickcheck]
    fn test_compress_decompress_lz4(data: Vec<u8>) -> TestResult {
        let compressor = VoidCompressor {};
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input.txt");
        let decompressed_path = temp_dir.path().join("output.txt");
        fs::write(&input_path, &data).unwrap();

        // Compress
        let compressed_path = compressor.compress_lz4(input_path.clone()).unwrap();

        // Decompress
        compressor
            .decompress_lz4(compressed_path, decompressed_path.clone())
            .unwrap();

        // Read decompressed data
        let mut decompressed_data = Vec::new();
        let mut file = File::open(decompressed_path).unwrap();
        file.read_to_end(&mut decompressed_data).unwrap();

        // Assert that the decompressed data matches the original data
        TestResult::from_bool(data == decompressed_data)
    }
}
