use bitvec::{order::Msb0, vec::BitVec};
use petgraph::{graphmap::DiGraphMap, visit::Dfs};
use std::{collections::HashMap, error::Error};

pub struct BitBlocker {}
impl BitBlocker {
    pub fn block(
        &self,
        sequence: BitVec<u8, Msb0>,
        per_segment: usize,
        per_overlap: usize,
    ) -> Result<Vec<BitVec<u8, Msb0>>, std::io::Error> {
        // Return errors if block has improper parameters/inputs.
        if sequence.len() <= per_segment {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Blocking failed: initial sequence too short"));
        }
        if per_overlap >= per_segment {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Blocking failed: per_overlap is larger or equal to per_segment"));
        }
        // Print out debugging info
        println!("Initial sequence: {}", sequence);

        let mut result = Vec::new();
        let mut start = 0;
        let mut overlaps: HashMap<BitVec<u8, Msb0>, ()> = HashMap::new();

        while start < sequence.len() {
            // Find the start and end values of the current block.
            let mut end = if start + per_segment > sequence.len() {
                sequence.len()
            } else {
                start + per_segment
            };
            // If the overlap for this block already exists, return error.
            if overlaps.contains_key(&sequence[end - per_overlap..end]) {
                return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Blocking failed: unable to build unique blocks. Try increasing the value of per_overlaps."));
            }
            // Push the result and push overlaps to its hashmap.
            result.push(sequence[start..end].to_owned());
            overlaps.insert(sequence[end - per_overlap..end].to_owned(),());
            // Move start to a new point. If start isn't at the end, ensure the overlap is included in the next block.
            start += per_segment;
            if start < sequence.len() {
                start -= per_overlap;
            }
        }
        // Output blocks for debugging.
        println!("Blocked sequences:");
        for (i, chunk) in result.iter().enumerate() {
            println!("Bl{}: {}", i, chunk);
        }
        Ok(result)
    }

    pub fn rebuild(&self, blocks: Vec<BitVec<u8, Msb0>>, per_overlap: usize) -> Result<BitVec<u8, Msb0>, std::io::Error> {
        let mut overlaps: HashMap<BitVec<u8, Msb0>, usize> = HashMap::new();
        let mut graph = DiGraphMap::<usize, ()>::new();
        let mut first_index = usize::MAX;

        // For each "chunk" in the input, ensure its end overlap are unique.
        // If so, add its end overlap to a hashmap as the key with the current chunk indice as the value.
        for (i, chunk) in blocks.iter().enumerate() {
            let overlap_key = chunk[chunk.len() - per_overlap..].to_owned();
            if overlaps.contains_key(&chunk[chunk.len() - per_overlap..]) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Reconstruction failed: non-unique overlaps detected"));   
            }
            overlaps.insert(overlap_key, i);
        }

        // For each chunk, match its beginning overlap to the corresponding end overlap in the hashmap.
        // Create a graph edge from the end indice to this indice.
        // If no match exists, set the block to the first indice.
        for (i, chunk) in blocks.iter().enumerate() {
            let query = chunk[..per_overlap].to_owned();
            if let Some(&result) = overlaps.get(&query) {
                graph.add_edge(result, i, ());
            } else {
                first_index = i;
            }
        }

        // If the first indice isn't found, return error. Else, print for debugging.
        if first_index == usize::MAX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Reconstruction failed: No starting block detected.")); 
        } else {
            println!("First index: {}", first_index);
        }

        // Output the unordered sequences for debugging.
        let mut result = BitVec::<u8, Msb0>::new();
        println!("Shuffled sequences: ");
        for (i, chunk) in blocks.iter().enumerate() {
            println!("Bl{}: {}", i, chunk);
        }

        // Output graph edges for debugging.
        println!("Graph edges:");
        for edge in graph.all_edges() {
            println!("{:?}", edge);
        }

        // Create a depth-first search graph.
        // Start at the first indice, and move to the last edge.
        let mut dfs = Dfs::new(&graph, first_index);
        while let Some(node_index) = dfs.next(&graph) {
            if result.is_empty() {
                result.extend(&blocks[node_index]);
            } else {
                result.extend(&blocks[node_index][per_overlap..]);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, prelude::SliceRandom};

    #[quickcheck]
    fn test_blocker(sequence_bits: Vec<u8>) {
        let sequence = BitVec::from_vec(sequence_bits.clone());
        let blocker = BitBlocker {};

        // Run block. If it does not return an error, proceed. Else, exit.
        let test_sequence_result = blocker.block(sequence.clone(), 20, 15);
        let test_sequence: Vec<BitVec<u8, Msb0>>; 
        match test_sequence_result {
            Ok(reconstructed_data) => {
                test_sequence = reconstructed_data;
            }
            Err(error) => {
                eprintln!("Error during blocking, test aborted. {}", error);
                return;
            }
        }

        // Shuffle the BitVecs in the output
        let mut shuffled_sequence = test_sequence.to_owned();
        shuffled_sequence.shuffle(&mut thread_rng());

        // Reconstruct the original sequence. If an error is returned, print it and exit.
        let output_sequence: BitVec<u8, Msb0>;
        let output_sequence_result = blocker.rebuild(shuffled_sequence, 15);
        match output_sequence_result {
            Ok(output_sequence_data) => {
                output_sequence = output_sequence_data; 
            }
            Err(error) => {
                eprintln!("Error during rebuilding: {}", error);
                return;
            }
        }

        // Check that the end and beginning sequences match.
        assert_eq!(sequence, output_sequence);
        println!("Test successful ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^>^")
    }
}
