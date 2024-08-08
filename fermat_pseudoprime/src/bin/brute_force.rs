use std::{fs::OpenOptions, io::{BufWriter, Write}, sync::Arc, thread};

use fermat_pseudoprime::sieve_of_eratosthenes;
use lamellar::{ActiveMessaging, Darc};
use rug::Integer;

#[lamellar::AmData(Clone)]
struct ResultsAM {
    results: Darc<Vec<u64>>,
}

#[lamellar::am]
impl LamellarAm for ResultsAM {
    async fn exec(self) -> Vec<u64> {
        Vec::clone(&self.results)
    }
}

fn main () {
    let world = lamellar::LamellarWorldBuilder::new().build();

    let my_pe = world.my_pe();
    let num_pes = world.num_pes();

    let lower_bound = 3;
    let upper_bound = 2u64.pow(32);

    let step = (upper_bound - lower_bound)/ num_pes as u64;
    let local_min = (step * my_pe as u64) + lower_bound;
    let local_max = (step * (my_pe as u64 + 1)) + &lower_bound;

    let start = std::time::Instant::now();
    let primes = Arc::new(sieve_of_eratosthenes(upper_bound as u128));
    println!("pe:{}, done sieving, took {:?}", my_pe, start.elapsed());

    let results = Darc::new(&world, brute_force_search(local_min, local_max, &primes, 128)).unwrap();
    println!("pe:{}, done searching, took {:?}", my_pe, start.elapsed());
    world.barrier();

    if my_pe == 0 {
        let all_results: Vec<u64> = world.block_on(world.exec_am_all(ResultsAM{results})).into_iter().flatten().collect();

        println!("done message aggregating, found {}", all_results.len());
        let f = OpenOptions::new()
            .write(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .truncate(true)
            .open("plainlist.txt")
            .expect("Unable to open file");
        let mut stream = BufWriter::new(f);
        for prime in all_results {
            let string = prime.to_string() + "\n";
            stream.write_all(string.as_bytes()).expect("Unable to write data");   
        }
        stream.flush().unwrap();
    }

}

fn brute_force_search (min: u64, max: u64, primes: &Arc<Vec<bool>>, num_threads: u64) -> Vec<u64> {
    
    let mut handles = vec![];
    let step = (max-min) / num_threads;

    // println!("max: {}", max);
    for i in 0..num_threads {
        let thread_min = step * i + min;
        let mut thread_max = thread_min + step;
        let primes = primes.clone();
        if i == num_threads - 1 {
            thread_max = max;
        }
        handles.push(thread::spawn(move || single_thread(thread_min, thread_max, &primes)));
    }

    let mut res = Vec::new();

    for handle in handles {
        res.extend(handle.join().unwrap());
    }

    res
}

fn single_thread(mut min: u64, max: u64, primes: &Arc<Vec<bool>>) -> Vec<u64>{
    let mut pseudoprimes = Vec::new();
    if min % 2 == 0 {
        min += 1;
    }
    for i in (min..max).step_by(2) {
        if !primes.get(i as usize).unwrap() {
            if fermat_primality(i) {
                pseudoprimes.push(i)
            }
        }
    }

    pseudoprimes
}

pub fn fermat_primality (input: u64) -> bool{

    Integer::from(2).pow_mod(&Integer::from(input-1), &Integer::from(input)).unwrap() == 1

}