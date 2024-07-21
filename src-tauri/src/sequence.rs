use std::{fs, io, path::Path};

use bitvec::prelude::BitVec;

use crate::{
    compressor::Compressor,
    encoder::Encoder,
    primer::{Base, Primer},
};

fn encode(
    path: impl AsRef<Path>,
    primer: Primer,
    compressor: impl Compressor,
    encoder: impl Encoder,
) -> io::Result<Vec<Base>> {
    let file = fs::read(path)?;
    let compressed = compressor.compress(file);
    let bit_sequence = BitVec::from_vec(compressed);
    Ok(encoder.encode(bit_sequence))
}

fn decode() {}
