use std::{fs, io, path::PathBuf};

use bitvec::prelude::*;

use crate::{
    compressor::Compressor,
    encoder::Encoder,
    primer::{Base, Primer},
};

fn encode(
    path: PathBuf,
    primer: Primer,
    compressor: impl Compressor,
    encoder: impl Encoder,
) -> io::Result<Vec<Base>> {
    let compressed = compressor.compress(path)?;
    let compressed_bytes = fs::read(compressed)?;
    let bit_sequence = BitVec::<u8, Msb0>::from_slice(&compressed_bytes);
    Ok(encoder.encode(bit_sequence))
}

fn decode() {}
