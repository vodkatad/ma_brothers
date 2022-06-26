//use std::io;
//use std::io::prelude::*;
//use std::iter::Iterator;
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
    /// Creates a new generation simulation structure, pre-allocating all the needed generations.\
    // Since we generate the tree depth first tracking ngen is not that useful - we are calculating base indexes fo get
    // children indexes right now.
    pub fn new(n: usize) -> Generations {
        Generations { gens: BitVec::from_elem(from_gen_to_nodes(n), false), ngen: 0, rng: rand::thread_rng() }
    }

    pub fn generate_cell(&mut self, i: usize) {
        if i >= self.gens.len() { // but also check on generation management!
            println!("FOAD"); // will error checking slow us down?
            return; // TODO study again how to manage results
        }
        let upper_index = from_gen_to_nodes(self.ngen); // better to store this probably FIXME
        if i >= upper_index {  
            self.ngen += 1;
        }
        self.gens.set(i, true);
        self.cell_life_cycle(i);
    }

    pub fn cell_life_cycle(&mut self, i: usize) {
        /* 
        if !self.gens.get(i).unwrap() {
            println!("FOAD"); // will error checking slow us down?
            return; // TODO study again how to manage results
            
        } else {
        */
        if !self.will_die(i) {
            println!("I am alive {:?}", i);
            println!("{:?}", self);
            println!("{:?}", self.ngen);
            println!("{:?}", self.gens.len());
            let mut index = 0;
            let my_gen = ((i as f64).log2().floor()) as usize;
            if my_gen != 0 {
                index = from_gen_to_nodes(my_gen-1);
            }
            let upper_index = from_gen_to_nodes(my_gen);
            let left_child = upper_index + (i - index)*2;
            let right_child = upper_index + (i - index)*2 + 1;
            self.generate_cell(left_child); 
            self.generate_cell(right_child); 
            //self.cell_life_cycle(left_child); // call here and not in generate_cell to do BF
            //self.cell_life_cycle(right_child);
        } else {
            println!("I am dead {:?}", i);
            self.gens.set(i, false);    
        }
    }

    pub fn will_die(&mut self, _i: usize) -> bool {
        // We keep the index _i here just in case in the future we want to simulate different classes of cells.
        // random distributions https://docs.rs/rand_distr/0.4.3/rand_distr/index.html
        return self.rng.gen_range(0.0..1.0) > UNDEAD
    }

    pub fn select_rand_last_gen(& self) -> usize {
        let lower_index = from_gen_to_nodes(self.ngen-1);
        let upper_index = from_gen_to_nodes(self.ngen)-1;
        let mut alive_indexes = Vec::<usize>::with_capacity(BASE.pow(self.ngen as u32)); // TODO CHECK
        for i in lower_index .. upper_index {
            if self.gens.get(i).unwrap() {
                alive_indexes.push(i);
            }
        }
        println!("Bottom alive {:?}", alive_indexes.len());
        return 0;
    }
}