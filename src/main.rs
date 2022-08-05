extern crate ma_brothers; 
extern crate argparse;

use ma_brothers::generations::Generations; 
use argparse::{ArgumentParser, Store};

fn main() {
    let mut n_gens = 0;
    let mut n_runs = 0;
    { 
        let mut ap = ArgumentParser::new();
        ap.set_description("Deals with MA brothers MRCA estimates flawlessy. Maybe.");
        ap.refer(& mut n_gens).add_option(&["-g", "--gens"], Store, "Number of generations to simulate").required();
        ap.refer(& mut n_runs).add_option(&["-n", "--nruns"], Store, "Number of runs to perform").required();
        ap.parse_args_or_exit();
    }   

    //println!("Allocating your BitVec for {} generations", n_gens);
    let fixed_n = 3;
    for _i in 0..n_runs {
        let mut sim = Generations::new(n_gens);
        sim.generate_cell(0);  
        let random_last = sim.select_rand_last_gen(fixed_n);
        //println!("Done: \n {:?}", sim);
        //println!("Random: \n {:?}", random_last);
        let mut chosen = Vec::<usize>::with_capacity(fixed_n);
        for i in random_last.iter() {
            chosen.push(*i);
        }
        if chosen.len() >= fixed_n {
            let mrca = sim.find_mrca_three(chosen[0], chosen[1], chosen[2]);
            println!("{}\t{}", mrca.0, mrca.1);
        } else {
            println!("{}\t{}", -1, -1);
        }
    }
}