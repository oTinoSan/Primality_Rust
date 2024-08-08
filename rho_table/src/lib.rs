use std::{collections::HashMap, vec};

use num::Integer;
use num_prime::nt_funcs::factorize64;

pub mod threaded_rho;



pub fn generate_table(limit: u64) -> HashMap<u64, u64> {

    let mut rho_table: HashMap<u64, u64> = HashMap::new();

    let list_of_primes = sieve_of_eratosthenes(limit as u128);

    let mut dynamic_array = HashMap::new();

    for i in (3..limit).step_by(2) {
        // let mut temp = 2;
        if list_of_primes[i as usize] {
            // if p is prime, then rho(p) is a divisor of p-1;
            let rho = calculate_order_prime(i);
            dynamic_array.insert(i, rho);
            rho_table.insert(i as u64, rho);
        } else {
            rho_table.insert(i as u64, calculate_order_composite(i as u64, &mut dynamic_array));
        }
        // if temp == i - 1 {
        //     rho_table.insert(i, i-1);
        // }
    }

    rho_table

}

pub fn calculate_order_prime_exp (p: u64, e: u32, rho: u64) -> u64 {

    let m = p.pow(e);
    let mut q = rho;
    loop {
        if mod_exp::mod_exp(2, q as u128, m as u128) == 1 {
            return q;
        }
        q += rho;
    }
}

pub fn calculate_order_prime (p: u64) -> u64 {
    let t = p-1;

    let mut qs = vec![1];
    let factors = factorize64((t) as u64);

    for (prime,exponent) in factors {
        let mut qs_new = Vec::new();

        for q in qs { // make power set of prime factors
            for j in 0..=exponent {
                qs_new.push(q * prime.pow(j.try_into().unwrap()))
            }
        }
        qs = qs_new;
    }

    qs.sort();

    for q in qs.clone() {
        if mod_exp::mod_exp(2, q as u128, p as u128) == 1 {
            return q;
        }
    }
    return qs.pop().unwrap();
}

pub fn calculate_order_composite (n: u64, dynamic_array: &mut HashMap<u64, u64>) -> u64 {
    
    let mut mofs = Vec::new();

    for (prime, exponent) in factorize64(n) {

        let rho = match dynamic_array.get(&prime) {
            Some(rho) => {
                *rho
            },
            None => {
                let rho_value = calculate_order_prime(prime);
                dynamic_array.insert(prime, rho_value);
                rho_value
            },
        };
        if exponent == 1 {
            mofs.push(rho);
        } else {
            mofs.push(calculate_order_prime_exp(prime, exponent as u32, rho))
        }
        
    }

    return mofs.iter().fold(1, |acc, x| acc.lcm(&x));
}

pub fn sieve_of_eratosthenes(int: u128) -> Vec<bool> {
    let mut array: Vec<bool> = vec![true; (int + 1) as usize];

    let mut i: usize = 2;
    array[0] = false;
    array[1] = false;
    while i <= ((f64::sqrt(int as f64)) as u128 + 1) as usize {
        if array[i] {
            for n in (i * i..int as usize).step_by(i) {
                array[n] = false;
            }
        }
        i = i + 1;
    }
    array[int as usize] = false;
    return array;
}