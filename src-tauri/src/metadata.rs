use crate::{compressor::Compressor, encoder::Encoder};

// TODO: don't know if lifetimes are neccesarry

pub struct MetaData<'a> {
    file_name: &'a str,
    encoder_type: &'a dyn Encoder,
    num_bit_sequences: usize,
    bit_sequence_length: usize,
    compression_type: dyn Compressor,
}

impl MetaData<'_> {
    fn write_metadata(&self) {
        todo!()
    }
    fn read_metadata(&self) {
        todo!()
    }
}
