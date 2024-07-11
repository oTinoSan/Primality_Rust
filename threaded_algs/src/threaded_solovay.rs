use std::env;
use rug::{Integer, Complete, rand};

pub fn threaded_solovay_strassen(limit: Integer, num_threads: u64) -> Vec<Integer> {
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
                if bigint_solovay_strassen(10, thread_min.clone()) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        }));
    }
    let mut results = vec![];
    for handle in thread_handles {
        let mut thread_results = handle.join().unwrap();
        results.append(&mut thread_results);
    }

    results
}

#[derive(Debug, Clone)]
struct Jacobi {
    a: Integer,
    n: Integer,
    sign: bool,
}

impl Jacobi {
    fn new(a: Integer, n: Integer) -> Jacobi {
        Jacobi { a, n, sign: false }
    }

    fn mod_reduce(&mut self) {
        self.a = &self.a % &self.n;
    }

    fn remove_twos(&mut self) {
        while self.a.clone() % 2 as u64 == Integer::ZERO {
            self.a = self.a.clone() / 2 as u64;
            let mut mod_8 = &self.n % Integer::from_u64(8).unwrap();
            if !(mod_8 == Integer::from_u64(1 as u64).unwrap()
                || mod_8 == Integer::from_u64(7 as u64).unwrap())
            {
                self.sign = !self.sign;
            }
        }
    }
    fn invert(&mut self) {
        if &self.a % Integer::from_u64(4).unwrap() == Integer::from_u64(3).unwrap()
            && &self.n % Integer::from_u64(4).unwrap() == Integer::from_u64(3).unwrap()
        {
            self.sign = !self.sign;
        }
        let temp = self.a.clone();
        self.a = self.n.clone();
        self.n = temp.clone();
    }

    fn eval(&mut self) -> i32 {
        if &self.a % Integer::from(2).unwrap() == Integer::from_u64(0).unwrap() {
            self.remove_twos();
        }
        while *&self.a > Integer::from(1).unwrap() {
            self.invert();
            self.mod_reduce();
            if *&self.a == Integer::from(0).unwrap() {
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
        let mut jacobi = Jacobi::new(a, candidate.clone());
        let jacobi_result = jacobi.eval(a, &candidate);
        let mod_result = a.pow_mod(&(Integer::from(&candidate -1)/2), &candidate).unwrap();
        if mod_result == Integer::from(0) {
            return false;
        }
        if (mod_result == Integerjacobi_result) || (mod_result == Integer::from(&candidate -1) && jacobi_result == -1) {
            return true;
        }
    }
    return false;
    }

pub fn bigint_solovay_strassen_list(num_tests: u64, max_val: Integer) -> Vec<Integer>{
    let mut primes= vec![];
    let mut i = Integer::from(5);

    while i <= max_val {
        if bigint_solovay_strassen(num_tests, i.clone()){
            primes.push(i.clone());
        }
        i = i + 2;
    }

    primes
}