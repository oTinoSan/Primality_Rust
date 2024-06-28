use rand::{thread_rng, Rng};
use num::bigint::RandBigInt;
use num::traits::ConstZero;
use num::BigUint;
use num::FromPrimitive;
// use rug::{Assign, Integer};

fn modular_exponentiation(mut x: u64, mut a: u64, n: u64) -> u64 {

    let mut ans = 1;
    if a <= 0 {
        return 1;
    }
    loop {
        if a == 1 { return ans * x % n; }
        if a&1 == 0 { x = (x*x) % n; a>>=1; continue; }
        else { ans = (ans * x) % n; a-=1; }
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
    // while n<=number{
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
            let a = thread_rng().gen_range(2..=(candidate - 2));
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
    fn new(a:u64, n: u64) -> Jacobi {
        Jacobi {
            a,
            n,
            sign: false,
        }
    }

    fn mod_reduce(&mut self) {
        self.a = self.a % self.n;
    }

    fn remove_twos(&mut self) {
        let pow = self.a.trailing_zeros();
        self.a = self.a >> pow;
        let mod_8 = self.n % 8;
        if !(pow % 2 == 0 || mod_8 == 1 || mod_8 == 7){
            self.sign = !self.sign;
        }
    }

    fn invert(&mut self){
        if self.a % 4 == 3 && self.n % 4 == 3 {
            self.sign = !self.sign;
        }
        let temp = self.a;
        self.a = self.n;
        self.n = temp;
    }

    fn eval(&mut self) -> i32 {
        if self.a % 2 == 0{
            self.remove_twos();
        }
        while self.a > 1{
            self.invert();
            self.mod_reduce();
            if self.a == 0{
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
    for _ in 0..num_tests{
        let a = thread_rng().gen_range(2..=(candidate-2));
        let mut jacobi = Jacobi::new(a, candidate);
        let jacobi_result = jacobi.eval();
        let mod_result = modular_exponentiation(a, candidate -1, candidate);
        if mod_result == 0 {
            return false;
        }
        if (mod_result == 1 && jacobi_result == 1) || (mod_result == candidate -1 && jacobi_result == -1) {
            return true;
        }
    }
    return false;
    }

pub fn solovay_strassen_list(num_tests: u64, max_val: u64) -> Vec<u64>{
    let mut primes= vec![];

    for i in (5..=max_val).step_by(2){
        if solovay_strassen(num_tests, i){
            primes.push(i);
        }
    }

    primes
}

#[derive(Debug, Clone)]
struct bigJacobi {
    a: BigUint,
    n: BigUint,
    sign: bool,
}

impl bigJacobi {
    fn new(a: BigUint, n: BigUint) -> bigJacobi {
        bigJacobi { a, n, sign: false }
    }

    fn mod_reduce(&mut self) {
        self.a = &self.a % &self.n;
    }

    fn remove_twos(&mut self) {
        while self.a.clone() % 2 as u64 == BigUint::ZERO {
            self.a = self.a.clone() / 2 as u64;
            let mut mod_8 = &self.n % BigUint::from_u64(8).unwrap();
            if !(mod_8 == BigUint::from_u64(1 as u64).unwrap()
                || mod_8 == BigUint::from_u64(7 as u64).unwrap())
            {
                self.sign = !self.sign;
            }
        }
    }
    fn invert(&mut self) {
        if &self.a % BigUint::from_u64(4).unwrap() == BigUint::from_u64(3).unwrap()
            && &self.n % BigUint::from_u64(4).unwrap() == BigUint::from_u64(3).unwrap()
        {
            self.sign = !self.sign;
        }
        let temp = self.a.clone();
        self.a = self.n.clone();
        self.n = temp.clone();
    }

    fn eval(&mut self) -> i32 {
        if &self.a % BigUint::from_u64(2).unwrap() == BigUint::from_u64(0).unwrap() {
            self.remove_twos();
        }
        while *&self.a > BigUint::from_u64(1).unwrap() {
            self.invert();
            self.mod_reduce();
            if *&self.a == BigUint::from_u64(0).unwrap() {
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

pub fn solovay_strassen_bigint(num_tests: u64, candidate: BigUint) -> bool {
    let can_clone = candidate.clone();
    for _ in 0..num_tests {
        let can_minus_one = can_clone.clone() - BigUint::from_u64(1).unwrap();
        let big1 = BigUint::from_u64(1).unwrap();

        let rand_num = rand::thread_rng().gen_biguint_range(&big1, &can_minus_one);
        let a = rand_num.clone();

        let mut jacobi = bigJacobi::new(a, candidate.clone());
        let jacobi_result = jacobi.eval();
        let mod_result = rand_num.modpow(&can_minus_one, &candidate);
        // println!("Jacobi result {} Random Number {}", jacobi_result, rand_num);
        if mod_result == BigUint::ZERO {
            return false;
        }
        if (mod_result == BigUint::from_u64(1).unwrap() && jacobi_result == 1)
            || (mod_result == can_minus_one && jacobi_result == -1)
        {
            return true;
        }
    }
    return false;
}