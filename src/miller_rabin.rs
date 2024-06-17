use rand::prelude::*;

pub fn miller_rabin(k: u64, n: u64) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'outer: for _ in 0..k {
        let mut a: u64 = thread_rng().gen_range(2..=(n-2));

        a = mod_exp(a, d, n);

        if a == 1 {
            continue;
        }
        for _ in 0..s {
            if a == n - 1 {
                continue 'outer;
            }
            a = mod_exp(a, 2, n);
        }
        if a != n - 1 {
            return false;
        }
    }
    return true;
}

pub fn miller_rabin_list(k: u64, n: u64) -> Vec<u64> {
    let mut primes = vec![];
    for i in 5..=n {
        if miller_rabin(k, i) {
            primes.push(i);
        }
    }
    primes
}

fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn mod_exp_naive(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut res = 1;
    for _ in 0..exp {
        res = (res * base) % modulus;
    }
    return res;
}
