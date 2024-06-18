use rand::Rng;
pub mod miller_rabin;
pub mod solovay_strassen;
pub mod sieve;
pub mod wheel;

pub fn trial_vect(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for num in 2..=limit {
        if trial(num) {
            primes.push(num);
        }
    }
    primes
}

pub fn trial(candidate: u32) -> bool {
    let sqrt = (candidate as f64).sqrt() as u32;
    let mut is_prime = true;
    for divisor in 2..=sqrt {
        if candidate % divisor == 0 {
            is_prime = false;
            break;
        }
    }
    // println!("{} is prime? {}", candidate, is_prime);
    is_prime
}

pub fn trial_vect_2(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for num in 2..=limit {
        if trial_2(num) {
            primes.push(num);
        }
    }
    primes
}

pub fn trial_2(candidate: u32) -> bool {
    if candidate <= 1 {
        return false;
    }
    if candidate == 2 {
        return true;
    }
    if candidate % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while (i * i) <= candidate {
        if candidate % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}
