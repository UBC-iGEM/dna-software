use std::{io::Cursor, path::Path, process::Command};

#[cfg(target_os = "linux")]
use pentacle;

#[cfg(target_os = "windows")]
use memexec;

pub trait Executor {
    fn create_exec<P: AsRef<Path>>(
        binary_path: P,
        args: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct BinaryExecutor {}

impl Executor for BinaryExecutor {
    fn create_exec<P: AsRef<Path>>(
        binary_path: P,
        args: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let binary = include_bytes!(binary_path);
        let mut program = Cursor::new(binary);

        if cfg!(target_os = "linux") {
            pentacle::ensure_sealed()?;
            let mut command = pentacle::SealedCommand::new(&mut program)?;
            command.args(args);
            command.spawn()?.wait()?;
        } else if cfg!(target_os = "windows") {
            let exit_code = unsafe { memexec::memexec_exe(binary) };
            if exit_code != 0 {
                return Err(format!("Binary execution failed with code: {}", exit_code).into());
            }
        } else {
            return Err("Unsupported operating system.".into());
        }

        Ok(())
    }
}
