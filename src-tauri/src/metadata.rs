use crate::{compressor::Compressor, encoder::Encoder};

// TODO: don't know if lifetimes are neccesarry

pub struct MetaData<'a> {
    pub file_path: &'a str,
    pub encoder_type: &'a str,
    pub compression_type: &'a str,
    pub num_bit_sequences: usize,
    pub bit_sequence_length: usize,
}

impl MetaData<'_> {
    fn write_metadata(&self) {
        todo!()
    }
    fn read_metadata(&self) {
        todo!()
    }
}
