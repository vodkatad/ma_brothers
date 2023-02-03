//use std::io;
//use std::io::prelude::*;
//use std::iter::Iterator;
use bit_vec::BitVec;
use rand::Rng;
use std::collections::HashSet;
//use rand::distributions::{Bernoulli, Distribution};

// const NN: f64 = 0f64;
const BASE: usize = 2;
//const UNDEAD: f32 = 0.9; // d / b
//const DEAD: f32 = 0.9; // d / b

/// Struct used to store a binary vector representing the tree with cells generations.
#[derive(Debug)]
pub struct Generations {
    gens: BitVec,
    ngen: usize, // the generation that we have reached
    p_death: f32,
    rng: rand::rngs::ThreadRng,
}

/// Computes the index of the first leaf (leftmost) in generation gen+1
fn from_gen_to_nodes(gen: usize) -> usize {
    // A complete binary tree with h gen has 2^(gen+1) - 1 nodes
    return BASE.pow(gen as u32 +1)-1;
}

impl Generations {
    /// Creates a new generation simulation structure, pre-allocating all the needed generations.
    // Since we generate the tree depth first tracking ngen is not that useful - we are calculating base indexes to get
    // children indexes right now.
    pub fn new(n: usize, p: f32) -> Generations {
        Generations { gens: BitVec::from_elem(from_gen_to_nodes(n), false), ngen: 0, rng: rand::thread_rng(), p_death: p}
    }

    pub fn generate_cell(&mut self, i: usize) {
        if i >= self.gens.len() { // but also check on generation management! (? TODO)
            //println!("End of tree!");
            return; // TODO study again how to manage results in rust
        }
        let upper_index = from_gen_to_nodes(self.ngen); // better to store this probably FIXME
        if i >= upper_index {  
            self.ngen += 1;
        }
        self.gens.set(i, true);
        self.cell_life_cycle(i);
    }

    pub fn cell_life_cycle(&mut self, i: usize) {
        if !self.will_die(i) {
            //println!("I am alive {:?}", i);
            //println!("{:?}", self);
            //println!("{:?}", self.ngen);
            //println!("{:?}", self.gens.len());
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
        } else {
            //println!("I am dead {:?}", i);
            self.gens.set(i, false);    
        }
    }

    pub fn will_die(&mut self, _i: usize) -> bool {
        // We keep the index _i here just in case in the future we want to simulate different classes of cells.
        // random distributions https://docs.rs/rand_distr/0.4.3/rand_distr/index.html
        return self.rng.gen_range(0.0..1.0) < self.p_death
    }

    /// Returns a vector with indexes of alive cells in the last generation of our tree.
    pub fn alive_last_gen(& self) ->  Vec::<usize> {
        // can store alive indexes as we go on to be more efficient? FIXME
        if self.ngen == 0 {
            return vec![];
        }
        let lower_index = from_gen_to_nodes(self.ngen-1);
        let upper_index = from_gen_to_nodes(self.ngen); // no -1 because .. is an right open interval
        let mut alive_indexes = Vec::<usize>::with_capacity(BASE.pow(self.ngen as u32));
        for i in lower_index .. upper_index {
            if self.gens.get(i).unwrap() {
                alive_indexes.push(i);
            }
        }
        //println!("alive\t{}", alive_indexes.len());
        return alive_indexes;
    }

    /// Return n random alive cells from last generation, return their indexes.
    pub fn select_rand_last_gen(& mut self, n: usize, alive_cells: Vec::<usize>) -> HashSet::<usize> {

        //let alive_cells = self.alive_last_gen();
        let n_alive = alive_cells.len();
        let mut n_res = n;
        if n > n_alive {
            n_res = n_alive;
        }
        // We use a set to avoid picking more than once the same cell.
        let mut chosen_alive = HashSet::<usize>::with_capacity(n_res);
        while chosen_alive.len() < n_res {
            let rand_fraction : f64 = self.rng.gen_range(0.0..1.0);
            let index = (rand_fraction * n_alive as f64) as usize;
            chosen_alive.insert(alive_cells[index]);
        }
        return chosen_alive;
    }

    pub fn get_father(& self, i: usize, n_gen_mine: usize, n_gen_upper: usize) -> usize {
        let father_i : f32;
        if i > 0 {
            father_i = ((i as f32) / (BASE as f32)) + (n_gen_upper as f32) - ((n_gen_mine as f32) / (BASE as f32));
        } else {
            return 0;
        }
        if i % 2 == 0 { // right children
            return (father_i - 0.5) as usize;
        } else { // left children
            return father_i as usize;
        }
    }

    /// Function that given the index of a cell in the last generation returns the list of its ancestors.
    /// Unchecked precondition: given indexes come from alive cells.
    /// Would it be less awkward if recursive?
    pub fn find_ancestors(& self, mut i: usize) -> Vec::<(usize, usize)> {
        let mut ancestors_indexes = Vec::<(usize, usize)>::with_capacity(self.ngen);
        let mut lower_index = from_gen_to_nodes(self.ngen-1);
        let mut upper_index = from_gen_to_nodes(self.ngen); 
        let mut reached_gen = self.ngen-1;
        if self.ngen != 0 {
            let mut father_index = self.get_father(i, upper_index, lower_index);
            while father_index != 0 {
                ancestors_indexes.push((father_index, reached_gen));
                reached_gen = reached_gen - 1;
                upper_index = lower_index;
                lower_index = from_gen_to_nodes(reached_gen);
                i = father_index;
                father_index = self.get_father(i, upper_index, lower_index);
            }
            ancestors_indexes.push((father_index, reached_gen));
        } 
        return ancestors_indexes
    }

    // Function that given indexes of two cells returns the index/generation of their most recent common ancestor.
    // Unchecked precondition: given indexes come from alive cells.
    //pub fn find_mrca(& self, i: usize, j: usize) {

    //}

    // Function that given indexes of three cells returns the (index, generation) of their most recent common ancestor.
    // Unchecked precondition: given indexes come from alive cells.
    pub fn find_mrca_three(& self, i0: usize, i1: usize, i2: usize) -> (usize, usize) {
        let anc0 = self.find_ancestors(i0);
        let anc1 = self.find_ancestors(i1);
        let anc2 = self.find_ancestors(i2);
        let mut u = anc0.len()-1; // u is the last index, all ancestors are supposed to have the same len
        // this is true if we only pick cells from last generation though in reality (in the sim we are forcing this).
        //println!("anc0: \n {:?}", anc0);
        //println!("anc1: \n {:?}", anc1);
        //println!("anc2: \n {:?}", anc2);
        while anc0[u].0 == anc1[u].0 && anc1[u].0 == anc2[u].0 {
            u = u - 1
        }
        return anc0[u+1];
    }

    pub fn find_mrca_two(& self, i0: usize, i1: usize) -> (usize, usize) {
        let anc0 = self.find_ancestors(i0);
        let anc1 = self.find_ancestors(i1);
        let mut u = anc0.len()-1;
        //println!("anc0: \n {:?}", anc0);
        //println!("anc1: \n {:?}", anc1);
        //println!("anc2: \n {:?}", anc2);
        while anc0[u].0 == anc1[u].0 {
            u = u - 1
        }
        return anc0[u+1];
    }


    // The basic idea is that since we will be mainly working with set of three selected cells instead of going up in sync on the tree and stop ASAP
    // we get the list of all their ancestors and find the largest common one between the three vectors.
    // Instead of implementing the general approach we try to be more efficient knowing we will always work with three (2 or 1 will be excluded corner cases).

    // Do we need to implement the real experiment?
    // Sim would need to starts from 1 cell, then is curbed to random 100 and goes on.
}
