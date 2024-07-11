use crate::exec::Executor;
use bitvec::prelude::BitVec;
use std::{path::PathBuf, process::Command};

pub trait Compressor {
    fn compress_llm(&self, filepath: PathBuf) -> PathBuf;
    fn compress(&self, filepath: PathBuf) -> PathBuf;
}

pub struct VoidCompressor {}

impl Compressor for VoidCompressor {
    fn compress_llm(&self, filepath: PathBuf) -> PathBuf {
        // Get the name of the input file
        let filename = filepath.file_stem().unwrap();

        // Root directory of project
        let executable_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // Get directory of executable
        let mut executable_dir = executable_root;
        executable_dir.push("ts_zip");
        if cfg!(target_os = "windows") {
            executable_dir.push("win64");
        } else if cfg!(target_os = "linux") && cfg!(target_env = "gnu") {
            executable_dir.push("linux_gnu");
        }

        // Get path to executable
        let mut executable_path = executable_dir;
        if cfg!(target_os = "windows") {
            executable_path.push("ts_zip.exe");
        } else if cfg!(target_os = "linux") && cfg!(target_env = "gnu") {
            executable_path.push("ts_zip");
        }

        // Get path to model
        let mut model_path = executable_dir;
        model_path.push("modelname");

        // Run the ts_zip executable
        let output_dir = PathBuf::from("/output/dir");
        let result = self.executor.create_exec(
            executable_path,
            &[
                "-m",
                model_path.to_str().unwrap(),
                "c",
                filepath.to_str().unwrap(),
                output_dir.to_str().unwrap(),
            ],
        );

        if let Err(e) = result {
            eprintln!("Error executing binary: {}", e);
        }
    }

    fn compress(&self, path: PathBuf) -> PathBuf {
        // write code to find filetype
        // let mut type = "add here!"
        //    match type {
        //    txt | rtf | org | md =>
        //	self.compress_llm(&path);
    }
}
