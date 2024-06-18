use rand::prelude::*;
use super::mod_exp;

pub fn miller_rabin(k: u64, n: u64) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'outer: for _ in 0..k {
        let mut a: u64 = thread_rng().gen_range(2..=(n - 2));

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