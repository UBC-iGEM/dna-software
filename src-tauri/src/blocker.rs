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

        assert_eq!(result.len(), expected_result.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expected_result[i]);
        }
    }
}
