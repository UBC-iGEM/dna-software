use crate::primer::Base;
use rand::Rng;

pub trait RandError {
    fn modify_base(&self, input_base: &Base) -> Base;
    fn full_chaos(&self, initial_seq: &Vec<Base>, edit_prob: f64, mod_prob: f64) -> Vec<Base>;
}

pub struct Chaos {}

impl RandError for Chaos {
    // Takes a base as input, outputs a different base.
    fn modify_base(&self, input_base: &Base) -> Base {
        let mut rng = rand::thread_rng();
        loop {
            let new_base = rng.gen::<Base>();
            if new_base != *input_base {
                return new_base;
            }
        }
    }

    // Takes a vector of bases, an edit probability, and a deletion prob as input.
    // Outputs a vector of bases that may be modified.
    fn full_chaos(&self, initial_seq: &Vec<Base>, edit_prob: f64, del_prob: f64) -> Vec<Base> {
        let mut rng = rand::thread_rng();
        initial_seq
            .iter()
            .filter_map(|base| {
                // If the edit probability boolean passes...
                if rng.gen_bool(edit_prob) {
                    // Delete if deletion prob boolean passes.
                    if rng.gen_bool(del_prob) {
                        None
                    }
                    // Modify if deletion prob boolean fails.
                    else {
                        Some(self.modify_base(base))
                    }
                }
                // If the edit prob boolean fails, return the original base.
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

    // Generate a random vector of bases and execute full_chaos on it.
    // Output is printed.
    #[test]
    fn test_mutate() {
        let chaos = Chaos {};
        let mut rng = rand::thread_rng();

        let initial_sequence: Vec<Base> = (0..10).map(|_| rng.gen::<Base>()).collect();
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

    // Use full_chaos on a random vector of bases with edit_prob set to 0.
    // Output should be identical to input.
    #[test]
    fn chaos_sanity_no_edit() {
        let chaos = Chaos {};
        let mut rng = rand::thread_rng();

        let initial_sequence: Vec<Base> = (0..10).map(|_| rng.gen::<Base>()).collect();

        let modified_sequence = chaos.full_chaos(&initial_sequence, 0.0, 0.5);

        assert_eq!(initial_sequence, modified_sequence);
    }

    // Use full_chaos with edit_prob and del_prob set to 0.
    // All bases should be deleted.
    #[test]
    fn chaos_sanity_del_all() {
        let chaos = Chaos {};
        let mut rng = rand::thread_rng();

        let initial_sequence: Vec<Base> = (0..10).map(|_| rng.gen::<Base>()).collect();

        let modified_sequence = chaos.full_chaos(&initial_sequence, 1.0, 1.0);

        assert_eq!(modified_sequence.len(), 0);
    }

    // Use full_chaos with del_prob set to 0.
    // Output length should be same as input.
    #[test]
    fn chaos_sanity_only_mut() {
        let chaos = Chaos {};
        let mut rng = rand::thread_rng();

        let initial_sequence: Vec<Base> = (0..10).map(|_| rng.gen::<Base>()).collect();

        let modified_sequence = chaos.full_chaos(&initial_sequence, 0.5, 0.0);

        assert_eq!(modified_sequence.len(), initial_sequence.len());
        assert_eq!(modified_sequence.len(), 10);
    }
}
