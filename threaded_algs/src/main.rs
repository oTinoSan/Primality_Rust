use crate::wheel_algos::wheel_threaded;
use crate::AKS_prime::u64AKS;
use crate::lamellar_prime_tests::main;
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
    let prime_test = main();
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
                if bigint_miller_rabin(thread_min.clone(), 10) {
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

pub fn bigint_miller_rabin(n: Integer, loop_amount: u64) -> bool {
    let mut rand = rand::RandState::new();
    let minus_one = Integer::from(&n - 1);
    let s = minus_one.find_one(0).unwrap();
    let d = Integer::from(&minus_one >> s);
    'outer: for _ in 0..loop_amount {
        let mut a =
            Integer::from(Integer::from(Integer::from(&n - 3).random_below_ref(&mut rand)) + 1);
        a = a.pow_mod(&d, &n).unwrap();
        if a == 1 {
            continue;
        }
        for _ in 0..s {
            if a == Integer::from(&n - 1) {
                continue 'outer;
            }
            // a = a.pow_mod(&MiniInteger::from(2).borrow(), &n).unwrap();
        }
        if a != minus_one {
            return false;
        }
    }
    true
}
