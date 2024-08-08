use mod_exp::mod_exp;
use num::{bigint::RandBigInt, BigInt};
use rand::Rng;

pub fn miller_rabin_array(limit: BigInt) -> Vec<BigInt> {
    let mut array = Vec::new();
    let mut i: BigInt = BigInt::from(5);
    while &i < &limit {
        if bigint_miller_rabin(i.clone(), 10) {
            array.push(i.clone());
        }
        i += 2;
    }
    return array;
}

pub fn threaded_miller_rabin(limit: BigInt, num_threads: u64) -> Vec<BigInt> {
    let block_size = &limit / num_threads;

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: BigInt = i * &block_size + 5;
        let thread_max: BigInt = (i + 1) * &block_size + 5;

        if &thread_min % 2 == BigInt::ZERO {
            thread_min += 1;
        }
        let thread = std::thread::spawn(move || {
            let mut return_vector = Vec::new();
            while thread_min < thread_max {
                if bigint_miller_rabin(thread_min.clone(), 10) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        });
        thread_handles.push(thread);
    }

    let mut return_vector: Vec<BigInt> = Vec::new();
    // join all of the threads
    for handle in thread_handles {
        return_vector.append(&mut handle.join().unwrap());
    }

    return_vector
}

pub fn bigint_miller_rabin(n: BigInt, loop_amount: u64) -> bool {
    let s: u64 = (&n - BigInt::from(1)).trailing_zeros().unwrap();
    let d: BigInt = (&n - 1) / (2u64.pow(s.try_into().unwrap())) as u64;

    // println!("prime_candidate: {}, s: {}, d: {}, 2^s*d + 1 = {}", prime_candidate, s, d, {2u32.pow(s)*d+1});
    for _ in 0..loop_amount {
        let a: BigInt = rand::thread_rng()
            .gen_bigint_range(&BigInt::from(2), &(&n - BigInt::from(2)))
            .try_into()
            .unwrap();
        let mut x = a.modpow(&d, &n);
        let mut y: BigInt = BigInt::from(0);
        for _ in 0..s {
            y = x.modpow(&BigInt::from(2), &n);
            if y == BigInt::from(1) && x != BigInt::from(1) && x != &n - 1 {
                return false;
            }
            x = y.clone();
        }
        if y != BigInt::from(1) {
            return false;
        }
    }

    return true; // probably prime
}

pub fn miller_rabin_probabilistic(n: u64, loop_amount: u64) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) / (2u32.pow(s)) as u64;

    // println!("prime_candidate: {}, s: {}, d: {}, 2^s*d + 1 = {}", prime_candidate, s, d, {2u32.pow(s)*d+1});
    for _ in 0..loop_amount {
        let a: u64 = rand::thread_rng()
            .gen_range(2..=(n - 2))
            .try_into()
            .unwrap();
        let mut x = mod_exp(a, d, n);
        let mut y = 0;
        for _ in 0..s {
            y = mod_exp(x, 2, n);
            if y == 1 && x != 1 && x != n - 1 {
                return false;
            }
            x = y;
        }
        if y != 1 {
            return false;
        }
    }

    return true; // probably prime
}
