use mod_exp::mod_exp;
use rand::Rng;

use rug::Integer;

pub fn miller_rabin_array(limit: Integer) -> Vec<Integer> {
    let mut array = Vec::new();
    let mut i: Integer = Integer::from(5);
    while &i < &limit {
        if bigint_miller_rabin(&i, 10) {
            array.push(i.clone());
        }
        i += 2;
    }
    return array;
}

pub fn threaded_miller_rabin(limit: Integer, num_threads: u64) -> Vec<Integer> {
    let block_size = Integer::from(&limit / num_threads);

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: Integer = i * Integer::from(&block_size) + 5;
        let thread_max: Integer = (i + 1) * Integer::from(&block_size) + 5;

        if Integer::from(&thread_min) % 2 == Integer::ZERO {
            thread_min += 1;
        }
        let thread = std::thread::spawn(move || {
            let mut return_vector = Vec::new();
            while thread_min < thread_max {
                if bigint_miller_rabin(&thread_min, 10) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        });
        thread_handles.push(thread);
    }

    let mut return_vector: Vec<Integer> = Vec::new();
    // join all of the threads
    for handle in thread_handles {
        return_vector.append(&mut handle.join().unwrap());
    }

    return_vector
}

pub fn bigint_miller_rabin(n: &Integer, loop_amount: u64) -> bool {
    if *n == 5 || *n == 7 {
        return true;
    }
    let s = (n - Integer::from(1)).find_one(0).unwrap();
    let d: Integer = Integer::from(n - 1) / (2u64.pow(s.try_into().unwrap()));
    let mut rand = rug::rand::RandState::new();

    // println!("prime_candidate: {}, s: {}, d: {}, 2^s*d + 1 = {}", prime_candidate, s, d, {2u32.pow(s)*d+1});
    for _ in 0..loop_amount {
        let a: Integer = (n - Integer::from(2)).random_below(&mut rand);
        let mut x = a.pow_mod(&d, &n).unwrap();
        let mut y: Integer = Integer::from(0);
        for _ in 0..s {
            y = x.clone().pow_mod(&Integer::from(2), &n).unwrap();
            if y == Integer::from(1) && x != Integer::from(1) && x != Integer::from(n - 1) {
                return false;
            }
            x = y.clone();
        }
        if y != Integer::from(1) {
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
