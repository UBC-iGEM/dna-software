use rand::Rng;
use std::{io, iter};


pub trait ChaosDNA {
    fn modify_base(&self, input: &Vec<Base>, prob: f64) -> Vec<Base>; 
    fn insert_base(&self, input: &Vec<Base>, prob: f64) -> Vec<Base>;
    fn apply_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<Base>;
    fn diff_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<diff::Result<Base>>;
}

pub struct ErrorDNA {
}

impl ChaosDNA for ErrorDNA {
    //Takes a base vector and probability as input, outputs a base vector with modified bases
    fn modify_base(&self, input: &Vec<Base>, prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	input
	    .iter()
	    .map(|base| {
		//RNG boolean; if true, keeps running until a different base from the original is selected
            if rng.gen_bool(prob) { 
                loop {
                    let new_base = rng.gen::<Base>();
                    if new_base != *base {
                        return new_base; 
                    }
                }
		//If RNG boolean is false, returns original (no change)
            } else {
                *base 
            }
        })
	    .collect()
    }

    fn insert_base(&self, input: &Vec<Base>, prob: f64) -> Vec<Base> {
	todo!()
    }

    fn apply_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<Base> {
	todo!()
    }

    fn diff_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<diff::Result<Base>> {
        todo!()
    }
}
