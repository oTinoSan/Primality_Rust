use core::panic;
use std::{collections::{BTreeMap, BTreeSet}, sync::Arc, thread};
use num::integer::lcm;
use num_prime::nt_funcs::factorize128;

use crate::{fermat_primality, modular_inverse, sieve_of_eratosthenes};


fn category_s_pseudoprime_search(mut min: u128, max: u128, global_max: u128, two_thirds_global_max: u128, sqrt_global_max: u128, rho_table: Arc<Vec<u64>>) -> Vec<u128>{
    if min % 2 == 0 {
        min += 1;
    }

    let block_size = 10_000_000;
    let range = max - min;
    let num_blocks = range / block_size;
    let mut pseudoprimes = Vec::new();

    //for each block
    for i in 0..=num_blocks {

    
        let mut block_min = min + (block_size * i);
        if block_min % 2 == 0 {
            block_min += 1;
        }
        let mut block_max = min + (block_size * (i + 1));
        if i == num_blocks {
            block_max = max;
        }
        // println!("searching from {} to {}, primes up to {}",block_min, block_max, max);
        //calculate q1 and q2 for each q
        for q in (block_min..block_max).step_by(2) {
            let rho: u64;
            let factors = factorize128(q);
            if *factors.first_key_value().unwrap().0 == q {
                continue; // q is prime, don't run it.
            }
            if q < sqrt_global_max { // if we have precomputed rho, just look it up
                rho = *rho_table.get(((q-1)/2-1) as usize).unwrap();
            } else {
                if let Some((q1, q2)) = get_q1_q2(sqrt_global_max, factors) {
                    if q2 == 1 {
                        continue;
                    }
                    // println!("q: {}, q1: {}, q2: {}", q, q1, q2);
                    rho = lcm(*rho_table.get(((q1-1)/2-1) as usize).unwrap(), *rho_table.get(((q2-1)/2-1) as usize).unwrap());

                } else {
                    continue;
                }
            }

            // iterate on s s.t. s*q = n is a pseudoprime
            let q_inv = modular_inverse(q as i128, rho as i128) as u128;
            let mut s = q_inv % rho as u128;
            if s == 1 {
                continue; // no modular inverse exists
            }

            while s < q {
                if s % 2 == 0 {
                    s+= rho as u128;
                    continue;
                }
                let product = q*s;
                if product > global_max {
                    s+= rho as u128;
                    continue;
                }
                // println!("q: {}, rho: {}, s: {}", q, rho, s);
                // if rug::Integer::from(product).modulo(&rug::Integer::from(*rho_table.get(((s-1)/2-1) as usize).unwrap())) == 1{
                //     pseudoprimes.push(product);
                // }
                if fermat_primality(product) {
                    pseudoprimes.push(product);
                }

                s += rho as u128;
            
                // r.insert(q, get_q1_q2(q, 2u128.pow(32)));
            }
        }

    }
    pseudoprimes

}

pub fn s_search_threaded(min: u128, max: u128, global_max: u128, two_thirds_global_max: u128, sqrt_global_max: u128, rho_table: Arc<Vec<u64>>, num_threads: u128) -> BTreeSet<u128> {
    let mut handles = vec![];
    let step = (two_thirds_global_max-min) / num_threads;

    // println!("max: {}", max);
    for i in 0..num_threads {
        let rho_table = rho_table.clone();
        let thread_min = step * i + min;
        let mut thread_max = thread_min + step;
        if i == num_threads - 1 {
            thread_max = two_thirds_global_max;
        }
        handles.push(thread::spawn(move || category_s_pseudoprime_search(thread_min, thread_max, global_max, two_thirds_global_max, sqrt_global_max, rho_table)));
    }

    let mut res = BTreeSet::new();

    for handle in handles {
        res.extend(handle.join().unwrap());
    }

    res
}

// different version that iterates through prime factorization of input q
pub fn get_q1_q2 (sqrt_limit: u128, prime_factors: BTreeMap<u128, usize>) -> Option<(u128, u128)> {
    let n = prime_factors.len();

    // println!("{}, {:?}",n, prime_factors);

    // Iterate through all possible subsets using bitmasking
    for bitmask in 1..(1 << n) {
        let mut q1_subset = Vec::new();
        let mut q1: u128 = 1;
        let mut q2_subset = Vec::new();
        let mut q2: u128 = 1;


        // Include elements based on bitmas
        let mut i = 0;
        for (prime, exponent) in &prime_factors {
            if bitmask & (1 << i) != 0 {
                q1 *= *prime * (*exponent as u128);
                q1_subset.push(*prime);
            } else {
                q2 *= *prime * (*exponent as u128);
                q2_subset.push(*prime);
            }
            i+=1;
        }
        if q2 > q1 {
            (q1, q2) = (q2, q1);
        }
        if q1 > sqrt_limit {
            continue;
        }
        // sanity check
        // if gcd(q1, q2) != 1 {
        //     return None;
        // }
        return Some((q1, q2));
    }

    // then q is not n^1/2 smooth, stop searching
    None
    // panic!("unreachable state: composite: {}, limit: {}", composite, sqrt_limit);
    // return (1,1)
}

fn _get_rho_of_prime(rho_table: &Vec<u64>, input: u128) -> u128 {

    let rho_option = rho_table.get(((input-1)/2-1) as usize);
    
    let rho: u128;
    if rho_option.is_none() {
        // calculate rho
        panic!()
    } else {
        rho = *rho_option.unwrap() as u128;
    }

    rho
}