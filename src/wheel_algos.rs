use rayon::prelude::*;
use rug::{Integer, Complete, rand};
use std::{thread};
use crate::advanced_primality_tests::*;
use std::sync::Arc;

pub mod advanced_primality_tests;

pub fn wheel_threaded(
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

pub fn general_wheel_threaded(
    num_tests: u64,
    min: Integer,
    max: Integer,
    num_threads: u64,
    test: fn(u64, Integer) -> bool,
    primes: Vec<u64>,
    coprimes: Vec<u64>,
) -> Vec<Integer> {
    let product: u64 = primes.iter().product();
    let start = min.clone() / product + 1;
    let end = max.clone() / product + 1;
    let step = Integer::from(&end - &start) / num_threads;
    let mut handles = vec![];
    let coprimes = Arc::new(coprimes);

    for i in 0..num_threads {
        let thread_start = Integer::from(&start) + &step * i;
        let thread_end = Integer::from(&thread_start + &step);
        let coprimes = Arc::clone(&coprimes);
        let min = min.clone();
        let max = max.clone();
        handles.push(thread::spawn(move || {
            let mut idx = thread_start;
            let mut r = vec![];
            if i == 0 {
                idx -= 1;
                if idx != 0 {
                    for c in coprimes.iter().rev() {
                        let candidate = Integer::from(&idx * product) + c;
                        if candidate < min {
                            break;
                        } else if test(num_tests, candidate.clone()) {
                            r.push(candidate);
                        }
                    }
                }
                idx += 1;
            }
            while idx < thread_end {
                for c in coprimes.iter() {
                    let candidate = Integer::from(&idx * product) + c;
                    if candidate > max {
                        break;
                    }
                    if test(num_tests, candidate.clone()) {
                        r.push(candidate);
                    }
                }
                idx += 1;
            }
            if i == num_threads - 1 {
                'outer: loop {
                    for c in coprimes.iter() {
                        let candidate = Integer::from(&idx * product) + c;
                        if candidate > max {
                            break 'outer;
                        }
                        if test(num_tests, candidate.clone()) {
                            r.push(candidate);
                        }
                    }
                    idx += 1;
                }
            }
            r
        }))
    }
    handles
        .into_iter()
        .map(move |h| h.join().unwrap())
        .flatten()
        .collect()
}


pub fn general_wheel_rayon(
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
    wheel_threaded(num_tests, max, num_threads, bigint_miller_rabin)
}

pub fn solovay_strassen(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    wheel_threaded(num_tests, max, num_threads, bigint_solovay_strassen)
}

pub fn miller_rabin_general(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    general_wheel_threaded(
        num_tests,
        Integer::ZERO,
        max,
        num_threads,
        bigint_miller_rabin,
        vec![2, 3, 5],
        vec![1, 7, 11, 13, 17, 19, 23, 29],
    )
}

pub fn solovay_strassen_general(num_tests: u64, max: Integer, num_threads: u64) -> Vec<Integer> {
    general_wheel_threaded(
        num_tests,
        Integer::ZERO,
        max,
        num_threads,
        bigint_solovay_strassen,
        vec![2, 3, 5],
        vec![1, 7, 11, 13, 17, 19, 23, 29],
    )
}


pub fn wheel_threaded_two_fn(
    num_tests: u64,
    max: Integer,
    num_threads: u64,
    f: fn(u64, Integer) -> bool, g: fn(u64, Integer)-> bool
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
                .filter(|i: &Integer| g(num_tests, i.clone()))
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

pub fn general_wheel_threaded_two_fn(
    num_tests: u64,
    min: Integer,
    max: Integer,
    num_threads: u64,
    test_1: fn(u64, Integer) -> bool, test_2: fn(u64, Integer) -> bool,
    primes: Vec<u64>,
    coprimes: Vec<u64>,
) -> Vec<Integer> {
    let product: u64 = primes.iter().product();
    let start = min.clone() / product + 1;
    let end = max.clone() / product + 1;
    let step = Integer::from(&end - &start) / num_threads;
    let mut handles = vec![];
    let coprimes = Arc::new(coprimes);

    for i in 0..num_threads {
        let thread_start = Integer::from(&start) + &step * i;
        let thread_end = Integer::from(&thread_start + &step);
        let coprimes = Arc::clone(&coprimes);
        let min = min.clone();
        let max = max.clone();
        handles.push(thread::spawn(move || {
            let mut idx = thread_start;
            let mut r = vec![];
            if i == 0 {
                idx -= 1;
                if idx != 0 {
                    for c in coprimes.iter().rev() {
                        let candidate = Integer::from(&idx * product) + c;
                        if candidate < min {
                            break;
                        } else if test_1(num_tests, candidate.clone()) && test_2(num_tests, candidate.clone()){
                            r.push(candidate);
                        }
                    }
                }
                idx += 1;
            }
            while idx < thread_end {
                for c in coprimes.iter() {
                    let candidate = Integer::from(&idx * product) + c;
                    if candidate > max {
                        break;
                    }
                    if test_1(num_tests, candidate.clone()) {
                        r.push(candidate);
                    }
                }
                idx += 1;
            }
            if i == num_threads - 1 {
                'outer: loop {
                    for c in coprimes.iter() {
                        let candidate = Integer::from(&idx * product) + c;
                        if candidate > max {
                            break 'outer;
                        }
                        if test_1(num_tests, candidate.clone()) && test_2(num_tests, candidate.clone()) {
                            r.push(candidate);
                        }
                    }
                    idx += 1;
                }
            }
            r
        }))
    }
    handles
        .into_iter()
        .map(move |h| h.join().unwrap())
        .flatten()
        .collect()
}

pub fn miller_solovay(num_tests:u64, max:Integer, num_threads: u64) -> Vec<Integer>{
    wheel_threaded_two_fn(num_tests, max, num_threads, bigint_miller_rabin, bigint_solovay_strassen)
}

pub fn general_miller_solovay(num_tests:u64, max:Integer, num_threads: u64) -> Vec<Integer>{
    general_wheel_threaded_two_fn(num_tests, Integer::ZERO, max, num_threads, bigint_miller_rabin, bigint_solovay_strassen, vec![2, 3, 5],
        vec![1, 7, 11, 13, 17, 19, 23, 29])
}