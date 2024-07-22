use bitvec::{order::Msb0, slice::BitSlice, vec::BitVec};
use g2p;
struct GF {}
impl GF {}

// TODO: consider making a trait if we do other error correction strategies that just operate and return bits

struct GcEncoder {}
impl GcEncoder {
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

        let q_ary: Vec<&BitSlice<u8, Msb0>> = bin_seq.chunks(block_length).collect();

        q_ary
    }

    fn map_block_to_finite(blocks: Vec<&BitSlice<u8, Msb0>>) {
        todo!()
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

#[cfg(test)]
mod tests {
    use bitvec::{order::Msb0, vec::BitVec};

    use crate::gc::GcEncoder;

    #[quickcheck]
    fn test_qblocking(bytes: Vec<u8>) -> bool {
        if bytes.len() != 0 {
            let mut bits = BitVec::from_vec(bytes.clone());
            let bits_to_compare: BitVec<u8, Msb0> = BitVec::from_vec(bytes);
            let qblocks = GcEncoder::bin_to_qblocks(&mut bits);
            let bits = GcEncoder::mds_to_bin(qblocks);
            bits_to_compare.count_ones() == bits.count_ones()
        } else {
            true
        }
    }

    #[test]
    fn test_basic() {
        let bytes = vec![8];
        let mut bits = BitVec::from_vec(bytes.clone());
        let bits_to_compare: BitVec<u8, Msb0> = BitVec::from_vec(bytes);
        let qblocks = GcEncoder::bin_to_qblocks(&mut bits);
        let returned_bits = GcEncoder::mds_to_bin(qblocks);
        assert_eq!(bits_to_compare.count_ones(), returned_bits.count_ones());
    }
}
