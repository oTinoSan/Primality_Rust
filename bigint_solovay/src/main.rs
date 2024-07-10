use num::bigint::RandBigInt;
use num::traits::ConstZero;
use num::BigUint;
use num::FromPrimitive;
use ::rand::thread_rng;
// use std::env;
// use rug::{Assign, Integer, rand};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let number: BigUint = args[1].trim().parse().expect("Must be an integer");
    let test_number: BigUint = BigUint::from(274u32);
    // let solovay_primes = solovay_strassen_bigint(10, test_number);
    let solovay_primes = solovay_strassen_list_bigint(10, test_number);
    println!("Primes : {:?}", solovay_primes);
    // let primes = solovay_list(number);
    // println!("{:?}", primes);
}

#[derive(Debug, Clone)]
struct Jacobi {
    a: BigUint,
    n: BigUint,
    sign: bool,
}

impl Jacobi {
    fn new(a: BigUint, n: BigUint) -> Jacobi {
        Jacobi { a, n, sign: false }
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

fn solovay_strassen_bigint(num_tests: u64, candidate: BigUint) -> bool {
    let can_clone = candidate.clone();
    for _ in 0..num_tests {
        let can_minus_one = can_clone.clone() - BigUint::from_u64(1).unwrap();
        let big1 = BigUint::from_u64(1).unwrap();

        let rand_num = thread_rng().gen_biguint_range(&big1, &can_minus_one);
        let a = rand_num.clone();

        let mut jacobi = Jacobi::new(a, candidate.clone());
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

fn solovay_strassen_list_bigint(num_tests: u64, max_val: BigUint) -> Vec<BigUint>{
    let mut primes= vec![];

    // for i in range_step_inclusive{
    //     if solovay_strassen_bigint(num_tests, i){
    //         primes.push(i);
    //     }
    // }
    let mut idx = BigUint::from(5u64);
    while idx < max_val {
        if solovay_strassen_bigint(num_tests, idx.clone()) {
            primes.push(idx.clone());
        }
        idx= idx.clone() + BigUint::from(2u64);
    }

    primes
}

use rug::{integer::MiniInteger, rand, Complete, Integer};
use std::thread;

pub fn will_solovay_strassen(num_tests: u64, candidate: Integer) -> bool {
    let mut rand = rand::RandState::new();
    for _ in 0..num_tests {
        let test_base = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        let jacobi = test_base.jacobi(&candidate);
        let powmod_result = test_base
            .pow_mod(&(Integer::from(&candidate - 1) / 2), &candidate)
            .unwrap();
        if !((powmod_result == jacobi)
            || (powmod_result == Integer::from(&candidate - 1) && jacobi == -1))
        {
            return false;
        }
    }
    return true;
} // more accurate than my current code -- Will's code

#[test]
pub fn t_smallprimetest() {

    let mut count = 0;

    for input in 5..100_000_000 {
        //println!("input is: {}", input);
        // let is_prime = solovay_strassen_bigint(10u64, BigUint::from_u64(input).unwrap());
        let is_prime = will_solovay_strassen(10u64, Integer::from(input));
        if is_prime {
            count += 1;
        }
    }
    println!("number of primes in range: {}", count);
 
}
