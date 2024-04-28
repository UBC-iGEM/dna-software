use bitvec::{order::Msb0, prelude::BitVec};

use crate::primer::Base;

pub trait Encoder {
    fn encode(&self, input: BitVec<u8, Msb0>) -> Vec<Base>;
}

pub struct DummyEncoder {}
impl Encoder for DummyEncoder {
    fn encode(&self, input: BitVec<u8, Msb0>) -> Vec<Base> {
        todo!()
    }
}

pub struct RotationEncoder {}
impl Encoder for RotationEncoder {
    fn encode(&self, input: BitVec<u8, Msb0>) -> Vec<Base> {
        todo!()
    }
}

pub struct HEDGESEncoder {}
impl Encoder for HEDGESEncoder {
    fn encode(&self, input: BitVec<u8, Msb0>) -> Vec<Base> {
        todo!()
    }
}
