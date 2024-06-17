use mod_exp::mod_exp;
use rand::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number: u32 = args[1].trim().parse().expect("Must be an integer");
    // let is_prime = miller_test(number);
    let primes = miller_list(number);
    println!("{:?}", primes);
}

fn miller_list(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    // let sqrt = (limit as f64).sqrt() as u32;
    for num in 2..=limit {
        if miller_test(num) {
            primes.push(num);
        }
    }
    primes
}

fn miller_test(candidate: u32) -> bool {
    // while n<=number{
    let mut is_prime = false;
    let mut k = 0;
    let mut divisor = candidate - 1;
    let mut b;
    loop {
        if divisor % 2 == 0 {
            k += 1;
            divisor = divisor / 2;
        } else {
            break;
        }
    }
    // for i in 0..50 {
    let a = thread_rng().gen_range(2..=(candidate - 2));
    println!("{a}");
    b = mod_exp(a, divisor, candidate);
    if b == 1 {
        is_prime = true;
    } else {
        for i in (0..k).step_by(2) {
            if b == (candidate - 1) % candidate {
                is_prime = true;
                break;
            } else {
                b = mod_exp(b, 2, candidate);
            }
        }
    }
    // }
    return is_prime;
}

// fn modpow(number, ){
//     let mut modpow = a % candidate;
//     while index_2 < mod_numbers {
//         index_2 += 1;
//         modpow = (modpow * index_1) % mod_numbers;
//     }
// }
