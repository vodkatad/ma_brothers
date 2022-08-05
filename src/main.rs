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
    let fixed_n = 3;
    let random_last = sim.select_rand_last_gen(fixed_n);
    println!("Random: \n {:?}", random_last);
    let mut chosen = Vec::<usize>::with_capacity(fixed_n);
    for i in random_last.iter() {
        chosen.push(*i);
    }
    println!("MRCA: \n {:?}", sim.find_mrca_three(chosen[0], chosen[1], chosen[2]));

    //sim.cell_life_cycle(0);
    /*println!("Step 1: \n {:?}", sim);
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