use std::collections::HashMap;

use bio::{
    alignment::{pairwise::Aligner, AlignmentOperation},
    data_structures::qgram_index::Match,
};

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

pub struct AlignmentInfo {
    x_start: isize,
    y_start: isize,
    matches: Vec<AlignmentOperation>,
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

        let most_probable_strand: Vec<Base> = Vec::new();

        let scaffold = &metadata.scaffold.scaffolded_bases;
        for i in 0..top_thirty_strand_index {
            for (index, base) in scaffold.into_iter() {}
        }
        todo!()
    }
}
