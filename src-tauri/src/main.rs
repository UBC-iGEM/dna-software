// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(array_windows)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{fs, io};

use bitvec::{order::Msb0, prelude::BitVec};
use encoder::{Encoder, RotationEncoder};
use primer::{Base, MeltingTemperature, Primer};

mod chaosdna;
mod compressor;
mod decoder;
mod encoder;
mod primer;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_primers(
    len: usize,
    melting_temperature: MeltingTemperature,
    len_g: usize,
) -> Vec<Primer> {
    Primer::generate(len, melting_temperature, len_g)
}

#[tauri::command]
fn encode_sequence(file_path: &str) -> io::Result<Vec<Base>> {
    let encoder = RotationEncoder {};
    let bytes = fs::read(file_path)?;
    let bits = BitVec::<_, Msb0>::from_vec(bytes);
    Ok(encoder.encode(&bits))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_primers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
