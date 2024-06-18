use rand::Rng;

fn main() {

    // let result = trial_vect(1000);
    // println!("{:?}", result);
    // let result = trial(1000);
    // println!("{:?}", result);

    // let result = trial_vect_2(1000);
    // println!("{:?}", result);
    // let result = trial_2(1000);
    // println!("{:?}", result);

    // let result = sieve(1000);
    // println!("{:?}", result);
    // let result = sieve_vect(1000);
    // println!("{:?}", result);

    // let result = wheel_facts(1000);
    // println!("{:?}", result);
    // let result = wheel(1000);
    // println!("{:?}", result);

    // let result = miller_rabin(1000);
    // println!("{:?}", result);

}

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

pub fn sieve(candidate: u32) -> bool {
    if candidate <= 1 {
        return false;
    }
    let limit = (candidate as f64).sqrt() as u32;
    let mut primes = vec![true; (candidate + 1) as usize]; 
    primes[0] = false;
    primes[1] = false;

    for num in 2..=limit {
        if primes[num as usize] {
            let mut multiple = num * num;
            while multiple <= candidate { // Change here
                primes[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    primes[candidate as usize]
}

pub fn sieve_vect(limit: u32) -> Vec<u32> {
    (2..=limit).filter(|&num| sieve(num)).collect()
}

pub fn wheel_facts(candidate: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remainder = candidate;
    let wheel = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    for &prime in wheel.iter() {
        while remainder % prime == 0 {
            factors.push(prime);
            remainder /= prime;
        }
    }

    let mut f = 103; // Start from the next prime after 97
    while f * f <= remainder {
        if remainder % f == 0 {
            factors.push(f);
            remainder /= f;
        } else {
            f += 2; // Skip even numbers
        }
    }

    if remainder != 1 {
        factors.push(remainder);
    }

    factors
}

pub fn wheel(candidate: u64) -> bool {
    let factors = wheel_facts(candidate);
    factors.len() <= 2 
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

pub fn miller_rabin(candidate: u64) -> bool {
    if candidate < 2 {
        return false;
    }
    if candidate != 2 && candidate % 2 == 0 {
        return false;
    }

    let mut rng = rand::thread_rng();
    let mut d = candidate - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    'outer: for _ in 0..5 {
        let a = rng.gen_range(2..candidate - 1);
        let mut x = mod_pow(a, d, candidate);
        if x == 1 || x == candidate - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = mod_pow(x, 2, candidate);
            if x == candidate - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
