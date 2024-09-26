use std::collections::HashMap;

use itertools::Itertools;
use rand::{distributions::Standard, Rng};

use crate::{metadata::MetaData, primer::Base};

pub struct Scaffolder {}
impl Scaffolder {
    pub fn add_scaffold(
        &self,
        encoded_dna_sequence: Vec<Base>,
        frac_scaffold_bases: f32,
    ) -> (Vec<Base>, HashMap<isize, Base>) {
        let mut scaffold_hashmap = HashMap::new();
        let mut scaffolded_sequence: Vec<Base> = vec![];
        let mut added_scaffold_bases = 0;

        for (i, base) in encoded_dna_sequence.clone().into_iter().enumerate() {
            scaffolded_sequence.push(base);
            if rand::thread_rng().gen_bool(frac_scaffold_bases.into()) {
                let scaffolded_position = added_scaffold_bases + i;
                added_scaffold_bases += 1;
                let next_base = encoded_dna_sequence.get(i + 1).or(Some(&Base::A));
                let mut scaffold_base: Base = base;
                while scaffold_base == base || scaffold_base == *next_base.unwrap() {
                    scaffold_base = rand::thread_rng()
                        .sample_iter(Standard)
                        .take(1)
                        .collect::<Vec<Base>>()[0];
                }
                scaffold_hashmap.insert((scaffolded_position as isize) + 1, scaffold_base);
                scaffolded_sequence.push(scaffold_base);
            }
        }
        (scaffolded_sequence, scaffold_hashmap)
    }
}
