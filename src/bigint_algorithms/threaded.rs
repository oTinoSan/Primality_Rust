use super::{miller_rabin, solovay_strassen};
use rug::{Complete, Integer};
use std::thread;

pub fn miller_rabin_threaded(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    let per_thread = (&max / num_threads).complete();
    let mut threads = vec![];
    for i in 0..num_threads {
        let mut thread_min: Integer = (i * &per_thread).complete() + 5;
        if Integer::from(&thread_min % 2) == 0 {
            thread_min += 1;
        }
        let thread_max = ((i + 1) * &per_thread).complete() + 5;
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
    for handle in threads {
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
        let thread_max = ((i + 1) * &per_thread).complete() + 5;
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
    for handle in threads {
        let mut thread_results = handle.join().unwrap();
        results.append(&mut thread_results);
    }

    results
}
