use bitvec::{field::BitField, order::Msb0, slice::BitSlice, vec::BitVec};
use itertools::Itertools;

g2p::g2p!(GF16, 4);

fn factorial(n: usize) -> usize {
    (1..n).product()
}

struct GF {}
impl GF {}

// TODO: consider making a trait if we do other error correction strategies that just operate and return bits

struct GCEncoder {}
impl GCEncoder {
    fn bin_to_qblocks(bin_seq: &mut BitVec<u8, Msb0>) -> Vec<&BitSlice<u8, Msb0>> {
        let k = bin_seq.len();
        let k_next_power_of_two = k.next_power_of_two();
        let block_length = (k_next_power_of_two as f64).log2() as usize;
        let num_symbols = k_next_power_of_two.div_ceil(block_length);
        let leading_zeros_needed = (num_symbols * block_length) - k;

        for _ in 0..leading_zeros_needed {
            bin_seq.insert(0, false);
        }

        assert_eq!(bin_seq.len(), num_symbols * block_length);

        let q = (2 as u32).pow((k_next_power_of_two as f64).log2() as u32);

        assert_eq!(q as usize, k_next_power_of_two);

        bin_seq.chunks(block_length).collect()
    }

    fn map_block_to_finite(blocks: Vec<&BitSlice<u8, Msb0>>) -> Vec<GF16> {
        blocks
            .iter()
            .map(|block| Into::<GF16>::into(block.load_be::<u8>()))
            .collect_vec()
    }

    // 2. convert to systematic MDS code
    fn convert_to_mds(blocks: Vec<&BitSlice<u8, Msb0>>, num_del: usize) {
        let deletions = num_del;
        let c = deletions + 1;
        todo!()
    }

    // 3. q-ary to binary
    fn mds_to_bin(blocks: Vec<&BitSlice<u8, Msb0>>) -> BitVec<u8, Msb0> {
        blocks.into_iter().flatten().collect()
    }
}

struct GCDecoder {}
impl GCDecoder {
    // 0. decode parity symbols
    // 1. make assumption on which block the bit deletion has occured (guess part)
    // 2. chunk the bits accordingly, treat the affected the block as erased and check whether obtained sequences is consistent with the parities (checking part)
    // 3. go over all the possibilities
    fn guess_deletion(k: usize, delta: usize, bin_seq: &mut BitVec<u8, Msb0>) {
        let log_k = (k as f64).log2();
        let num_blocks = k.div_ceil(log_k as usize);
        let binary_code_word_len = k as f64 + 2 as f64 * log_k;

        // the number of ways to distribute delta deletions amoung k / log k blocks
        // C(k / log k + delta - 1, delta)
        // for initial purpose, let's assume delta is 1
        let deletion_combinations_length =
            (num_blocks - delta + 1..=num_blocks).product::<usize>() / factorial(delta);

        // assume all EC bits are recovered

        // generate all the possible deletion combinations
        let deletion_combinations = (1..num_blocks).combinations(delta);
        deletion_combinations.map(|blocks_with_possible_deletions| {});
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use bitvec::{order::Msb0, vec::BitVec};

    use crate::gc::GCEncoder;

    use super::GCDecoder;

    #[quickcheck]
    fn test_qblocking(bytes: Vec<u8>) -> bool {
        if bytes.len() != 0 {
            let mut bits = BitVec::from_vec(bytes.clone());
            let bits_to_compare: BitVec<u8, Msb0> = BitVec::from_vec(bytes);
            let qblocks = GCEncoder::bin_to_qblocks(&mut bits);
            let bits = GCEncoder::mds_to_bin(qblocks);
            bits_to_compare.count_ones() == bits.count_ones()
        } else {
            true
        }
    }

    #[test]
    fn test_deletions() {
        let delta = 1;
        let k = 8 as usize;
        let log_k = (k as f64).log2();
        let num_blocks = k.div_ceil(log_k as usize);
        let binary_code_word_len = k as f64 + 2 as f64 * log_k;
        let deletion_combinations = (1..num_blocks).combinations(delta);
    }
    #[test]
    fn test_basic() {
        let bytes = vec![8];
        let mut bits = BitVec::from_vec(bytes.clone());
        let bits_to_compare: BitVec<u8, Msb0> = BitVec::from_vec(bytes);
        let qblocks = GCEncoder::bin_to_qblocks(&mut bits);
        let returned_bits = GCEncoder::mds_to_bin(qblocks);
        assert_eq!(bits_to_compare.count_ones(), returned_bits.count_ones());
    }
}
