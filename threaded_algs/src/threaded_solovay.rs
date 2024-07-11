use std::env;
use rug::{Integer, Complete, rand};

pub fn threaded_solovay_strassen(limit: Integer, num_threads: u64) -> Vec<Integer> {
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
                if bigint_solovay_strassen(10, thread_min.clone()) {
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

pub fn bigint_solovay_strassen(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    for _ in 0..num_tests{
        let a = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        let jacobi_result = a.clone().jacobi(&candidate);
        let mod_result = a.pow_mod(&(Integer::from(&candidate -1)/2), &candidate).unwrap();
        if mod_result == Integer::from(0) {
            return false;
        }
        if (mod_result == jacobi_result) || (mod_result == Integer::from(&candidate -1) && jacobi_result == -1) {
            return true;
        }
    }
    return false;
    }

pub fn bigint_solovay_strassen_list(num_tests: u64, max_val: Integer) -> Vec<Integer>{
    let mut primes= vec![];
    let mut i = Integer::from(5);

    while i <= max_val {
        if bigint_solovay_strassen(num_tests, i.clone()){
            primes.push(i.clone());
        }
        i = i + 2;
    }

    primes
}