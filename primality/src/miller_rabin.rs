use rand::prelude::*;
use super::mod_pow;


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