use ::rand::thread_rng;
use num::bigint::RandBigInt;
use num::traits::ConstZero;
use num::FromPrimitive;
use num_iter::{range, range_inclusive, range_step_inclusive};
use rug::ops::AssignRound;
// use rug::{integer::MiniInteger,
use rug::{float::Round, rand, Complete, Float, Integer};
use rug_polynomial::ModPoly;
use std::thread;
// use rug::{Assign, Integer};

pub mod wheel_algos;

fn modular_exponentiation(mut x: u64, mut a: u64, n: u64) -> u64 {
    let mut ans = 1;
    if a <= 0 {
        return 1;
    }
    loop {
        if a == 1 {
            return ans * x % n;
        }
        if a & 1 == 0 {
            x = (x * x) % n;
            a >>= 1;
            continue;
        } else {
            ans = (ans * x) % n;
            a -= 1;
        }
    }
}

pub fn mult_prime(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for num in 2..=limit {
        if prime_test(num) {
            primes.push(num);
        }
    }
    primes
}

fn prime_test(candidate: u32) -> bool {
    let sqrt = (candidate as f64).sqrt() as u32;
    let mut is_prime = true;
    for divisor in 2..=sqrt {
        if candidate % divisor == 0 {
            is_prime = false;
            break;
        }
    }
    // println!("{} is prime? {}", candidate, is_prime);
    is_prime
}

pub fn wheel_mult_prime(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for num in 2..=limit {
        if wheel_test(num) {
            primes.push(num);
        }
    }
    primes
}

fn wheel_test(candidate: u32) -> bool {
    let mut wheel = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let mut is_prime = true;
    // loop {
    for mut divisor in wheel {
        if candidate == divisor {
            break;
        } else if candidate % divisor == 0 {
            is_prime = false;
            break;
        }
        // divisor += 30;
    }
    //     if !is_prime | (candidate > wheel[1]) {
    //         break;
    //     }
    // }
    is_prime
}

pub fn sieve(limit: u32) -> Vec<bool> {
    let mut a: Vec<bool> = vec![true; (limit + 1) as usize];
    let sqrt = (limit as f64).sqrt() as u32;
    for index in 2..=sqrt.try_into().unwrap() {
        if a[index] {
            for num in (index * index..limit as usize).step_by(index as usize) {
                a[num] = false;
            }
        }
    }
    return a;
}

pub fn miller_list(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    // let sqrt = (limit as f64).sqrt() as u32;
    for num in (5..=limit).step_by(2) {
        if miller_test(num) {
            primes.push(num);
        }
    }
    primes
}

fn miller_test(candidate: u32) -> bool {
    // while n<=number
    let mut is_composite = true;
    let mut k = 0;
    let mut divisor = candidate - 1;
    let mut b;
    loop {
        if divisor % 2 == 0 {
            k += 1;
            divisor = divisor / 2;
        } else {
            break;
        }
    }
    for i in 0..50 {
        // let a = rand::thread_rng().gen_range(2, (candidate - 2));
        let a = 2;
        b = modular_exponentiation(a as u64, divisor as u64, candidate as u64) as u32;
        if b == 1 {
            is_composite = false;
            break;
        } else {
            for i in (0..k).step_by(2) {
                if b == (candidate - 1) {
                    is_composite = false;
                    break;
                }
                b = modular_exponentiation(b as u64, 2, candidate as u64) as u32;
            }
        }
    }
    return !is_composite;
}

#[derive(Debug, Clone)]
struct Jacobi {
    a: u64,
    n: u64,
    sign: bool,
}

impl Jacobi {
    fn new(a: u64, n: u64) -> Jacobi {
        Jacobi { a, n, sign: false }
    }

    fn mod_reduce(&mut self) {
        self.a = self.a % self.n;
    }

    fn remove_twos(&mut self) {
        let pow = self.a.trailing_zeros();
        self.a = self.a >> pow;
        let mod_8 = self.n % 8;
        if !(pow % 2 == 0 || mod_8 == 1 || mod_8 == 7) {
            self.sign = !self.sign;
        }
    }

    fn invert(&mut self) {
        if self.a % 4 == 3 && self.n % 4 == 3 {
            self.sign = !self.sign;
        }
        let temp = self.a;
        self.a = self.n;
        self.n = temp;
    }

    fn eval(&mut self) -> i32 {
        if self.a % 2 == 0 {
            self.remove_twos();
        }
        while self.a > 1 {
            self.invert();
            self.mod_reduce();
            if self.a == 0 {
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

fn solovay_strassen(num_tests: u64, candidate: u64) -> bool {
    for _ in 0..num_tests {
        // let a = thread_rng().gen_range(2, (candidate - 2));
        let a = 2;
        let mut jacobi = Jacobi::new(a, candidate);
        let jacobi_result = jacobi.eval();
        let mod_result = modular_exponentiation(a, candidate - 1, candidate);
        if mod_result == 0 {
            return false;
        }
        if (mod_result == 1 && jacobi_result == 1)
            || (mod_result == candidate - 1 && jacobi_result == -1)
        {
            return true;
        }
    }
    return false;
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


pub fn u64AKS(prime_candidate: u64) -> bool {
    let mut prime_int = Integer::from(prime_candidate.clone());
    let mut prime_float:Float = Float::with_val(31, 0);
    prime_float.assign_round(prime_int.clone(), Round::Nearest);
    let log2n = prime_float.clone().log2();
    //check if n is a perfect power
    if prime_int.clone().is_perfect_power() {
        return false;
    }
    let mut r = 0;
    //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
    for mut r in num_iter::range(Integer::from(0), prime_int.clone()) {
        r = Integer::from(r);
        for k in num_iter::range(Integer::from(0), r.clone()) {
            if prime_int.clone().pow_mod(&k, &r.clone()) == Ok(Integer::from(1)) {
                if log2n.clone() * log2n.clone() < r.clone() {
                    if prime_int.clone().gcd(&r.clone()) == Integer::from(1) {
                        break;
                    }
                    return false;
                }
            }
        }
    } // checking for all 2<=a>= min(r,prime_candidate-1) that a does not divide n
    if prime_int.clone() - Integer::from(0) < r.clone() {
        for a in num_iter::range(Integer::from(2), prime_int.clone()) {
            if prime_int.clone() % a == 0 {
                return false;
            }
        }
    } else {
        for a in num_iter::range_inclusive(Integer::from(2), Integer::from(r)) {
            if prime_int.clone() % a == 0 {
                return false;
            }
        }
    }
    // if n =< r output prime
    if prime_int.clone() <= r {
        return true;
    }
    // check finite ring
    let mut totient = Float::new(53);
    for i in num_iter::range(Integer::from(1), prime_int.clone()) {
        if prime_int.clone().gcd(&i) == Integer::from(1) {
            totient += 1;
        }
    }
    let roof = (totient.sqrt() * log2n).to_integer_round(Round::Down).unwrap();
    for a in num_iter::range(Integer::from(1), roof.0) {
        let mut polynomial: ModPoly = ModPoly::with_roots(vec![-a.clone()], &Integer::from(u64::MAX)); // X + a
        let p = polynomial.clone();
        for _ in num_iter::range_inclusive(Integer::from(1), prime_int.clone()) {
            polynomial.mul(&p); // (X + a)^n
        }

        // for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate) {
        //     poly_no_mod.polynomial_multiplication(p, prime_candidate); // (X + a)^n
        // }

        let mut eq_poly = ModPoly::from_int(Integer::from(u64::MAX), a.clone()); // X^n + a
        eq_poly.set_coefficient((prime_candidate + 1).try_into().unwrap(), &Integer::from(1));
        let mut mod_poly = ModPoly::from_int(Integer::from(u64::MAX), Integer::from(-1)); // X^r - 1
        mod_poly.set_coefficient(r + 1, &Integer::from(1));
        // if (X+a)^n ≠ X^n+a (mod X^r − 1,n), then output composite
        if polynomial.rem(&mod_poly) != eq_poly.rem(&mod_poly)
            || polynomial.rem(&ModPoly::from_int(Integer::from(u64::MAX), prime_int.clone()))
                != eq_poly.rem(&ModPoly::from_int(Integer::from(u64::MAX),prime_int.clone()))
        {
            return false;
        }
    }

    return true;
}
