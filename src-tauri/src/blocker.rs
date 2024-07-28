use crate::primer::Base;

#[derive(Clone)]
enum DataType {
    Base(Base),
    BitSlice(bool),
}

pub trait Blocker {
    fn block(
        &self,
        sequence: Vec<DataType>,
        bits_per_segment: usize,
        bits_per_overlap: usize,
    ) -> Vec<Vec<DataType>>;
    fn deblock(&self, segments: Vec<Vec<DataType>>) -> Vec<DataType>;
}

pub struct DNABlocker {}
impl Blocker for DNABlocker {
    fn block(
        &self,
        sequence: Vec<DataType>,
        bits_per_segment: usize,
        bits_per_overlap: usize,
    ) -> Vec<Vec<DataType>> {
        let mut iter = sequence.windows(bits_per_segment);
        let mut dna_sequences = Vec::new();
        while let Some(next_segment) = iter.next() {
            dna_sequences.push(next_segment.to_vec());
        }
        dna_sequences
    }
    fn deblock(&self, dna_blocks: Vec<Vec<DataType>>) -> Vec<DataType> {
        // 1. find overlaps with dynamic programming
        let mut grid_raw = vec![0; dna_blocks.len()];
        let mut grid_base: Vec<_> = grid_raw
            .as_mut_slice()
            .chunks_mut(dna_blocks.len())
            .collect();
        let mut overlap_matrix = grid_base.as_mut_slice();

        &dna_blocks
            .iter()
            .map(|dna_seg| &dna_blocks.iter().map(|second_dna_seg| {}))

        // 2.
        // 3.
    }
}
