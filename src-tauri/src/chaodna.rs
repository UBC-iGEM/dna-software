use rand::{Rng, distributions::{Distribution, Standard}};
use crate::primer::Base;
// use diff::Diff;


pub trait RandError {
    fn modify_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base>; 
    fn delete_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base>;
    fn full_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, del_prob: f64) -> Vec<Base>;
//    fn diff_chaos(&self, input: &Vec<Base>, del_prob: f64, mod_prob: f64) -> Vec<diff::Result<Base>>;
}


pub struct Chaos {
}


impl RandError for Chaos {
    // Takes a base vector and probability as input, outputs a base vector with modified bases
    fn modify_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	initial_seq
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

    fn delete_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	initial_seq
	    .iter()
	    .filter_map(|base| {
		// RNG boolean; if true, returns nothing
		if rng.gen_bool(prob) {
                    None
		}
		    // If RNG boolean is false, returns original (no change)
		else {
                    Some(*base)
		}
            })
	    .collect()
    }

    fn full_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, del_prob: f64) -> Vec<Base> {
	let mod_seq = self.modify_base(initial_seq, mod_prob);
	self.delete_base(&mod_seq, del_prob)
    }

//    fn diff_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, del_prob: f64) -> Vec<diff::Result<Base>> {
//	todo()!
//    }
}


// Here is some example code that I used to test:

// fn main() {
//     let chaos = Chaos{};
//     let initial_sequence = vec![Base::G, Base::T, Base::A, Base::C, Base::C, Base::G, Base::A, Base::T, Base::T, Base::G];
//     let modified_sequence = chaos.full_chaos(&initial_sequence, 0.2, 0.2);
//     println!("{:?}", modified_sequence);
// }
