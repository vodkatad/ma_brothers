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
    /*sim.cell_life_cycle(0);
    println!("Step 1: \n {:?}", sim);
    sim.generate_cell(2);
    sim.cell_life_cycle(2);
    println!("Step 2: \n {:?}", sim);
    sim.generate_cell(10);
    sim.cell_life_cycle(10);
    println!("Step 3: \n {:?}", sim);

    for _i in 0 .. 10 {
        sim.cell_life_cycle(0);
    }*/
}