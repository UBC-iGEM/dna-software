use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::primer::Base;

// TODO: don't know if lifetimes are neccesarry

#[derive(Clone, Serialize, Deserialize)]
pub struct Scaffold {
    pub scaffolded_bases: Vec<HashMap<isize, Base>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub file_path: String,
    pub encoder_type: String,
    pub compression_type: String,
    pub num_bit_sequences: usize,
    pub bit_sequence_length: usize,
    pub nucleotide_sequence_length: usize,
    pub scaffold: Scaffold,
}

impl MetaData {
    fn write_metadata(&self) {
        todo!()
    }
    fn read_metadata(&self) {
        todo!()
    }
}
