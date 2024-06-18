use super::mod_exp;
use rand::prelude::*;

pub struct Jacobi {
    m: u64,
    n: u64,
    sign: bool,
}

impl Jacobi {
    pub fn new(m: u64, n: u64, sign: bool) -> Jacobi {
        Jacobi {
            m: m,
            n: n,
            sign: sign,
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

    pub fn eval(&mut self) -> bool {
        while self.m > 1 {
            self.invert();
            self.mod_reduce();
            self.remove_twos();
        }
        self.sign
    }
}

pub fn solovay_strassen(num_tests: u64, candidate: u64) -> bool {
    for _ in 0..num_tests {
        let a = thread_rng().gen_range(2..=(candidate - 2));
        let mut jacobi = Jacobi::new(a, candidate, true);
        let jacobi_result;
        if jacobi.eval() {
            jacobi_result = 1;
        } else {
            jacobi_result = candidate - 1;
        }
        if jacobi_result != mod_exp(a, (candidate - 1) / 2, candidate) {
            return false;
        }
    }

    return true;
}
