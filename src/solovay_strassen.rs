use super::mod_exp;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Jacobi {
    m: u64,
    n: u64,
    sign: bool,
}

impl Jacobi {
    pub fn new(m: u64, n: u64, sign: bool) -> Jacobi {
        Jacobi {
            m,
            n,
            sign,
        }
    }

    fn mod_reduce(&mut self) {
        self.m = self.m % self.n;
    }

    fn remove_twos(&mut self) {
        let pow = self.m.trailing_zeros();
        self.m = self.m >> pow;
        let mod_8 = self.n % 8;
        if !(pow % 2 == 0 || mod_8 == 1 || mod_8 == 7) {
            self.sign = !self.sign;
        }
    }

    fn invert(&mut self) {
        if self.m % 4 == 3 && self.n % 4 == 3 {
            self.sign = !self.sign;
        }
        let temp = self.m;
        self.m = self.n;
        self.n = temp;
    }

    pub fn eval(&mut self) -> i32 {
        if self.m % 2 == 0 {
            self.remove_twos();
        }
        while self.m > 1 {
            self.invert();
            self.mod_reduce();
            if self.m == 0 {
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

pub fn solovay_strassen(num_tests: u64, candidate: u64) -> bool {
    for _ in 0..num_tests {
        let a = thread_rng().gen_range(2..=(candidate - 2));
        let mut jacobi = Jacobi::new(a, candidate, false);
        let jacobi_result = jacobi.eval();
        let mod_result = mod_exp(a, (candidate - 1) / 2, candidate);
        if !((mod_result == 0 && jacobi_result == 0)
            || (mod_result == 1 && jacobi_result == 1)
            || (mod_result == candidate - 1 && jacobi_result == -1))
        {
            return false;
        }
    }

    return true;
}

pub fn solovay_strassen_list(num_tests: u64, max_val: u64) -> Vec<u64> {
    let mut primes = vec![];

    for i in (5..=max_val).step_by(2) {
        if solovay_strassen(num_tests, i) {
            primes.push(i);
        }
    }

    primes
}
