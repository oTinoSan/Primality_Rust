use std::{sync::Arc, thread};

use rug;

use crate::{calculate_order_prime, sieve_of_eratosthenes};

pub fn e_search_threaded(min: u128, max: u128, global_max: u128, rho_table: Arc<Vec<u64>>, num_threads: u128) -> Vec<u128> {
    let mut handles = vec![];
    let step = (max-min) / num_threads;

    let primes = sieve_of_eratosthenes(max);
    for i in 0..num_threads {
        let rho_table = rho_table.clone();
        let thread_min = step * i + min;
        let mut thread_max = thread_min + step;
        let prime = primes.clone();
        if i == num_threads - 1 {
            thread_max = max;
        }
        handles.push(thread::spawn(move || category_e_pseudoprime_search(thread_min, thread_max, global_max, rho_table,prime)));
    }

    let mut res = Vec::new();

    for handle in handles {
        res.extend(handle.join().unwrap());
    }

    res
}

pub fn category_e_pseudoprime_search(min: u128, max: u128, global_max: u128, rho_table: Arc<Vec<u64>>, list_of_primes: Vec<bool>) -> Vec<u128> {
    
    let mut pseudoprimes = Vec::new();
    
    let mut p: u128 = min;
    while p < global_max/2 && p < max{
        if !list_of_primes[p as usize] {
            p+=2;
            continue;
        }
        
        let rho: u128 = get_rho_of_prime(&rho_table, p);

        let mut s: u128;
        if rho % 2 == 0 {
            if !(p <= (global_max/(rho+1))) {
                p+=2;
                continue;
            }
            s = rho + 1;
        } else {
            if !(p <= (global_max/(2*rho + 1))) {
                p +=2;
                continue;
            }
            s = 2 * rho + 1;
        }
        // println!("testing {}, rho: {}, starting s: {}", p, rho, s);
        while s < p {
            // println!("s: {}, s*p: {}, s*p mod rho(s) = {}", s, s*p, Integer::from(p*s).modulo(&Integer::from(rho_table[s as usize])));
            let product = p*s;
            if product > global_max {
                s+= rho;
                continue;
            }
            if rug::Integer::from(product).modulo(&rug::Integer::from(get_rho_of_prime(&rho_table, s))) == 1{
                pseudoprimes.push(product);
                // p+= 2;
                // continue;
            }
            
            s += rho;
        }

        p += 2;
    }

    pseudoprimes
}

pub fn get_rho_of_prime(rho_table: &Vec<u64>, input: u128) -> u128 {

    let rho_option = rho_table.get(((input-1)/2-1) as usize);
    
    let rho: u128;
    if rho_option.is_none() {
        // calculate rho
        rho = calculate_order_prime(input);
    } else {
        rho = *rho_option.unwrap() as u128;
    }

    rho
}