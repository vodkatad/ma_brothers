use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use bit_vec::BitVec;
use rand::Rng;

// const NN: f64 = 0f64;
const BASE: usize = 2;
const UNDEAD: f32 = 0.8;

/// Struct used to store a binary vector representing the tree with cells generations.
#[derive(Debug)]
pub struct Generations {
    gens: BitVec,
    ngen: usize, // the generation that we have reached
    rng: rand::rngs::ThreadRng,
}

fn from_gen_to_nodes(gen: usize) -> usize {
    //let mut res: usize = 0;
    //for i in 0..gen {
    //    res = res + (BASE^i);
    //}
    // A complete binary tree with h gen has 2^(gen+1) - 1 nodes
    return BASE.pow(gen as u32 +1)-1;
}

impl Generations {
    /// Creates a new generation simulation structure, pre-allocating all the needed generations.
    pub fn new(n: usize) -> Generations {
        Generations { gens: BitVec::from_elem(from_gen_to_nodes(n), false), ngen: 0, rng: rand::thread_rng() }
    }

    pub fn generate_cell(&mut self, i: usize) {
        if i > self.gens.len() { // but also check on generation management!
            println!("FOAD"); // will error checking slow us down?
            return; // TODO study again how to manage results
        }
        let upper_index = from_gen_to_nodes(self.ngen); // better to store this probably FIXME
        if i >= upper_index {  
            self.ngen += 1;
        }
        self.gens.set(i, true)
    }

    pub fn cell_life_cycle(&mut self, i: usize) {
        if !self.gens.get(i).unwrap() {
            println!("FOAD"); // will error checking slow us down?
            return; // TODO study again how to manage results
            
        } else {
            if !self.will_die(i) {
                println!("I am alive {:?}", i);
            } else {
                println!("I am dead {:?}", i);
                self.gens.set(i, false);    
            }
            return;
        }
    }

    pub fn will_die(&mut self, i: usize) -> bool {
        // random distributions https://docs.rs/rand_distr/0.4.3/rand_distr/index.html
        return self.rng.gen_range(0.0..1.0) > UNDEAD
    }
}