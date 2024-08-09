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
        let mut result = Vec::new();
        let mut index = 0;
        while index <= sequence.len() {
            result.push(
                sequence
                    .clone()
                    .slice(index..index + per_segment)
                    .to_owned(),
            );
            index += per_segment - per_overlap;
        }
        result
    }
}
