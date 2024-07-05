use super::mod_pow::mod_pow;
use rand::prelude::*;

pub fn miller_rabin(candidate: u64, iterations: i32) -> bool {
    match candidate {
        0 | 1 => false, // 0 and 1 are not prime
        2 | 3 => true,  // 2 and 3 are prime
        _ => {
            let mut rng = rand::thread_rng();
            let mut d = candidate - 1;
            let mut r = 0;
            while d % 2 == 0 {
                d /= 2;
                r += 1;
            }

            'outer: for _ in 0..iterations {
                // Adjusted to ensure the range is valid for candidates greater than 3
                let a = rng.gen_range(2..candidate);
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
    }
}
