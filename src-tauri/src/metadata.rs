use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::primer::Base;

// TODO: don't know if lifetimes are neccesarry

#[derive(Clone, Serialize, Deserialize)]
pub struct Scaffold {
    pub scaffolded_bases: HashMap<isize, Base>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub file_path: String,
    pub encoder_type: String,
    pub compression_type: String,
    pub scaffold: Scaffold,
    pub nucleotide_strand_length: usize,
}
