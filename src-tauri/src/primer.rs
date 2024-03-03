use std::iter;

use itertools::Itertools;
use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeltingTemperatureConstraint {
    Below,
    Above,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeltingTemperature {
    temperature: usize,
    constraint: MeltingTemperatureConstraint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    A,
    T,
    G,
    C,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Primer(Vec<Base>);

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
        // TODO: only hairpin?

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
        dbg!(primers.len());
        assert!(!primers.is_empty());
    }
}
