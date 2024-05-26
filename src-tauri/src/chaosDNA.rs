use rand::Rng;
use std::{io, iter};
use diff::Diff;


pub trait ChaosDNA {
    fn modify_base(&self, input: &Vec<Base>, prob: f64) -> Vec<Base>; 
    fn insert_base(&self, input: &Vec<Base>, prob: f64) -> Vec<Base>;
    fn apply_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<Base>;
    fn diff_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<diff::Result<Base>>;
}

pub struct ErrorDNA {
}

impl ChaosDNA for ErrorDNA {
    // Takes a base vector and probability as input, outputs a base vector with modified bases
    fn modify_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	input
	    .iter()
	    .map(|base| {
		// RNG boolean; if true, keeps running until a different base from the original is selected
		if rng.gen_bool(prob) { 
                    loop {
			let new_base = rng.gen::<Base>();
			if new_base != *base {
                            return new_base; 
			}
                    }
		    // If RNG boolean is false, returns original (no change)
		} else {
                    *base 
		}
            })
	    .collect()
    }

    fn insert_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	let mut inserted_seq = Vec::new();

	// Insert random bases at start while rng boolean is true
	while rng.gen_bool(prob) {
	    inserted_seq.push(rng.gen::<Base>());
	}
	// Insert random base(s) after current
	for base in input {
	    inserted_seq.push(*base);
	    while rng.gen_bool(prob) {
		inserted_seq.push(rng.gen::<Base>());
	    }
	}
    }

    fn full_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, ins_prob: f64) -> Vec<Base> {
	let mod_seq = self.modify_base(initial_seq, mod_prob);
	self.insert_base(mod_seq, ins_prob)
    }

    fn diff_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, ins_prob: f64) -> Vec<diff::Result<Base>> {
	todo()!
    }
}
