use bitvec::{order::Msb0, vec::BitVec};

use crate::metadata::MetaData;

pub struct BitBlocker {}
impl BitBlocker {
    pub fn block(
        &self,
        mut metadata: MetaData,
        sequence: BitVec<u8, Msb0>,
        per_segment: usize,
        per_overlap: usize,
    ) -> Vec<BitVec<u8, Msb0>> {
        todo!()
    }
    pub fn deblock(&self, sequences: Vec<BitVec<u8, Msb0>>) -> BitVec<u8, Msb0> {
        todo!()
    }
}
