use std::{cmp, f64::INFINITY};

use crate::primer::Base;

#[derive(Clone)]
pub enum DataType {
    Base(Base),
    Bit(bool),
}

pub trait Blocker {
    fn block(
        &self,
        sequence: Vec<DataType>,
        per_segment: usize,
        per_overlap: usize,
    ) -> Vec<Vec<DataType>>;
    fn deblock(&self, blocks: Vec<Vec<DataType>>) -> Vec<DataType>;
}

pub struct BitBlocker {}
impl Blocker for BitBlocker {
    fn block(
        &self,
        sequence: Vec<DataType>,
        per_segment: usize,
        per_overlap: usize,
    ) -> Vec<Vec<DataType>> {
        todo!()
    }

    fn deblock(&self, blocks: Vec<Vec<DataType>>) -> Vec<DataType> {
        todo!()
    }
}

pub struct DNABlocker {}
impl Blocker for DNABlocker {
    fn block(
        &self,
        sequence: Vec<DataType>,
        per_segment: usize,
        per_overlap: usize,
    ) -> Vec<Vec<DataType>> {
        let mut iter = sequence.windows(per_segment);
        let mut dna_sequences = Vec::new();
        while let Some(next_segment) = iter.next() {
            dna_sequences.push(next_segment.to_vec());
        }
        dna_sequences
    }

    fn deblock(&self, dna_blocks: Vec<Vec<DataType>>) -> Vec<DataType> {
        // 1. find overlaps with dynamic programming
        for first_dna_seg in &dna_blocks {
            for second_dna_seg in &dna_blocks {
                // TODO: fill_in_overlap_matrix(first_dna_seg, second_dna_seg);
            }
        }
        todo!()
        // 2.
        // 3.
    }
}

fn init_overlap_matrix(overlap_matrix: &mut [&mut [f64]]) {
    // first column gets 0s
    let dimension = overlap_matrix.len();
    for j in 0..dimension {
        overlap_matrix[j][0] = 0 as f64;
    }
    // first row get infinity
    for i in 0..dimension {
        overlap_matrix[0][i] = INFINITY;
    }
}

fn min_dist(
    overlap_matrix: &mut [&mut [f64]],
    first_dna_seg: &Vec<Base>,
    second_dna_seg: &Vec<Base>,
    i: f64,
    j: f64,
) -> usize {
    let s = vec![
        vec![0, 4, 2, 4, 8],
        vec![4, 0, 4, 2, 8],
        vec![2, 4, 0, 4, 8],
        vec![4, 2, 4, 0, 8],
        vec![8, 8, 8, 8, 8],
    ];

    let i_minus_1_s_index = match first_dna_seg[i as usize - 1] {
        Base::A => 0,
        Base::T => 3,
        Base::G => 2,
        Base::C => 1,
    };

    let j_minus_1_s_index = match second_dna_seg[j as usize - 1] {
        Base::A => 0,
        Base::T => 3,
        Base::G => 2,
        Base::C => 1,
    };

    let i_minus_1 = overlap_matrix[i as usize - 1][j as usize] + s[i_minus_1_s_index][4] as f64;
    let j_minus_1 = overlap_matrix[i as usize][j as usize - 1] + s[4][j_minus_1_s_index] as f64;
    let i_minus_1_j_minus_1 = overlap_matrix[i as usize - 1][j as usize - 1]
        + s[i_minus_1_s_index][j_minus_1_s_index] as f64;

    cmp::min(
        i_minus_1 as usize,
        cmp::min(j_minus_1 as usize, i_minus_1_j_minus_1 as usize),
    )
}

fn fill_in_overlap_matrix(first_dna_seg: &Vec<Base>, second_dna_seg: &Vec<Base>) {
    let dimension = first_dna_seg.len();
    let mut grid_raw = vec![0 as f64; dimension];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(dimension).collect();

    let mut overlap_matrix = grid_base.as_mut_slice();

    init_overlap_matrix(overlap_matrix);

    for i in 0..dimension {
        for j in 0..dimension {
            overlap_matrix[i][j] = min_dist(
                overlap_matrix,
                &first_dna_seg,
                &second_dna_seg,
                i as f64,
                j as f64,
            ) as f64;
        }
    }
    todo!()
}
