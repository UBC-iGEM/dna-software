// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use primer::{MeltingTemperature, Primer};

mod compressor;
mod encoder;
mod primer;
mod sequence;
mod chaosdna;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_primers(
    len: usize,
    melting_temperature: MeltingTemperature,
    len_g: usize,
) -> Vec<Primer> {
    Primer::generate(len, melting_temperature, len_g)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_primers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
