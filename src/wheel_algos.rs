use rayon::prelude::*;
use rug::{Integer, Complete, rand};
use std::{thread};
use std::sync::Arc;

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
#[derive(Debug, Clone)]
struct bigJacobi {
    a: Integer,
    n: Integer,
    sign: bool,
}

impl bigJacobi {
    fn new(a: Integer, n: Integer) -> bigJacobi {
        bigJacobi { a, n, sign: false }
    }

    fn mod_reduce(&mut self) {
        self.a = Integer::from(&self.a % &self.n);
    }

    fn remove_twos(&mut self) {
        while self.a.clone() % 2 as u64 == Integer::ZERO {
            self.a = self.a.clone() / 2 as u64;
            let mut mod_8 = &self.n % Integer::from(8);
            if !(mod_8 == Integer::from(1 as u64)
                || mod_8 == Integer::from(7 as u64))
            {
                self.sign = !self.sign;
            }
        }
    }
    fn invert(&mut self) {
        if &self.a % Integer::from(4) == Integer::from(3)
            && &self.n % Integer::from(4) == Integer::from(3)
        {
            self.sign = !self.sign;
        }
        let temp = self.a.clone();
        self.a = self.n.clone();
        self.n = temp.clone();
    }

    fn eval(&mut self) -> i32 {
        if &self.a % Integer::from(2) == Integer::from(0) {
            self.remove_twos();
        }
        while *&self.a > Integer::from(1) {
            self.invert();
            self.mod_reduce();
            if *&self.a == Integer::from(0) {
                return 0;
            }
            self.remove_twos();
        }
        if self.sign {
            return -1;
        } else {
            return 1;
        }
    }
}

pub fn bigint_solovay_strassen(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    for _ in 0..num_tests{
        let a = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        let mut jacobi = bigJacobi::new(a.clone(), candidate.clone());
        let jacobi_result = jacobi.eval();
        let mod_result = a.pow_mod(&(Integer::from(&candidate -1)/2), &candidate);
        if mod_result == Ok(Integer::from(0)) {
            return false;
        }
        if (mod_result == Ok(Integer::from(jacobi_result))) || (mod_result == Ok(Integer::from(&candidate -1)) && jacobi_result == -1) {
            return true;
        }
    }
    return false;
    }

pub fn threaded_miller_rabin(limit: Integer, num_threads: u64) -> Vec<Integer> {
    let block_size = (&limit / num_threads).complete();

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: Integer = (i * &block_size).complete() + 5;
        let thread_max: Integer = ((i + 1) * &block_size).complete() + 5;

        if Integer::from(&thread_min % 2) == 0 {
            thread_min += 1;
        }
        thread_handles.push(std::thread::spawn(move || {
            let mut return_vector = Vec::new();
            while thread_min < thread_max {
                if bigint_miller_rabin(10, thread_min.clone()) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        }));
    }
    let mut results = vec![];
    for handle in thread_handles {
        let mut thread_results = handle.join();
        results.append(&mut thread_results.unwrap());
    }

    results
}

pub fn bigint_miller_rabin(loop_amount: u64, n: Integer) -> bool {
    let mut rand = rand::RandState::new();
    let minus_one = Integer::from(&n - 1);
    let s = minus_one.find_one(0).unwrap();
    let d = Integer::from(&minus_one >> s);
    'outer: for _ in 0..loop_amount {
        let mut a =
            Integer::from(Integer::from(Integer::from(&n - 3).random_below_ref(&mut rand)) + 1);
        a = a.pow_mod(&d, &n).unwrap();
        if a == 1 {
            continue;
        }
        for _ in 0..s {
            if a == Integer::from(&n - 1) {
                continue 'outer;
            }
            // a = a.pow_mod(&MiniInteger::from(2).borrow(), &n).from;
        }
        if a != minus_one {
            return false;
        }
    }
    true
}