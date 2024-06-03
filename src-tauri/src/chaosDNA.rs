use rand::{Rng, distributions::{Distribution, Standard}};
// use diff::Diff;


pub trait ChaosDNA {
    fn modify_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base>; 
    fn insert_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base>;
    fn full_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, ins_prob: f64) -> Vec<Base>;
//    fn diff_chaos(&self, input: &Vec<Base>, ins_prob: f64, mod_prob: f64) -> Vec<diff::Result<Base>>;
}


pub struct ErrorDNA {
}


impl ChaosDNA for ErrorDNA {
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

    fn insert_base(&self, initial_seq: &Vec<Base>, prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	let mut inserted_seq = Vec::new();

	// Insert random bases at start while rng boolean is true
	while rng.gen_bool(prob) {
	    inserted_seq.push(rng.gen::<Base>());
	}
	// Insert random base(s) after current
	for base in initial_seq {
	    inserted_seq.push(*base);
	    while rng.gen_bool(prob) {
		inserted_seq.push(rng.gen::<Base>());
	    }
	}
	inserted_seq
    }

    fn full_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, ins_prob: f64) -> Vec<Base> {
	let mod_seq = self.modify_base(initial_seq, mod_prob);
	self.insert_base(&mod_seq, ins_prob)
    }

//    fn diff_chaos(&self, initial_seq: &Vec<Base>, mod_prob: f64, ins_prob: f64) -> Vec<diff::Result<Base>> {
//	todo()!
//    }
}


// Here is some example code that I used to test:


// fn main() {
//     let chaos = ErrorDNA{};
//     let initial_sequence = vec![Base::G, Base::T, Base::A, Base::C, Base::C, Base::G, Base::A, Base::T, Base::T, Base::G];
//     let modified_sequence = chaos.full_chaos(&initial_sequence, 0.2, 0.2);
//     println!("{:?}", modified_sequence);
// }
