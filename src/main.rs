extern crate ma_brothers; 
extern crate argparse;

use ma_brothers::generations::Generations; 
use argparse::{ArgumentParser, Store};

fn main() {
    let mut n_gens = 0;
    { 
        let mut ap = ArgumentParser::new();
        ap.set_description("Deals with MA brothers MRCA estimates flawlessy. Maybe.");
        ap.refer(& mut n_gens).add_option(&["-g", "--gens"], Store, "Number of generations to simulate").required();
        ap.parse_args_or_exit();
    }   

    println!("Allocating your BitVec for {} generations", n_gens);
    let mut sim = Generations::new(n_gens);
    println!("Done: \n {:?}", sim);
    sim.generate_cell(0);
    println!("Step 1: \n {:?}", sim);
}