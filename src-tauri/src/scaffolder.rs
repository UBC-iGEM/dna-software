use std::collections::HashMap;

use itertools::Itertools;
use rand::{distributions::Standard, Rng};

use crate::{metadata::MetaData, primer::Base};

pub struct Scaffolder {}
impl Scaffolder {
    pub fn add_scaffold(
        &self,
        encoded_dna_sequences: Vec<Vec<Base>>,
        frac_scaffold_bases: f32,
    ) -> (Vec<Vec<Base>>, Vec<HashMap<isize, Base>>) {
        let mut scaffolds: Vec<HashMap<isize, Base>> = vec![];
        let mut added_scaffold_bases = 0;
        (
            encoded_dna_sequences
                .iter()
                .map(|dna_sequence| {
                    let mut scaffold_hashmap = HashMap::new();
                    let mut scaffolded_sequence: Vec<Base> = vec![];
                    for (i, base) in dna_sequence.into_iter().enumerate() {
                        if rand::thread_rng().gen_bool(frac_scaffold_bases.into()) {
                            let scaffolded_position = added_scaffold_bases + i;
                            added_scaffold_bases += 1;
                            let mut scaffold_base: Base = *base;
                            while scaffold_base == *base {
                                scaffold_base = rand::thread_rng()
                                    .sample_iter(Standard)
                                    .take(1)
                                    .collect::<Vec<Base>>()[0];
                            }
                            scaffold_hashmap.insert(scaffolded_position as isize, scaffold_base);
                            scaffolded_sequence.push(scaffold_base);
                        }
                        scaffolded_sequence.push(*base);
                    }
                    scaffolds.push(scaffold_hashmap);
                    scaffolded_sequence
                })
                .collect_vec(),
            scaffolds,
        )
    }
}
