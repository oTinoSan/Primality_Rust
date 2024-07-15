use rug::{Integer, Complete, rand};
use std::{thread};

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
        if !((mod_result == Ok(Integer::from(0)) && jacobi_result == Integer::from(0))
            || (mod_result == Ok(Integer::from(1)) && jacobi_result == 1)
            || (mod_result == Ok(candidate.clone() - 1) && jacobi_result == -1))
        {
            return false;
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

pub fn bigint_miller_rabin_list(num_tests: u64, max_val: Integer) -> Vec<Integer>{
    let mut primes= vec![];
    let mut i = Integer::from(5);

    while i <= max_val {
        if bigint_miller_rabin(num_tests, i.clone()){
            primes.push(i.clone());
        }
        i = i + 2;
    }

    primes
}