use crate::wheel_algos::wheel_threaded;
use crate::AKS_prime::u64AKS;
use crate::lamellar_prime_tests::{lamellar, bigint_miller_rabin};
use rug::{rand, Complete, Integer}; //integer::MiniInteger,
use std::env;
use std::thread;

pub mod AKS_prime;
pub mod wheel_algos;
pub mod lamellar_prime_tests;

fn main() {
    // threaded_miller_rabin(Integer::from(1000000000), 8);
    // let limit = "10000000000";
    // let limit_int = limit.parse::<Integer>().unwrap();
    lamellar();
}

pub fn threaded_miller_rabin(limit: Integer, num_threads: u64) -> Vec<Integer> {
    let block_size = (&limit / num_threads).complete();

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: Integer = (i * &block_size).complete() + 5;
        let thread_max: Integer = ((i + 1) * &block_size).complete() + 5;

        if Integer::from(&thread_min % 2) == 0 {
            thread_min += 1;
        }
        thread_handles.push(std::thread::spawn(move || {
            let mut return_vector = Vec::new();
            while thread_min < thread_max {
                if bigint_miller_rabin(10, thread_min.clone()) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        }));
    }
    let mut results = vec![];
    for handle in thread_handles {
        let mut thread_results = handle.join().unwrap();
        results.append(&mut thread_results);
    }

    results
}

