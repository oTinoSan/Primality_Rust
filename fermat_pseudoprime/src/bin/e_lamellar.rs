use std::{collections::BTreeSet, fs::OpenOptions, io::{BufWriter, Write}, sync::Arc};

use fermat_pseudoprime::search_e::e_search_threaded;
use lamellar::{ActiveMessaging, Darc};
use num::integer::Roots;

#[lamellar::AmData(Clone)]
struct ResultsAM {
    results: Darc<Vec<u128>>,
}

#[lamellar::AmData(Clone)]
struct RhoTableAM {
    rho_table: Darc<Vec<u64>>,
}

#[lamellar::am]
impl LamellarAm for ResultsAM {
    async fn exec(self) -> Vec<u128> {
        Vec::clone(&self.results)
    }
}

#[lamellar::am]
impl LamellarAm for RhoTableAM {
    async fn exec(self) -> Vec<u64> {
        Vec::clone(&self.rho_table)
    }
}


fn main () {
    let world = lamellar::LamellarWorldBuilder::new().build();

    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    let start = std::time::Instant::now();
    
    let mut rho_table: Vec<u64> = Vec::new();
    if my_pe == 0 {
        let file = std::fs::read("rho_table.bin").unwrap();
        rho_table = bincode::deserialize(&file[..]).unwrap();
        println!("deserialized rho table, took {:?}", start.elapsed());
    }
    let rho_darc = Darc::new(&world, rho_table.clone()).unwrap();
    world.barrier();


    if my_pe != 0 {
        rho_table = world.block_on(world.exec_am_pe(0, RhoTableAM{rho_table: rho_darc}));
    }

    world.barrier();

    let rho_table = Arc::new(rho_table);

    let lower_bound = 3;
    let upper_bound = 2u128.pow(64);
    // let list_of_primes = sieve_of_eratosthenes(upper_bound);

    let step = (upper_bound - lower_bound)/ num_pes as u128;

    let local_min = (step * my_pe as u128) + lower_bound;
    let mut local_max = (step * (my_pe as u128 + 1)) + &lower_bound;
    if my_pe == num_pes - 1 {
        local_max = upper_bound.clone();
    }

    if my_pe == 0 {
        //deserialize rho table

    }

    let results = Darc::new(&world, e_search_threaded(local_min, local_max, upper_bound, rho_table, 128)).unwrap();

    world.barrier();

    if my_pe == 0 {
        let elapsed = start.elapsed();
        println!("search successful. time elapsed: {:?}", elapsed);

        let all_results: Vec<u128> = world.block_on(world.exec_am_all(ResultsAM{results})).into_iter().flatten().collect();

        let elapsed = start.elapsed();
        println!("aggregated messages. time elapsed: {:?}", elapsed);
        println!("searched from {} to {}, found {} pseudoprimes with method s.", lower_bound, upper_bound, all_results.len());
        
        let f = OpenOptions::new()
            .write(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .truncate(true)
            .open("data/s_fermat.txt")
            .expect("Unable to open file");
        let mut stream = BufWriter::new(f);
        let serialized = bincode::serialize(&all_results).unwrap();
        stream.write_all(&serialized).unwrap();
        stream.flush().unwrap();
    }
}