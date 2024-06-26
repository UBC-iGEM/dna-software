use std::{fmt::Display, iter};

use itertools::Itertools;
use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum MeltingTemperatureConstraint {
    Below,
    Above,
}

// make specific ranges for different lengths

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct MeltingTemperature {
    temperature: usize,
    constraint: MeltingTemperatureConstraint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub enum Base {
    A,
    T,
    G,
    C,
}

impl Base {
    fn complement(&self) -> Self {
        match self {
            Base::A => Base::T,
            Base::T => Base::A,
            Base::G => Base::C,
            Base::C => Base::G,
        }
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Base::A => 'A',
            Base::T => 'T',
            Base::G => 'G',
            Base::C => 'C',
        };
        char.fmt(f)
    }
}

impl TryFrom<char> for Base {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Base::A),
            'T' => Ok(Base::T),
            'G' => Ok(Base::G),
            'C' => Ok(Base::C),
            other => Err(format!("Expected one of A, T, G, C, got {other}")),
        }
    }
}

impl Distribution<Base> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Base {
        let idx = rng.gen_range(0..4);
        match idx {
            0 => Base::A,
            1 => Base::T,
            2 => Base::G,
            3 => Base::C,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Primer(Vec<Base>);

impl Display for Primer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            b.fmt(f)?
        }
        Ok(())
    }
}

impl TryFrom<&str> for Primer {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .map(Base::try_from)
            .collect::<Result<_, _>>()
            .map(Primer)
    }
}

impl Primer {
    const POOL_SIZE: usize = 1000;
    fn generate_initial_primers(len: usize, len_g: usize) -> Vec<Self> {
        (0..Self::POOL_SIZE)
            .map(|_| {
                Self(
                    rand::thread_rng()
                        .sample_iter(Standard)
                        .take(len)
                        .chain(iter::repeat(Base::G).take(len_g))
                        .collect(),
                )
            })
            .collect()
    }

    fn melting_temperature(&self) -> f64 {
        let num_gc = self.num_gc() as f64;

        4.0 * num_gc + 2.0 * (self.0.len() as f64 - num_gc)
    }

    fn num_gc(&self) -> usize {
        self.0
            .iter()
            .filter(|b| matches!(b, Base::G | Base::C))
            .count()
    }

    fn good_gc_content(&self) -> bool {
        (0.4..=0.6).contains(&self.gc_content())
    }

    fn starts_with_gc(&self) -> bool {
        matches!(
            self.0.first().expect("Primer should not be empty."),
            Base::G | Base::C
        )
    }

    fn reverse_complement(&self) -> Self {
        Self(self.0.iter().rev().map(|b| b.complement()).collect())
    }

    fn contains(&self, other: &Self) -> bool {
        self.0.windows(other.0.len()).any(|w| w == other.0)
    }

    fn has_hairpin(&self) -> bool {
        const MIN_HAIRPIN_LOOP: usize = 4;
        const MIN_HAIRPIN_LENGTH: usize = 3;
        for hairpin_len in MIN_HAIRPIN_LENGTH..self.0.len() - MIN_HAIRPIN_LOOP {
            for i in 0..self.0.len() - hairpin_len - MIN_HAIRPIN_LOOP {
                let hairpin_candidate =
                    Primer(self.0[i..i + hairpin_len].to_vec()).reverse_complement();
                let forward_primer = Primer(self.0[i + hairpin_len + MIN_HAIRPIN_LOOP..].to_vec());
                if forward_primer.contains(&hairpin_candidate) {
                    return true;
                }
            }
        }
        false
    }

    // minimum: 36, maximum: 60
    fn good_melting_temperature(&self, melting_temperature: MeltingTemperature) -> bool {
        let range = match melting_temperature.constraint {
            MeltingTemperatureConstraint::Below => 36.0..=melting_temperature.temperature as f64,
            MeltingTemperatureConstraint::Above => melting_temperature.temperature as f64..=60.0,
        };
        range.contains(&self.melting_temperature())
    }

    fn fitness(&self, melting_temperature: MeltingTemperature) -> f64 {
        let mut score = 0_f64;

        // Start with G or C
        if self.starts_with_gc() {
            score += 5.0
        }

        // GC content: 40-60%
        let gc_content = self.gc_content();
        if self.good_gc_content() {
            score += (1.0 - (0.52 - gc_content).abs()) * 5.0;
        }

        // Melting temperature? 36-46
        // Range between melting T of thermostable and wt tdt
        // salt???
        if self.good_melting_temperature(melting_temperature) {
            score += 1.0
                / (self.melting_temperature() as isize - melting_temperature.temperature as isize)
                    .pow(2) as f64;
        }

        // Secondary structure, only self primer hairpin
        if !self.has_hairpin() {
            score += 10.0;
        }

        score
    }

    fn gc_content(&self) -> f64 {
        self.num_gc() as f64 / self.0.len() as f64
    }

    pub fn generate(
        total_len: usize,
        melting_temperature: MeltingTemperature,
        len_g: usize,
    ) -> Vec<Self> {
        let len_no_g = total_len - len_g;
        let mut initial_primers = Self::generate_initial_primers(len_no_g, len_g);
        const MUTATION_RATE: f64 = 0.8;

        loop {
            let parents = initial_primers
                .choose_multiple_weighted(&mut rand::thread_rng(), Self::POOL_SIZE, |p| {
                    p.fitness(melting_temperature)
                })
                .unwrap();

            let children = parents.tuples().flat_map(|(p1, p2)| {
                let index = rand::thread_rng().gen_range(0..len_no_g);
                let c1 = Self([&p1.0[..index], &p2.0[index..]].concat());
                let c2 = Self([&p2.0[..index], &p1.0[index..]].concat());

                [c1, c2]
            });

            let mutated_children: Vec<_> = children
                .map(|mut c| {
                    if rand::thread_rng().gen_bool(MUTATION_RATE) {
                        let index = rand::thread_rng().gen_range(0..len_no_g);
                        c.0[index] = rand::random();
                    }
                    c
                })
                .collect();

            let good_primers: Vec<_> = mutated_children
                .iter()
                .filter(|p| {
                    p.good_gc_content()
                        && p.starts_with_gc()
                        && p.good_melting_temperature(melting_temperature)
                        && !p.has_hairpin()
                })
                .cloned()
                .collect();

            if !good_primers.is_empty() {
                return good_primers;
            } else {
                initial_primers = mutated_children;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let primers = Primer::generate(
            18,
            MeltingTemperature {
                temperature: 46,
                constraint: MeltingTemperatureConstraint::Above,
            },
            4,
        );
        assert!(!primers.is_empty());
    }

    #[test]
    fn secondary_structure() {
        let test_cases = [
            ("GCATACTATCATTCGGGG", false),
            ("GCTAATGGGTATTGGGGG", true),
            // ("GGACTATGCTATTGGGGG", false),
            ("ATCGATCAAAAGATCG", true),
            ("CCCCATGCATCCCC", false),
        ];
        for (primer_str, has_hairpin) in test_cases {
            let primer = Primer::try_from(primer_str).unwrap();
            assert_eq!(primer.has_hairpin(), has_hairpin);
        }
    }

    #[test]
    fn twenty_four_nt() {}
}
