use rug::{integer::MiniInteger, rand, Complete, Integer};
use std::thread;

pub fn solovay_strassen(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    for _ in 0..num_tests {
        let test_base = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        let jacobi = test_base.jacobi(&candidate);
        let powmod_result = test_base
            .pow_mod(&(Integer::from(&candidate - 1) / 2), &candidate)
            .unwrap();
        if !((powmod_result == jacobi)
            || (powmod_result == Integer::from(&candidate - 1) && jacobi == -1))
        {
            return false;
        }
    }
    return true;
}

pub fn miller_rabin(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    let minus_one = Integer::from(&candidate - 1);
    let s = minus_one.find_one(0).unwrap();
    let d = Integer::from(&minus_one >> s);
    'outer: for _ in 0..num_tests {
        let mut a = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        a = a.pow_mod(&d, &candidate).unwrap();
        if a == 1 {
            continue;
        }
        for _ in 0..s {
            if a == Integer::from(&candidate - 1) {
                continue 'outer;
            }
            a = a
                .pow_mod(&MiniInteger::from(2).borrow(), &candidate)
                .unwrap();
        }
        if a != minus_one {
            return false;
        }
    }
    true
}

pub fn miller_rabin_range(num_tests: u64, max: Integer) -> Vec<Integer> {
    let mut r = vec![];
    let mut idx = Integer::from(5);
    while idx < max {
        if miller_rabin(num_tests, idx.clone()) {
            r.push(idx.clone());
        }
        idx += 2;
    }
    r
}

pub fn solovay_strassen_range(num_tests: u64, max: Integer) -> Vec<Integer> {
    let mut r = vec![];
    let mut idx = Integer::from(5);
    while idx < max {
        if solovay_strassen(num_tests, idx.clone()) {
            r.push(idx.clone());
        }
        idx += 2;
    }
    r
}

pub fn miller_rabin_threaded(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    let per_thread = (&max / num_threads).complete();
    let mut threads = vec![];
    for i in 0..num_threads {
        let mut thread_min: Integer = (i * &per_thread).complete() + 5;
        if Integer::from(&thread_min % 2) == 0 {
            thread_min += 1;
        }
        let thread_max = ((i + 1) * &per_thread).complete();
        threads.push(thread::spawn(move || {
            let mut r = vec![];
            let mut idx = thread_min;
            while idx < thread_max {
                if miller_rabin(num_tests, idx.clone()) {
                    r.push(idx.clone());
                }
                idx += 2;
            }
            r
        }));
    }
    let mut results = vec![];
    for _ in 0..num_threads {
        let handle = threads.remove(0);
        let mut thread_results = handle.join().unwrap();
        results.append(&mut thread_results);
    }

    results
}

pub fn solovay_strassen_threaded(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    let per_thread = (&max / num_threads).complete();
    let mut threads = vec![];
    for i in 0..num_threads {
        let mut thread_min: Integer = (i * &per_thread).complete() + 5;
        if Integer::from(&thread_min % 2) == 0 {
            thread_min += 1;
        }
        let thread_max = ((i + 1) * &per_thread).complete();
        threads.push(thread::spawn(move || {
            let mut r = vec![];
            let mut idx = thread_min;
            while idx < thread_max {
                if solovay_strassen(num_tests, idx.clone()) {
                    r.push(idx.clone());
                }
                idx += 2;
            }
            r
        }));
    }
    let mut results = vec![];
    for _ in 0..num_threads {
        let handle = threads.remove(0);
        let mut thread_results = handle.join().unwrap();
        results.append(&mut thread_results);
    }

    results
}