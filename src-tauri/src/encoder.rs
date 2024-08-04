use std::{collections::HashMap, io::Read, iter, u128};

use bitvec::{order::Msb0, prelude::BitVec};

use crate::primer::Base;

pub trait Encoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base>;
}

pub struct ChurchEncoder {}
impl Encoder for ChurchEncoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base> {
        let mut prev_base = Base::T;
        input
            .iter()
            .by_vals()
            .map(|bit| {
                prev_base = alternate_base(prev_base, bit);
                prev_base
            })
            .collect()
    }
}

fn alternate_base(prev_base: Base, bit: bool) -> Base {
    let church_mapping = vec![vec![Base::A, Base::T], vec![Base::C, Base::G]];

    match bit {
        true => {
            if prev_base.eq(&church_mapping[1][0]) {
                church_mapping[1][1]
            } else {
                church_mapping[1][0]
            }
        }
        false => {
            if prev_base.eq(&church_mapping[0][0]) {
                church_mapping[0][1]
            } else {
                church_mapping[0][0]
            }
        }
    }
}

pub struct QuaternaryEncoder {}
impl Encoder for QuaternaryEncoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base> {
        input
            .iter()
            .by_vals()
            .array_chunks()
            .map(|[first, second]| match [first, second] {
                [true, true] => Base::A,
                [true, false] => Base::T,
                [false, true] => Base::G,
                [false, false] => Base::C,
            })
            .collect()
    }
}

pub struct RotationEncoder {}
impl Encoder for RotationEncoder {
    fn encode(&self, input: &BitVec<u8, Msb0>) -> Vec<Base> {
        let trits: Vec<u8> = input
            .clone()
            .bytes()
            .flat_map(|b| num_to_digits(b.unwrap() as usize, 3, 6))
            .collect();
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

fn num_to_digits(mut num: usize, num_base: usize, num_length: usize) -> Vec<u8> {
    let mut digits = Vec::new();

    for _ in 0..num_length {
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
    use crate::decoder::{ChurchDecoder, Decoder, QuaternaryDecoder, RotationDecoder};
    use crate::encoder::{ChurchEncoder, Encoder, QuaternaryEncoder, RotationEncoder};
    use bitvec::prelude::BitVec;

    #[quickcheck]
    fn rotation_encode_decode(bytes: Vec<u8>) -> bool {
        let encoder = RotationEncoder {};
        let decoder = RotationDecoder {};
        let bits = BitVec::from_vec(bytes);
        bits == decoder.decode(&encoder.encode(&bits))
    }
    #[quickcheck]
    fn church_encode_decode(bytes: Vec<u8>) -> bool {
        let encoder = ChurchEncoder {};
        let decoder = ChurchDecoder {};
        let bits = BitVec::from_vec(bytes);
        bits == decoder.decode(&encoder.encode(&bits))
    }

    #[quickcheck]
    fn quaternary_encode_decode(bytes: Vec<u8>) -> bool {
        if bytes.len() % 2 == 1 {
            true
        } else {
            let encoder = QuaternaryEncoder {};
            let decoder = QuaternaryDecoder {};
            let bits = BitVec::from_vec(bytes);
            bits == decoder.decode(&encoder.encode(&bits))
        }
    }
}
