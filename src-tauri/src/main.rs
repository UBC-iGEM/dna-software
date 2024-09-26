// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(array_windows)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]

use blocker::BitBlocker;
use ec::TdTAligner;
use fasta::{FastaBase, FastaParser, Parser};
use metadata::{MetaData, Scaffold};
use primer::{Base, MeltingTemperature, Primer, PrimerInfo};
use scaffolder::Scaffolder;
use serde_json::to_string;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use bitvec::{order::Msb0, prelude::BitVec};
use decoder::{ChurchDecoder, Decoder, QuaternaryDecoder, RotationDecoder};
use encoder::{ChurchEncoder, Encoder, QuaternaryEncoder, RotationEncoder};

mod blocker;
mod chaosdna;
mod decoder;
mod ec;
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
fn encode_sequence(encoder_type: &str, file_path: &str) -> Result<Vec<Base>, String> {
    let path = PathBuf::from(file_path);

    let bytes = fs::read(path.clone()).map_err(|err| err.to_string())?;
    let bits = BitVec::<_, Msb0>::from_slice(&bytes);

    let encoder: Box<dyn Encoder> = match encoder_type {
        "quaternary" => Box::new(QuaternaryEncoder {}),
        "rotation" => Box::new(RotationEncoder {}),
        "church" => Box::new(ChurchEncoder {}),
        _ => return Err("Selected encoder does not exist.".to_string()),
    };

    let encoded_dna_blocks = encoder.encode(&bits);
    let scaffolder = Scaffolder {};
    let (scaffolded_dna_blocks, scaffold_metadata) =
        scaffolder.add_scaffold(encoded_dna_blocks.clone(), 0.40 as f32);

    let scaffold = Scaffold {
        scaffolded_bases: scaffold_metadata,
    };
    let metadata = MetaData {
        file_path: file_path.to_string(),
        encoder_type: encoder_type.to_string(),
        compression_type: "lz4".to_string(),
        scaffold: scaffold,
        nucleotide_strand_length: encoded_dna_blocks.len() * encoded_dna_blocks.len(),
    };

    let mut ret = [
        path.file_stem().expect("should work").to_str().unwrap(),
        Some("_metadata").unwrap(),
    ]
    .join("");
    fs::create_dir_all(ret.clone());
    let outpath = Path::new(&ret).join("metadata").with_extension("json");
    let mut output_file = File::create(outpath.as_path()).unwrap();
    let data = serde_json::to_string(&metadata).unwrap();
    output_file.write_all(data.as_bytes());

    Ok(scaffolded_dna_blocks)
}

fn read_metadata_from_file<P: AsRef<Path>>(path: P) -> Result<MetaData, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `MetaData`.
    let m = serde_json::from_reader(reader)?;

    // Return the `MetaData`.
    Ok(m)
}

#[tauri::command]
fn decode_sequence(file_path: &str) -> Result<String, String> {
    let the_file = "tdt_metadata/metadata.json";
    let metadata = read_metadata_from_file(the_file).unwrap();
    let fasta_file_content = fs::read_to_string(PathBuf::from(file_path)).unwrap();
    let fasta_bases = FastaParser::parse_into(&fasta_file_content);
    let corrected_sequence = TdTAligner::compress_align_resolve(&metadata, fasta_bases);
    let decoder = QuaternaryDecoder {};
    let decoded_sequence: BitVec<u8, Msb0> = decoder.decode(&corrected_sequence);
    Ok(decoded_sequence.to_string())
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
