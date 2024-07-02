use rug::Integer;
// use std::cmp::min;
use rand::Rng;  // Import Rng trait for random number generation
use rand::thread_rng;

pub fn miller_rabin_bigrug(n: &Integer, k: u32) -> bool {
    if n == &Integer::from(2) {
        return true;
    }
    if n < &Integer::from(2) || n.is_even() {
        return false;
    }

    let mut d = Integer::from(n - 1);
    let mut r = 0;

    while d.is_even() {
        d >>= 1;
        r += 1;
    }

    'outer: for _ in 0..k {
        let mut rng = thread_rng();
        // let mut rand_state = RandState::new();
        let a: u64 = rng.gen_range(2..=n.to_u64().unwrap());
        let mut x = Integer::from(a).pow_mod(&d, n).unwrap();

        if x == 1 || x == Integer::from(n - 1) {
            continue 'outer;
        }

        for _ in 0..r - 1 {
            x = x.pow_mod(&Integer::from(2), n).unwrap();
            if x == Integer::from(n - 1) {
                continue 'outer;
            }
        }

        return false;
    }

    true
}