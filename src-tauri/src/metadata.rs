use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{compressor::Compressor, encoder::Encoder, primer::Base};

// TODO: don't know if lifetimes are neccesarry

#[derive(Serialize, Deserialize)]
pub struct Scaffold {
    pub scaffolded_bases: Vec<HashMap<isize, Base>>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct MetaData<'a> {
    pub file_path: &'a str,
    pub encoder_type: &'a str,
    pub compression_type: &'a str,
    pub num_bit_sequences: usize,
    pub bit_sequence_length: usize,
    pub scaffold: &'a Scaffold,
}

impl MetaData<'_> {
    fn write_metadata(&self) {
        todo!()
    }
    fn read_metadata(&self) {
        todo!()
    }
}
