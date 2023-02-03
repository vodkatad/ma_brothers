extern crate ma_brothers; 
extern crate argparse;

use ma_brothers::generations::Generations; 
use argparse::{ArgumentParser, Store};

fn main() {
    let mut n_gens = 0;
    let mut n_runs = 0;
    let mut p_death: f32 = 0.0;
    let mut mode = 0;

    { 
        let mut ap = ArgumentParser::new();
        ap.set_description("Deals with MA brothers MRCA estimates flawlessy. Maybe.");
        ap.refer(& mut n_gens).add_option(&["-g", "--gens"], Store, "Number of generations to simulate").required();
        ap.refer(& mut n_runs).add_option(&["-n", "--nruns"], Store, "Number of runs to perform").required();
        ap.refer(& mut p_death).add_option(&["-d", "--pdeath"], Store, "probability of death when trying to divide, d/b from MA").required();
        ap.refer(& mut mode).add_option(&["-m", "--mode"], Store, "1 single tree 2 full MA").required();
        ap.parse_args_or_exit();
    }   
    if mode == 1 {
        //println!("Allocating your BitVec for {} generations", n_gens);
        let fixed_n = 3;
        for _i in 0..n_runs {
            let mut sim = Generations::new(n_gens, p_death);
            sim.generate_cell(0);  
            let alive = sim.alive_last_gen();
            println!("tree\t{}\t{}", _i, alive.len());
            let random_last = sim.select_rand_last_gen(fixed_n, alive);
            //println!("Done: \n {:?}", sim);
            //println!("Random: \n {:?}", random_last);
            let mut chosen = Vec::<usize>::with_capacity(fixed_n);
            for i in random_last.iter() {
                chosen.push(*i);
            }
            //println!("Random2: \n {:?}", chosen);
            if chosen.len() >= fixed_n {
                //let mrca = sim.find_mrca_two(chosen[0], chosen[1]);
                let mrca = sim.find_mrca_three(chosen[0], chosen[1], chosen[2]);
                println!("anc\t{}\t{}", mrca.0, mrca.1);
            } else {
                println!("anc\t{}\t{}", -1, -1);
            }
        }
    } else if mode == 2 {
        const N: usize = 3;
        const BOTTLEN: usize = 100;
        // Right now we ignore n_runs and always do 2 rounds of 100 before selecting last 3
        let mut rounds = vec![vec![vec![(42,42); BOTTLEN]; BOTTLEN]; N];
        for i in 0..rounds.len() {
            if i == 0 {
                let mut sim = Generations::new(n_gens, p_death);
                sim.generate_cell(0);  
                let alive = sim.alive_last_gen();
                if alive.len() < BOTTLEN {
                    println!("Failed cloning {}", alive.len());
                } else {
                    let random_last = sim.select_rand_last_gen(BOTTLEN, alive);
                    let mut chosen = Vec::<usize>::with_capacity(BOTTLEN);
                    for i in random_last.iter() {
                        chosen.push(*i);
                    }
                    for c1 in 0..BOTTLEN {
                        for c2 in (c1+1)..BOTTLEN {
                            println!("anc\t{}\t{}", c1, c2);
                            rounds[i][c1][c2] = sim.find_mrca_two(chosen[c1], chosen[c2]);    
                            // At the first level we store a single number rather than a tuple
                        }
                    }
                }
            } else {
                // TODO
            }
        }
        for c1 in 0..BOTTLEN {
            for c2 in (c1+1)..BOTTLEN {
                println!("{}\t{}\t{}\t{}", c1, c2, rounds[0][c1][c2].0, rounds[0][c1][c2].1);
            }
        }
    }
}