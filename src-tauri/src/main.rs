// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(array_windows)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]

use blocker::BitBlocker;
use fasta::{FastaBase, FastaParser, Parser};
use metadata::MetaData;
use primer::{Base, MeltingTemperature, Primer, PrimerInfo};
use scaffolder::Scaffolder;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{fs, path::PathBuf};

use bitvec::{order::Msb0, prelude::BitVec};
use compressor::{Compressor, VoidCompressor};
use decoder::{ChurchDecoder, Decoder, QuaternaryDecoder, RotationDecoder};
use encoder::{ChurchEncoder, Encoder, QuaternaryEncoder, RotationEncoder};

mod blocker;
mod chaosdna;
mod compressor;
mod decoder;
mod encoder;
mod fasta;
mod metadata;
mod primer;
mod scaffolder;

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
fn encode_sequence(encoder_type: &str, file_path: &str) -> Result<Vec<Vec<Base>>, String> {
    let path = PathBuf::from(file_path);

    let compressor = VoidCompressor {};
    let compressed = compressor
        .compress(path.clone())
        .map_err(|err| err.to_string())?;

    let bytes = fs::read(compressed).map_err(|err| err.to_string())?;
    let bits = BitVec::<_, Msb0>::from_slice(&bytes);

    let blocker = BitBlocker {};
    let (bit_blocks, blocking_metadata) = blocker.block(bits, 20, 19);
    let encoder: Box<dyn Encoder> = match encoder_type {
        "quaternary" => Box::new(QuaternaryEncoder {}),
        "rotation" => Box::new(RotationEncoder {}),
        "church" => Box::new(ChurchEncoder {}),
        _ => return Err("Selected encoder does not exist.".to_string()),
    };

    let encoded_dna_blocks = bit_blocks
        .iter()
        .map(|bit_block| encoder.encode(bit_block))
        .collect();

    let scaffolder = Scaffolder {};
    let (scaffolded_dna_blocks, scaffold_metadata) =
        scaffolder.add_scaffold(encoded_dna_blocks, 0.20 as f32);

    let out_dir = "metadata";
    fs::create_dir_all(out_dir).unwrap();

    Ok(scaffolded_dna_blocks)
}

#[tauri::command]
fn decode_sequence(file_paths: Vec<&str>) -> Result<String, String> {
    let decoded_sequences: Vec<BitVec<u8, Msb0>> = file_paths
        .iter()
        .map(|file_path| {
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
            decoder.decode(&bases)
        })
        .collect();
    let blocker = BitBlocker {};
    let decoded_file = blocker.deblock(decoded_sequences);
    Ok(decoded_file.to_string())
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
