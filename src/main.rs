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
                let mut n;
                let mut alive:  Vec::<usize>;
                let mut sim: Generations;
                loop { 
                    sim = Generations::new(n_gens, p_death);
                    sim.generate_cell(0);  
                    alive = sim.alive_last_gen();
                    n = alive.len();
                    if n >= BOTTLEN {
                        break
                    }
                }
                let random_last = sim.select_rand_last_gen(BOTTLEN, alive);
                let mut chosen = Vec::<usize>::with_capacity(BOTTLEN);
                for i in random_last.iter() {
                    chosen.push(*i);
                }
                for c1 in 0..BOTTLEN {
                    for c2 in (c1+1)..BOTTLEN {
                        //println!("anc\t{}\t{}", c1, c2);
                        rounds[i][c1][c2] = sim.find_mrca_two(chosen[c1], chosen[c2]);    
                    }
                }
            } else {
                // call fx that does BOTTLEN simulations and fill rounds[1:3]
            }
        }
        for c1 in 0..BOTTLEN {
            for c2 in (c1+1)..BOTTLEN {
                println!("{}\t{}\t{}\t{}", c1, c2, rounds[0][c1][c2].0, rounds[0][c1][c2].1);
            }
        }
    }
}


// mess between n_trees, n_bottle and :(
fn simulate_bottlenecks(n_bottle: usize, n_gens: usize, p_death: f32, n_bottlenecks: usize) -> Vec<Vec<(usize, usize)>> {
    let n_trees = n_bottlenecks - 1; // substitute with n of leaf in a three with this height TODO
    let mut trees_index_in_alive = vec![0; n_bottle*n_trees]; 
    let mut alive_cells = Vec::<usize>::with_capacity(2_usize.pow(n_gens as u32)*n_trees);
    for i in 0 .. n_bottle {
        let a = simulate_bottleneck_run(n_gens, p_death);
        trees_index_in_alive[i] = a.len();
        alive_cells.extend(a);
    }
    // pick 100 random from alive_cells
    // iterate on all pairs of alive, if they are from different trees store (tree_id1, tree_id2)
    // if they are from the same determine common ancestor and store (level, level)
    // we'll need a mock Generations to do so.

    return vec![vec![(0,0)]]
}

fn simulate_bottleneck_run(n_gens: usize, p_death: f32) -> Vec<usize> {
    let mut sim = Generations::new(n_gens, p_death);
    sim.generate_cell(0);  
    return sim.alive_last_gen();
}