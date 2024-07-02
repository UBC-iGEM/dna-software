use rand::{Rng, distributions::{Distribution, Standard}};
use crate::primer::Base;


pub trait RandError {
    fn modify_base(&self, input_base: &Base) -> Base; 
    fn full_chaos(&self, initial_seq: &Vec<Base>, edit_prob: f64, mod_prob: f64) -> Vec<Base>;
}


pub struct Chaos {
}


impl RandError for Chaos {
    // Takes a base vector and probability as input, outputs a base vector with modified bases
    fn modify_base(&self, input_base: &Base) -> Base {
	let mut rng = rand::thread_rng();
	loop {
	    let new_base = rng.gen::<Base>();
	    if new_base != *input_base {
		return new_base;
	    }
	}
    }


    fn full_chaos(&self, initial_seq: &Vec<Base>, edit_prob: f64, del_prob: f64) -> Vec<Base> {
	let mut rng = rand::thread_rng();
	initial_seq
	    .iter()
	    .filter_map(|base| {
		if rng.gen_bool(edit_prob) {
		    if rng.gen_bool(del_prob) {
			None
		    }
		    else {
			Some(self.modify_base(base))
		    }
		}
		else {
		    Some(*base)
		}
	    })
	    .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primer::Base;

    #[test]
    fn test_mutate() {
        let chaos = Chaos {};
	let mut rng = rand::thread_rng();

        let initial_sequence: Vec<Base> = (0..10)
	    .map(|_| rng.gen::<Base>())
	    .collect();
	println!("Initial sequence: ");
	for base in &initial_sequence {
	    print!("{}", base);
	}
	println!();
	
        let modified_sequence = chaos.full_chaos(&initial_sequence, 0.3, 0.5);
	println!("Modified sequence: ");
	for base in &modified_sequence {
	    print!("{}", base);
	}
	println!();
    }
}
