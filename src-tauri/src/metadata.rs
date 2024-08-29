use std::collections::HashMap;

use crate::primer::Base;

// TODO: don't know if lifetimes are neccesarry

pub struct Scaffold {
    pub scaffolded_bases: Vec<HashMap<isize, Base>>,
}

#[derive(Clone, Copy)]
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
