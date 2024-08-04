use std::collections::HashMap;

use bitvec::{order::Msb0, vec::BitVec};
use itertools::Itertools;

use crate::primer::Base;

pub trait Decoder {
    fn decode(&self, bases: &Vec<Base>) -> BitVec<u8, Msb0>;
}

pub struct ChurchDecoder {}
impl Decoder for ChurchDecoder {
    fn decode(&self, bases: &Vec<Base>) -> BitVec<u8, Msb0> {
        BitVec::from_iter(bases.iter().map(|base| match base {
            Base::A => false,
            Base::T => false,
            Base::G => true,
            Base::C => true,
        }))
    }
}

pub struct QuaternaryDecoder {}
impl Decoder for QuaternaryDecoder {
    fn decode(&self, bases: &Vec<Base>) -> BitVec<u8, Msb0> {
        BitVec::from_iter(bases.iter().flat_map(|b| match b {
            Base::A => [true, true],
            Base::T => [true, false],
            Base::G => [false, true],
            Base::C => [false, false],
        }))
    }
}

pub struct RotationDecoder {}
impl Decoder for RotationDecoder {
    fn decode(&self, bases: &Vec<Base>) -> BitVec<u8, Msb0> {
        let rotation_mapping = HashMap::from([
            (Base::A, [Base::T, Base::G, Base::C]),
            (Base::T, [Base::A, Base::G, Base::C]),
            (Base::C, [Base::A, Base::G, Base::T]),
            (Base::G, [Base::T, Base::A, Base::C]),
        ]);

        let mut trits = Vec::new();

        for [prev, curr] in bases.array_windows() {
            trits.push(
                rotation_mapping[prev]
                    .iter()
                    .position(|b| b == curr)
                    .unwrap(),
            )
        }

        let chunks = trits.iter().chunks(6);
        let bytes = chunks.into_iter().map(|chunk| {
            chunk
                .zip((0..6).rev())
                .map(|(trit, i)| 3_u8.pow(i) * (*trit as u8))
                .sum::<u8>()
        });

        BitVec::from_iter(bytes)
    }
}
