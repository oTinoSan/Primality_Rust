use std::{fs::OpenOptions, io::{BufWriter, Write}, ops::Deref, sync::Mutex};

use lamellar::{self, ActiveMessaging, Darc};
use primality_testing::prime_search::threaded_prime_search;
use rug::{ops::SubFrom, Complete, Integer};

#[lamellar::AmData(Clone)]
struct ResultsAM {
    results: Darc<Mutex<Vec<Integer>>>,
}

#[lamellar::AmData(Clone)]
struct PrimeCountAM {
    results: Darc<Mutex<usize>>,
}


#[lamellar::am]
impl LamellarAm for ResultsAM {
    async fn exec(self) -> Vec<Integer> {
        let test = self.results.lock().unwrap();
        test.deref().clone()
    }
}

#[lamellar::am]
impl LamellarAm for PrimeCountAM {
    async fn exec(self) -> usize {
        let test = self.results.lock().unwrap();
        test.deref().clone()
    }
}


fn main () {
    let world = lamellar::LamellarWorldBuilder::new().build();
    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    let start = std::time::Instant::now();

    let lower_bound = Integer::from(0);
    let upper_bound = Integer::from(1_000_000_000i64);

    let step = Integer::from((&upper_bound - &lower_bound).complete() / num_pes);

    let local_min = Integer::from(&step * my_pe) + &lower_bound;
    let mut local_max = Integer::from(&step * (my_pe + 1)) + &lower_bound;
    if my_pe == num_pes - 1 {
        local_max = upper_bound.clone();
    }

    let results = threaded_prime_search(local_min, local_max, 128);

    

    let (primes, b2psp, lsps, slsps, bpswsps) = results;

    let primes = Darc::new(&world, Mutex::new(primes.len())).unwrap();
    let b2psp = Darc::new(&world, Mutex::new(b2psp)).unwrap();
    let lsps = Darc::new(&world, Mutex::new(lsps)).unwrap();
    let slsps = Darc::new(&world, Mutex::new(slsps)).unwrap();
    let bpswsps = Darc::new(&world, Mutex::new(bpswsps)).unwrap();

    world.barrier();
    

    if my_pe == 0 {
        let elapsed = start.elapsed();
        println!("time elapsed: {:?}", elapsed);

        let all_primes: usize = world.block_on(world.exec_am_all(PrimeCountAM{results:primes.clone()})).iter().sum();
        let all_b2psp: Vec<Integer> = world.block_on(world.exec_am_all(ResultsAM{results:b2psp.clone()})).into_iter().flatten().collect();
        let all_lsps: Vec<Integer> = world.block_on(world.exec_am_all(ResultsAM{results:lsps.clone()})).into_iter().flatten().collect();
        let all_slsps: Vec<Integer> = world.block_on(world.exec_am_all(ResultsAM{results:slsps.clone()})).into_iter().flatten().collect();
        let all_bpswsps: Vec<Integer> = world.block_on(world.exec_am_all(ResultsAM{results:bpswsps.clone()})).into_iter().flatten().collect();
        let elapsed = start.elapsed();

        println!("aggregated messages. time elapsed: {:?}", elapsed);
        println!("searched from {} to {}, found: \n{} primes, \n{} strong base 2 pseudoprimes, \n{} lucas pseudoprimes, \n{} strong lucas pseudoprimes, \n{} baillie psw pseudoprimes", lower_bound, upper_bound, all_primes, all_b2psp.len(), all_lsps.len(), all_slsps.len(), all_bpswsps.len());       

        println!("bpsw pseudoprimes: {:?}", all_bpswsps);

        let f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open("data/spsp2.txt")
            .expect("Unable to open file");
        let mut stream = BufWriter::new(f);
        for prime in all_b2psp {
            let string = prime.to_string() + "\n";
            stream.write_all(string.as_bytes()).expect("Unable to write data");   
        }
        stream.flush().unwrap();

        let f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open("data/lpsp.txt")
            .expect("Unable to open file");
        let mut stream = BufWriter::new(f);
        for prime in all_lsps {
            let string = prime.to_string() + "\n";
            stream.write_all(string.as_bytes()).expect("Unable to write data");   
        }
        stream.flush().unwrap();

        let f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open("data/slpsp.txt")
            .expect("Unable to open file");
        let mut stream = BufWriter::new(f);
        for prime in all_slsps{
            let string = prime.to_string() + "\n";
            stream.write_all(string.as_bytes()).expect("Unable to write data");   
        }
        stream.flush().unwrap();

        let f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open("data/bpswsp.txt")
            .expect("Unable to open file");
        let mut stream = BufWriter::new(f);
        for prime in all_bpswsps {
            let string = prime.to_string() + "\n";
            stream.write_all(string.as_bytes()).expect("Unable to write data");   
        }
        stream.flush().unwrap();
    }



}