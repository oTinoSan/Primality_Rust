use rug::{Assign, Integer};
use std::cmp::min;
use rand::Rng;  // Import Rng trait for random number generation
use rand::thread_rng;

pub fn miller_rabin(n: &Integer, k: u32) -> bool {
    if n == &Integer::from(2) {
        return true;
    }
    if n < &Integer::from(2) || n.is_even() {
        return false;
    }

    let mut d: Integer = n - 1;
    let mut r = 0;

    while d.is_even() {
        d >>= 1;
        r += 1;
    }

    let mut rng = thread_rng();  // Use thread_rng for RNG

    'outer: for _ in 0..k {
        let a = Integer::from(2) + Integer::from(rng.gen_range(0..(n - 3u64).unwrap()));
        let mut x = mod_pow(&a, &d, &n);

        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 0..(r - 1) {
            x = mod_pow(&x, &Integer::from(2), &n);

            if x == n - 1 {
                continue 'outer;
            }
        }

        return false;
    }

    true
}
