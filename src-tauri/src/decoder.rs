use std::collections::HashMap;

use bitvec::{order::Msb0, vec::BitVec};

use crate::primer::Base;

pub trait Decoder {
    fn decode(&self, bases: &Vec<Base>) -> BitVec<u8, Msb0>;
}

pub struct DummyDecoder {}
impl Decoder for DummyDecoder {
    fn decode(&self, bases: &Vec<Base>) -> BitVec<u8, Msb0> {
        todo!()
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

        let num: u128 = trits
            .iter()
            .rev()
            .zip(0..)
            .map(|(trit, i)| 3_u128.pow(i) * (*trit as u128))
            .sum();

        let bytes = num.to_be_bytes();
        let bits: BitVec<u8, Msb0> = BitVec::from_slice(&bytes);
        bits.iter().skip(bits.leading_zeros()).skip(1).collect()
    }
}
