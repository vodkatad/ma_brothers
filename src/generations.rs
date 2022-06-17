use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use bit_vec::BitVec;

// const NN: f64 = 0f64;
const BASE: usize = 2;

/// Struct used to store a binary vector representing the tree with cells generations.
#[derive(Debug)]
pub struct Generations {
    gens: BitVec,
    ngen: usize, // the generation that we have reached
}

fn from_gen_to_nodes(gen: usize) -> usize {
    let mut res: usize = 0;
    for i in 0..gen {
        res = res + (BASE^i);
    }
    return res;
}

impl Generations {
    /// Creates a new generation simulation structure, pre-allocating all the needed generations.
    pub fn new(n: usize) -> Generations {
        Generations { gens: BitVec::with_capacity(from_gen_to_nodes(n)), ngen: 1 }  // TODO can we fill with 0?  VecBool?
    }

    pub fn generate_cell(&mut self, i: usize) {
        println!("puppa {:?} {:?}", self.gens.capacity(), i);
        if i > self.gens.capacity() { // but also check on generation management!
            println!("FOAD"); // will error checking slow us down?
            return; // TODO study again how to manage results
        }
        if i < self.ngen {  // TODO implement check if we are adding a new gen
            self.gens.set(i, true)
            // TODO self.ngen management
        } else {
            // new gen management, here?
        }
    }

    pub fn cell_life_cycle(&mut self, i: usize) {
        if !self.gens.get(i).unwrap() {
            println!("FOAD"); // will error checking slow us down?
            return; // TODO study again how to manage results
            
        } else {
            // if ! self.will_die(i)
                // generate sons
            // else
                // self.gens[i] = 0;    
            println!("YAY!");
            return;
        }
    }
}