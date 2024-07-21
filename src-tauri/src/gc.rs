// 1. binary to q-ary
fn bin_to_qblocks(bin_seq: &str) -> Result<Vec<String>, String> {
    let k = bin_seq.len();
    let q = (k as f64).log2() as usize;

    let block_size = k / q;
    let mut qary_seq = Vec::new();

    for i in 0..q {
        let start = i * block_size;
        let end = start + block_size;
        let block = &bin_seq[start..end];
        qary_seq.push(block.to_string());
    }

    Ok(qary_seq)
}

fn map_block_to_finite(blocks: Vec<String>) {
    
}

// 2. convert to systematic MDS code
fn convert_to_mds(blocks: Vec, num_del:i8) {
    let deletions = num_del;
    let c = deletions + 1;


}

// 3. q-ary to binary
fn mds_to_bin(blocks: Vec) {

}

fn main() {
    let bin_seq = "1100110011001100"; // Example binary sequence

    match bin_to_qblocks(bin_seq) {
        Ok(qary_seq) => {
            println!("Q-ary sequence: {:?}", qary_seq);
        }
        Err(e) => {
            println!("Error: could not convery binary to q-blocks", e);
        }
    }
}