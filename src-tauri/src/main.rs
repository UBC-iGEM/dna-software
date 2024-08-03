// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(array_windows)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]

use fasta::{FastaBase, FastaParser, Parser};
use primer::{Base, MeltingTemperature, Primer, PrimerInfo};
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{fs, path::PathBuf};

use bitvec::{order::Msb0, prelude::BitVec};
use compressor::{Compressor, VoidCompressor};
use decoder::{Decoder, QuaternaryDecoder, RotationDecoder};
use encoder::{Encoder, QuaternaryEncoder, RotationEncoder};
mod chaosdna;
mod compressor;
mod decoder;
mod encoder;
mod fasta;
mod primer;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_primers(
    len: usize,
    melting_temperature: MeltingTemperature,
    len_g: usize,
) -> Vec<PrimerInfo> {
    Primer::generate(len, melting_temperature, len_g)
}

#[tauri::command]
fn encode_sequence(encoder_type: &str, file_path: &str) -> Result<Vec<Base>, String> {
    let path = PathBuf::from(file_path);
    let compressor = VoidCompressor {};
    let compressed = compressor
        .compress(path.clone())
        .map_err(|err| err.to_string())?;
    let bytes = fs::read(compressed).map_err(|err| err.to_string())?;
    let bits = BitVec::<_, Msb0>::from_slice(&bytes);
    let encoder: Box<dyn Encoder> = match encoder_type {
        "quaternary" => Box::new(QuaternaryEncoder {}),
        "rotation" => Box::new(RotationEncoder {}),
        _ => return Err("Selected encoder does not exist.".to_string()),
    };

    Ok(encoder.encode(&bits).into())
}

#[tauri::command]
fn decode_sequence(file_path: &str) -> Result<String, String> {
    let fasta_file_content = fs::read_to_string(PathBuf::from(file_path)).unwrap();
    let fasta_bases = FastaParser::parse_into(&fasta_file_content);
    let decoder = QuaternaryDecoder {};
    let bases = fasta_bases
        .into_iter()
        .flatten()
        .map(|base| match base {
            FastaBase::Base(b) => Base::try_from(b).unwrap(),
            FastaBase::NotBase(b) => Base::A, // TODO: this is just for now since the error correction will deal with the NotBase
        })
        .collect::<Vec<Base>>();
    Ok(decoder.decode(&bases).to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_primers,
            encode_sequence,
            decode_sequence
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
