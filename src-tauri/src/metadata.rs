use crate::{compressor::Compressor, encoder::Encoder};

// TODO: don't know if lifetimes are neccesarry

#[derive(Clone, Copy)]
pub struct MetaData<'a> {
    pub file_path: &'a str,
    pub encoder_type: &'a str,
    pub compression_type: &'a str,
    pub bit_sequence_overlaps: usize,
}

impl MetaData<'_> {
    fn write_metadata(&self) {
        todo!()
    }
    fn read_metadata(&self) {
        todo!()
    }
}
