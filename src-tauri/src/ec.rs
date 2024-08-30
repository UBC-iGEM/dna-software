use std::collections::HashMap;

use crate::{
    fasta::FastaBase,
    metadata::{MetaData, Scaffold},
    primer::Base,
};

pub struct BaseCount {
    a: isize,
    t: isize,
    g: isize,
    c: isize,
}

pub struct ScaffoldAlignmentInfo {
    strands: Vec<Vec<Base>>,
    scaffold: HashMap<isize, Base>,
}

pub struct TdTAligner {}
impl TdTAligner {
    pub fn compress_align_resolve(
        metadata: &MetaData,
        fasta_bases: Vec<Vec<FastaBase>>,
    ) -> Vec<Base> {
        // assume bases are homo
        // ignore unresolved bases (N)
        let mut compressed_strands: Vec<Vec<Base>> = fasta_bases
            .iter()
            .map(|synthesized_strand| {
                let mut compressed_strand: Vec<Base> = Vec::new();
                for base in synthesized_strand {
                    let mut curr_base = match base {
                        FastaBase::Base(b) => Some(Base::try_from(*b)),
                        FastaBase::NotBase(b) => None,
                    };
                    if !compressed_strand.is_empty() {
                        let mut prev_base = compressed_strand.last().unwrap();
                        if curr_base.is_some() {
                            if *prev_base != curr_base.unwrap().expect("failure") {
                                compressed_strand.push(curr_base.unwrap().expect("failure"));
                            }
                        }
                    } else {
                        if curr_base.is_some() {
                            compressed_strand.push(curr_base.unwrap().expect("failure"));
                        }
                    }
                }
                compressed_strand
            })
            .collect();
        compressed_strands.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
        let top_thirty_strand_index = (compressed_strands.len() as f32 * 0.30) as isize;
        let top_thirty_strand_len = compressed_strands
            .get(top_thirty_strand_index as usize)
            .unwrap()
            .len();

        let mut scaffolded_strands: Vec<ScaffoldAlignmentInfo> = Vec::new();

        for scaffold in metadata.scaffold.scaffolded_bases.clone() {
            let mut scaffold_length = scaffold.len();
            let mut aligned_strands = Vec::new();
            let mut seen: Vec<Vec<Base>> = Vec::new();
            while let Some(strand) = Some(compressed_strands.remove(0)) {
                if seen.contains(&strand) {
                    break;
                } else {
                    seen.push(strand.clone());
                }
                for (index, base) in &scaffold {
                    // check that scaffold exists
                    let synthesized_base = strand.get(*index as usize);
                    if *base == *synthesized_base.unwrap() {
                        scaffold_length -= 1;
                    }
                }
                if scaffold_length != 0 {
                    // check if length is long enough
                    if strand.len() >= top_thirty_strand_len && scaffold_length <= 1 {
                        aligned_strands.push(strand.clone());
                    } else {
                        compressed_strands.push(strand);
                    }
                } else {
                    aligned_strands.push(strand.clone());
                }
                scaffold_length = scaffold.len();
            }
            scaffolded_strands.push(ScaffoldAlignmentInfo {
                scaffold: scaffold,
                strands: aligned_strands,
            });
        }
        let strand_length = metadata.nucleotide_sequence_length;
        scaffolded_strands.iter().map(|scaffold_alignment_info| {
            let base_index_probabilities: Vec<BaseCount> = Vec::new();
            let scaffold = &scaffold_alignment_info.scaffold;
            let strands = &scaffold_alignment_info.strands;
            for strand in strands {
                for (i, scaffold_base) in scaffold {}
            }
        });
        todo!()
    }
}
