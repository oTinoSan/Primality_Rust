use rayon::prelude::*;
use rug::{Complete, Integer};
use std::thread;

/// wheeling and threading a primality test using the 2-3-5 wheel of coprimes up to 30 to sieve
fn wheel_threaded(
    num_tests: u64,
    max: Integer,
    num_threads: u64,
    f: fn(u64, Integer) -> bool,
) -> Vec<Integer> {
    let coprimes = [1, 7, 11, 13, 17, 19, 23, 29];
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = Integer::from(&max / 30) / num_threads * i + 1;
        let end = Integer::from(&max / 30) / num_threads * (i + 1) + 1;
        let handle = thread::spawn(move || {
            num_iter::range(start, end)
                .map(move |i| i * 30)
                .map(move |i| std::iter::repeat(i).zip(coprimes).map(move |(i, j)| i + j))
                .flatten()
                .filter(|i: &Integer| f(num_tests, i.clone()))
                .collect::<Vec<_>>()
        });
        handles.push(handle);
    }

    handles
        .into_iter()
        .map(move |h| h.join().unwrap())
        .flatten()
        .collect()
}

fn general_wheel(
    num_tests: u64,
    min: Integer,
    max: Integer,
    test: fn(u64, Integer) -> bool,
    primes: Vec<u64>,
    coprimes: Vec<u64>,
) -> Vec<Integer> {
    let product = primes.iter().fold(1, |a, i| a * i);
    let start = min / product + 1;
    let end = max / product + 1;
    num_iter::range(start, end)
        .map(move |i| i * 30)
        .map(move |i| {
            std::iter::repeat(i)
                .zip(coprimes.clone())
                .map(move |(i, j)| i + j)
        })
        .flatten()
        .par_bridge()
        .filter(|i: &Integer| test(num_tests, i.clone()))
        .collect()
}

pub fn miller_rabin(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    wheel_threaded(num_tests, max, num_threads, super::miller_rabin)
}

pub fn solovay_strassen(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    wheel_threaded(num_tests, max, num_threads, super::solovay_strassen)
}

pub fn miller_rabin_rayon(num_tests: u64, max: Integer) -> Vec<Integer> {
    general_wheel(num_tests, Integer::ZERO, max, super::miller_rabin, vec![2, 3, 5], vec![1, 7, 11, 13, 17, 19, 23, 29])
}