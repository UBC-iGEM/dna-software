use bitvec::prelude::BitVec;

pub trait Compressor {
    fn compress(&self, input: Vec<u8>) -> Vec<u8>;
}

pub struct VoidCompressor {}
impl Compressor for VoidCompressor {
    fn compress(&self, input: Vec<u8>) -> Vec<u8> {
        todo!()
    }
}
