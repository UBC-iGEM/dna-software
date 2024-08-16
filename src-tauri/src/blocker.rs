use bitvec::{order::Msb0, vec::BitVec};
use petgraph::{graphmap::DiGraphMap, visit::Dfs};
use std::collections::HashMap;

use crate::metadata::MetaData;

pub struct BitBlocker {}
impl BitBlocker {
    pub fn block(
        &self,
        //metadata: MetaData,
        sequence: BitVec<u8, Msb0>,
        per_segment: usize,
        per_overlap: usize,
    ) -> Vec<BitVec<u8, Msb0>> {
        println!("Initial sequence: {}", sequence);
        let mut result = Vec::new();
        let mut start = 0;
        while start < sequence.len() {
            let end = if start + per_segment > sequence.len() {
                sequence.len()
            } else {
                start + per_segment
            };
            result.push(sequence[start..end].to_owned());
            start += per_segment;
            if start < sequence.len() {
                start -= per_overlap;
            }
        }
        println!("Blocked sequences:");
        for (i, chunk) in result.iter().enumerate() {
            println!("Bl{}: {}", i, chunk);
        }
        result
    }

    pub fn rebuild(&self, blocks: Vec<BitVec<u8, Msb0>>, per_overlap: usize) -> BitVec<u8, Msb0> {
        let mut overlaps: HashMap<BitVec<u8, Msb0>, usize> = HashMap::new();
        let mut graph = DiGraphMap::<usize, ()>::new();
        let mut first_index = usize::MAX;

        for (i, block) in blocks.iter().enumerate() {
            let overlap_key = block[block.len() - per_overlap..].to_owned();
            overlaps.insert(overlap_key, i);
        }

        for (i, block) in blocks.iter().enumerate() {
            let query = block[..per_overlap].to_owned();
            if let Some(&result) = overlaps.get(&query) {
                graph.add_edge(result, i, ());
            } else {
                first_index = i;
            }
        }

        if first_index == usize::MAX {
            panic!("Error: No starting block found!");
        }
	else {
	    println!("First index: {}", first_index);
	}

        let mut result = BitVec::<u8, Msb0>::new();
        println!("Overlaps: {:?}", overlaps);
        println!("Graph edges:");
        for edge in graph.all_edges() {
            println!("{:?}", edge);
        }
        let mut dfs = Dfs::new(&graph, first_index);

        while let Some(node_index) = dfs.next(&graph) {
            if result.is_empty() {
                result.extend(&blocks[node_index]);
            } else {
                result.extend(&blocks[node_index][per_overlap..]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::bitvec;
    use rand::{seq::SliceRandom, thread_rng, Rng};

    #[test]
    fn test_rebuild() {
        let blocker = BitBlocker {};
        let blocks: Vec<BitVec<u8, Msb0>> = vec![
            bitvec![u8, Msb0; 1, 1, 1, 0, 1, 0, 0, 1, 0],
            bitvec![u8, Msb0; 0, 0, 0, 0, 1, 0, 1, 1, 1],
            bitvec![u8, Msb0; 0, 1, 0, 1, 1, 1, 0, 0, 1],
            bitvec![u8, Msb0; 0, 0, 1, 1, 0, 0, 1, 1, 0],
        ];
        let per_overlap = 3;

        let expected_result = bitvec![u8, Msb0;
            0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0];

        let result = blocker.rebuild(blocks, per_overlap);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_block() {
        let blocker = BitBlocker {};
        let sequence = bitvec![u8, Msb0; 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0];
        let per_segment = 5;
        let per_overlap = 2;

        let expected_result = vec![
            bitvec![u8, Msb0; 1, 0, 1, 0, 1],
            bitvec![u8, Msb0; 0, 1, 1, 0, 0],
            bitvec![u8, Msb0; 0, 0, 1, 1, 1],
            bitvec![u8, Msb0; 1, 1, 0, 1, 0],
        ];

        let result = blocker.block(sequence, per_segment, per_overlap);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_blocker() {
	// Generate sequence of random length between 40-150 bits
        let mut rng = thread_rng();
        let bitvec_len = rng.gen_range(40..=150);
        let sequence = generate_random_bitvec(&mut rng, bitvec_len);

	// Run the block function on the generated sequence
        let blocker = BitBlocker {};
        let test_sequence = blocker.block(sequence.clone(), 20, 15);

	// Shuffle the BitVecs in the output
        let mut shuffled_sequence = test_sequence.to_owned();
        shuffled_sequence.shuffle(&mut thread_rng());

	// Reconstruct the original and check that sequence and output_sequence are identical.
        let output_sequence = blocker.rebuild(shuffled_sequence, 15);
        assert_eq!(sequence, output_sequence);
    }

    // Create a BitVec with 50% RNG (0 or 1)
    fn generate_random_bitvec(rng: &mut impl Rng, len: usize) -> BitVec<u8, Msb0> {
        let mut bits: BitVec<u8, Msb0> = BitVec::with_capacity(len);
        for _ in 0..len {
            bits.push(rng.gen_bool(0.5));
        }
        bits
    }
}
