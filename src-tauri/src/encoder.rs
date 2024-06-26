use std::{collections::HashMap, iter, u128};

use bitvec::{order::Msb0, prelude::BitVec};

use crate::primer::Base;

pub trait Encoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base>;
}

pub struct DummyEncoder {}
impl Encoder for DummyEncoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base> {
        match input {
            _ => {}
        }
        todo!()
    }
}

pub struct RotationEncoder {}
impl Encoder for RotationEncoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base> {
        let num: u128 = input
            .iter()
            .rev()
            .map(|bit| (*bit as u128))
            .chain(iter::once(1))
            .zip(0..)
            .map(|(bit, i)| 2_u128.pow(i) * bit)
            .sum();

        let trits = num_to_digits(num, 3);
        let rotation_mapping = HashMap::from([
            (Base::A, [Base::T, Base::G, Base::C]),
            (Base::T, [Base::A, Base::G, Base::C]),
            (Base::C, [Base::A, Base::G, Base::T]),
            (Base::G, [Base::T, Base::A, Base::C]),
        ]);
        trits.iter().fold(vec![Base::A], |mut bases: Vec<Base>, t| {
            bases.push(rotation_mapping[bases.last().unwrap()][*t as usize]);
            bases
        })
    }
}

fn num_to_digits(mut num: u128, num_base: u128) -> Vec<u8> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % num_base) as u8);
        num /= num_base;
    }

    digits.reverse();
    digits
}

pub struct HEDGESEncoder {}
impl Encoder for HEDGESEncoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::{Decoder, RotationDecoder};
    use crate::encoder::{Encoder, RotationEncoder};
    use bitvec::prelude::BitVec;

    #[quickcheck]
    fn rotation_encode_decode(bytes: Vec<u8>) -> bool {
        let encoder = RotationEncoder {};
        let decoder = RotationDecoder {};
        let bits = BitVec::from_vec(bytes);
        bits == decoder.decode(&encoder.encode(&bits))
    }
}
