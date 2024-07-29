use rug::integer::MiniInteger;
use rug::ops::Pow;
use rug::{rand, Complete, Integer};
use std::thread;

pub fn threaded_solovay_strassen(num_threads: u64, limit: Integer) -> Vec<Integer> {
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
            let mod_8 = &self.n % Integer::from(8);
            if !(mod_8 == Integer::from(1 as u64) || mod_8 == Integer::from(7 as u64)) {
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
    for _ in 0..num_tests {
        let a = Integer::from(
            Integer::from(Integer::from(&candidate - 3).random_below_ref(&mut rand)) + 1,
        );
        let mut jacobi = bigJacobi::new(a.clone(), candidate.clone());
        let jacobi_result = jacobi.eval();
        let mod_result = a.pow_mod(&(Integer::from(&candidate - 1) / 2), &candidate);
        if !((mod_result == Ok(Integer::from(0)) && jacobi_result == Integer::from(0))
            || (mod_result == Ok(Integer::from(1)) && jacobi_result == 1)
            || (mod_result == Ok(candidate.clone() - 1) && jacobi_result == -1))
        {
            return false;
        }
    }
    return false;
}

pub fn bigint_solovay_strassen_list(num_tests: u64, max_val: Integer) -> Vec<Integer> {
    let mut primes = vec![];
    let mut i = Integer::from(5);

    while i <= max_val {
        if bigint_solovay_strassen(num_tests, i.clone()) {
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
            a = a
                .pow_mod(&MiniInteger::from(2).borrow(), &n)
                .unwrap();

        }
        if a != minus_one {
            return false;
        }
    }
    true
}

pub fn bigint_miller_rabin_list(num_tests: u64, max_val: Integer) -> Vec<Integer> {
    let mut primes = vec![];
    let mut i = Integer::from(5);

    while i <= max_val {
        if bigint_miller_rabin(num_tests, i.clone()) {
            primes.push(i.clone());
        }
        i = i + 2;
    }

    primes
}

pub fn baillie_psw_array(limit: Integer) -> Vec<Integer> {
    let mut array: Vec<Integer> = Vec::new();
    let mut i = Integer::from(5);
    while i < limit {
        if baillie_wagstaff_lucas_test(&i).0 {
            array.push(i.clone());
        }
        i += 2;
    }
    return array;
}

// pub fn baillie_psw_pseudoprime_test(prime_candidate: &Integer) -> bool {
//     // step 1: check if it is a base 2 strong probable prime
//     let base_2_strong_probable = base_2_strong_probable_prime_test(prime_candidate);
//     if !base_2_strong_probable {
//         return false;
//     }
//     // step 2: check if it is a lucas probable prime
//     let lucas_probable = lucas_test(prime_candidate);
//     // step 3: check if it is actually prime
//     let miller_rabin_primality = bigint_miller_rabin(prime_candidate, 10);

//     return true;
// }

pub fn threaded_baillie_psw(
    lower_limit: Integer,
    upper_limit: Integer,
    num_threads: u64,
) -> Vec<Integer> {
    let block_size = Integer::from(Integer::from(&upper_limit - &lower_limit) / num_threads);

    let mut thread_handles = Vec::new();

    for i in 0..num_threads {
        let mut thread_min: Integer = i * Integer::from(&block_size) + &lower_limit + 5;
        let thread_max: Integer = (i + 1) * Integer::from(&block_size) + &lower_limit + 5;

        if Integer::from(&thread_min) % 2 == Integer::ZERO {
            thread_min += 1;
        }
        let thread = std::thread::spawn(move || {
            let mut return_vector = Vec::new();
            while thread_min < thread_max {
                if baillie_psw_test(&thread_min) {
                    return_vector.push(thread_min.clone());
                }
                thread_min += 2;
            }
            return_vector
        });
        thread_handles.push(thread);
    }

    let mut return_vector: Vec<Integer> = Vec::new();
    // join all of the threads
    for handle in thread_handles {
        return_vector.append(&mut handle.join().unwrap());
    }
    if return_vector.len() > 0 {
        println!("{:?}", return_vector);
    }
    return_vector
}

pub fn baillie_psw_test(n: &Integer) -> bool {
    // Step 0: optionally perform trial division for small prime divisors p < 1000
    let small_primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];
    for i in small_primes {
        if Integer::from(n % i) == 0 {
            if i == *n {
                return true;
            } else {
                return false;
            }
        }
    }

    // Step 1: perform miller-rabin test with base 2
    if !base_2_strong_probable_prime_test(&n) {
        return false;
    }
    // Step 2: perfrom lucas test

    let result = crandall_pomerance_lucas_test(&n);

    return result.1;
}

pub fn base_2_strong_probable_prime_test(n: &Integer) -> bool {
    let s = (n - Integer::from(1)).find_one(0).unwrap();
    let d: Integer = Integer::from(n - 1) / (2u64.pow(s.try_into().unwrap()));

    // println!("prime_candidate: {}, s: {}, d: {}, 2^s*d + 1 = {}", prime_candidate, s, d, {2u32.pow(s)*d+1});
    let a: Integer = Integer::from(2);
    let mut x = a.pow_mod(&d, &n).unwrap();
    let mut y: Integer = Integer::from(0);
    for _ in 0..s {
        y = x.clone().pow_mod(&Integer::from(2), &n).unwrap();
        if y == Integer::from(1) && x != Integer::from(1) && x != Integer::from(n - 1) {
            return false;
        }
        x = y.clone();
    }
    if y != Integer::from(1) {
        return false;
    }
    return true;
}

pub fn crandall_pomerance_lucas_test(n: &Integer) -> (bool, bool) {
    // Lucas test: Find the first D in the sequence 5, −7, 9, −11, 13, −15, ...
    // for which the Jacobi symbol (D/n) is −1. Set P = 1 and Q = (1 − D) / 4.

    // if Integer::from(n % 3) == 0 || Integer::from(n % 5) == 0 || Integer::from(n % 7) == 0 {
    //     return false;
    // }
    let s = (n - Integer::from(1)).find_one(0).unwrap();
    let d: Integer = Integer::from(n - 1) / (2u64.pow(s.try_into().unwrap()));

    let mut lucas_probable = false;
    let mut strong_lucas_probable = false;
    let mut delta: i32 = 0;
    let mut sign = true;

    for i in (5..100).step_by(2) {
        let result: i32;
        if sign {
            result = Integer::from(i).jacobi(&n);
        } else {
            result = Integer::from(-i).jacobi(&n);
        }
        if result == 0 { //then D and n have a prime factor in common, quit
            return (false, false);
        }
        if result == -1 {
            if sign {
                delta = i;
            } else {
                delta = -i;
            }
            break;
        }
        sign = !sign;
    }
    if delta == 0 {
        // println!("got through 100 jacobi iterations without finding a D s.t. (D/n)== -1.
        // {} is probably a square number, use newtons method of detecting them", n);
        return (false, false);
    }

    let p: Integer = Integer::from(1);
    let q = Integer::from((1 - delta) / 4);
    // println!("n: {}, p: {}, q: {}, d: {}, (d/n): {}, gcd(n, 2pqd): {}", n, p, q, d, evaluate_jacobi(d, n.clone()), n.clone().gcd(&((2 * (&p * &q).complete()) * d)));

    // println!("(5/n): {}", evaluate_jacobi(5, n.clone()));
    // println!("(-7/n): {}", evaluate_jacobi(-7, n.clone()));
    // println!("(9/n): {}", evaluate_jacobi(9, n.clone()));
    // println!("(-11/n): {}", evaluate_jacobi(-11, n.clone()));


    let q_inv = modular_inverse(&q, &n);
    if q_inv == Integer::ZERO {
        // println!("there is no inverse for {} modulo {}", q, n);
        return (false, false);
    }

    // println!("q is {}, q inverse is {}", q, q_inv);

    // Auxiliary parameters
    let a = Integer::from(p.clone().pow(2) * q_inv - 2).modulo(&n);
    let m: Integer = (n - evaluate_jacobi(delta, n.clone())).complete() / 2;

    // lucas chain starts here
    let (mut u, mut v) = (Integer::from(2), a.clone());

    // if d = 1 and p = n, then v1 = vd is 0 (mod n), so n is a strong lprp
    if d == 1 && u.clone().modulo(n) == 0 {
        strong_lucas_probable = true;
    }

    // let mut x = 0;
    // let mut y = 1;
    for i in (0..=m.significant_bits()).rev() {
        if m.get_bit(i) {
            (u, v) = (
                (&u * &v - &a).complete().modulo(&n),
                (&v * &v - Integer::from(2)).modulo(&n),
            );
            // x = 2*x+1;
            // y = 2*y;
        } else {
            (u, v) = (
                (&u * &u - Integer::from(2)).modulo(&n),
                (&u * &v - &a).complete().modulo(&n),
            );
            // x = 2*x;
            // y = 2*y-1;
        }
        // println!("x: {}, y: {}, u: {}, v: {}",x,y,u,v);
        // check strong lucas conditions
        
        if i == n.significant_bits() - s { // 1: u(d) == 0
            if modular_inverse(&Integer::from(delta), n) * ((2*&v) - (&p * &u).complete()) == 0{
                strong_lucas_probable = true;
            }
        }
        if i >= n.significant_bits() - s && i < n.significant_bits() && u == 0 {
            strong_lucas_probable = true;
        }
    }

    if (a * u).modulo(n) == (Integer::from(2) * &v).modulo(&n) {
        lucas_probable = true;
    }

    return (lucas_probable, strong_lucas_probable);
}

pub fn baillie_wagstaff_lucas_test(n: &Integer) -> (bool, bool) {
    // Lucas test: Find the first D in the sequence 5, −7, 9, −11, 13, −15, ...
    // for which the Jacobi symbol (D/n) is −1. Set P = 1 and Q = (1 − D) / 4.

    // if Integer::from(n % 3) == 0 || Integer::from(n % 5) == 0 || Integer::from(n % 7) == 0 {
    //     return false;
    // }
    let mut lucas_probable = false;
    let mut strong_lucas_probable = false;
    let mut delta: i32 = 0;
    let mut sign = true;

    for i in (5..100).step_by(2) {
        let result: i32;
        if sign {
            result = Integer::from(i).jacobi(&n);
        } else {
            result = Integer::from(-i).jacobi(&n);
        }
        if result == 0 { //then D and n have a prime factor in common, quit
            return (false, false);
        }
        if result == -1 {
            if sign {
                delta = i;
            } else {
                delta = -i;
            }
            break;
        }
        sign = !sign;
    }
    if delta == 0 {
        // println!("got through 100 jacobi iterations without finding a D s.t. (D/n)== -1.
        // {} is probably a square number, use newtons method of detecting them", n);
        return (false, false);
    }

    let p: Integer = Integer::from(1);
    let q = Integer::from((1 - delta) / 4);
    // println!("n: {}, p: {}, q: {}, d: {}, (d/n): {}, gcd(n, 2pqd): {}", n, p, q, delta, evaluate_jacobi(delta, n.clone()), n.clone().gcd(&((2 * (&p * &q).complete()) * delta)));

    // println!("(5/n): {}", evaluate_jacobi(5, n.clone()));
    // println!("(-7/n): {}", evaluate_jacobi(-7, n.clone()));
    // println!("(9/n): {}", evaluate_jacobi(9, n.clone()));
    // println!("(-11/n): {}", evaluate_jacobi(-11, n.clone()));
    let n_plus_one = Integer::from(n+1);

    let s = n_plus_one.find_one(0).unwrap();
    let d: Integer = n_plus_one.clone() / (2u64.pow(s.try_into().unwrap()));
    // println!("s: {}, d: {}", s, d);
    
    let n_bits = n_plus_one.significant_bits();
    // let mut string = String::new();
    // for i in 0..n_bits {
    //     string += &n_plus_one.get_bit(n_bits-i-1).to_string();
    // }
    // println!("n_bits: {}, {}",n_bits, string);

    //initialize u1, v1, q^1
    let mut u_k: Integer = Integer::from(1);
    let mut v_k: Integer = p.clone();
    let mut q_k: Integer = q.clone();

    // if d = 1 and if p = n, then v1 = v(d) is 0 (mod n) , so n is already a strong lprp
    if d == 1 && v_k.clone().modulo(n) == 0 {
        strong_lucas_probable = true;
    }
    // let mut x = 1;
    for i in 2..=n_bits {
        (u_k, v_k, q_k) = ((&u_k * &v_k).complete().modulo(n), ((&v_k * &v_k).complete() -(&q_k* &Integer::from(2)).complete()).modulo(n), q_k.pow(2).modulo(n));
        // x *= 2;
        if n_plus_one.get_bit(n_bits-i) {
            let mut u_temp = (&p * &u_k + &v_k).complete();
            if !(u_temp.is_even()) { // make it even
                u_temp += n;
            }
            let mut v_temp = (&delta * &u_k).complete() + &p * &v_k;
            if !(v_temp.is_even()) {
                v_temp += n;
            }
            (u_k, v_k, q_k) = ((u_temp / Integer::from(2)).modulo(n),(v_temp / Integer::from(2)).modulo(n),(q_k * &q).modulo(n));
            // x += 1;
        }
        if i == n_bits - s && u_k == 0 {
            // println!("condition one");
            strong_lucas_probable = true;
        }
        if i >= n_bits - s && i < n_bits && v_k == 0 {
            // println!("condition two");
            strong_lucas_probable = true;
        }
        // println!("bit: {}, x: {}, u: {}, v: {}",n_plus_one.get_bit(n_bits-i),x,u_k,v_k);
    }
    if u_k == 0 {
        lucas_probable = true;
    }
    return (lucas_probable, strong_lucas_probable);
}

pub fn calculate_parameters (n: Integer) {
    let mut d: i32 = 0;
    let mut sign = true;

    for i in (5..100).step_by(2) {
        let result: i32;
        if sign {
            result = evaluate_jacobi(i, n.clone());
        } else {
            result = evaluate_jacobi(-i, n.clone());
        }
        if result == -1 {
            if sign {
                d = i;
            } else {
                d = -i;
            }
            break;
        }
        sign = !sign;
    }
    if d == 0 {
        println!("got through 100 jacobi iterations without finding a D s.t. (D/n)== -1.
        {} is probably a square number, use newtons method of detecting them", n);
    }

    let p: Integer = Integer::from(1);
    let q = Integer::from((1 - d) / 4);
    println!("n: {}, p: {}, q: {}, d: {}", n, p, q, d);
}

fn evaluate_jacobi(d: i32, prime_candidate: Integer) -> i32 {
    let mut a = Integer::from(d);
    let mut n = prime_candidate.clone();
    a = a.modulo(&n); // step 1
    let mut result = 1;
    let mut r: Integer;

    //step 3
    while a != Integer::from(0) {
        //step 2
        while Integer::from(a.clone().modulo(&Integer::from(2))) == Integer::from(0) {
            a /= 2;
            r = Integer::from(n.clone().modulo(&Integer::from(8)));
            if r == Integer::from(3) || r == Integer::from(5) {
                result = -result;
            }
        }

        //step 4
        r = n;
        n = a;
        a = r;
        if Integer::from(a.clone().modulo(&Integer::from(4))) == Integer::from(3)
            && Integer::from(n.clone().modulo(&Integer::from(4))) == Integer::from(3)
        {
            result = -result;
        }
        a = a.modulo(&n);
    }
    if n != Integer::from(1) {
        result = 0;
    }
    // println!("n: {}, d: {}, result: {}", prime_candidate, d, result);
    return result;
}

fn modular_inverse(input: &Integer, modulus: &Integer) -> Integer {
    let result = input.clone().modulo(&modulus);

    let y = extended_gcd(&modulus, &result);

    if y.2 == 1 {
        if y.1 < Integer::ZERO {
            let r = modulus + y.1;
            return r;
        } else {
            return y.1;
        }
    }

    // panic!("there is no inverse for {} modulo {}", input, modulus);
    return Integer::ZERO;
}

fn extended_gcd(x: &Integer, y: &Integer) -> (Integer, Integer, Integer) {
    let (mut a, mut b, mut g, mut u, mut v, mut w) = (
        Integer::from(1),
        Integer::from(0),
        x.clone(),
        Integer::from(0),
        Integer::from(1),
        y.clone(),
    );
    while w.clone() > Integer::ZERO {
        let q = Integer::from(&g / &w);
        (a, b, g, u, v, w) = (
            u.clone(),
            v.clone(),
            w.clone(),
            a - &q * &u,
            b - &q * &v,
            g - &q * &w,
        );
    }

    return (a, b, g);
}

use num_iter::{range, range_inclusive};
use rug::ops::AssignRound;
use rug::{float::Round, Float};
use std::collections::HashMap;

// pub fn u64AKS(prime_candidate: u64) -> bool {
//     let prime_int = Integer::from(prime_candidate.clone());
//     let mut prime_float: Float = Float::with_val(31, 0);
//     prime_float.assign_round(prime_int.clone(), Round::Nearest);
//     let log2n = prime_float.clone().log2();
//     //check if n is a perfect power
//     if prime_int.clone().is_perfect_power() {
//         return false;
//     }
//     let r = 0;
//     //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
//     for mut r in num_iter::range(Integer::from(0), prime_int.clone()) {
//         r = Integer::from(r);
//         for k in num_iter::range(Integer::from(0), r.clone()) {
//             if prime_int.clone().pow_mod(&k, &r.clone()) == Ok(Integer::from(1)) {
//                 if log2n.clone() * log2n.clone() < r.clone() {
//                     if prime_int.clone().gcd(&r.clone()) == Integer::from(1) {
//                         break;
//                     }
//                     return false;
//                 }
//             }
//         }
//     } // checking for all 2<=a>= min(r,prime_candidate-1) that a does not divide n
//     if prime_int.clone() - Integer::from(0) < r.clone() {
//         for a in num_iter::range(Integer::from(2), prime_int.clone()) {
//             if prime_int.clone() % a == 0 {
//                 return false;
//             }
//         }
//     } else {
//         for a in num_iter::range_inclusive(Integer::from(2), Integer::from(r)) {
//             if prime_int.clone() % a == 0 {
//                 return false;
//             }
//         }
//     }
//     // if n =< r output prime
//     if prime_int.clone() <= r {
//         return true;
//     }
//     // check finite ring
//     let mut totient = Float::new(53);
//     for i in num_iter::range(Integer::from(1), prime_int.clone()) {
//         if prime_int.clone().gcd(&i) == Integer::from(1) {
//             totient += 1;
//         }
//     }
//     let roof = (totient.sqrt() * log2n)
//         .to_integer_round(Round::Down)
//         .unwrap();
//     for a in num_iter::range(Integer::from(1), roof.0) {
//         let mut polynomial: ModPoly =
//             ModPoly::with_roots(vec![-a.clone()], &Integer::from(u64::MAX)); // X + a
//         let p = polynomial.clone();
//         for _ in num_iter::range_inclusive(Integer::from(1), prime_int.clone()) {
//             polynomial.mul(&p); // (X + a)^n
//         }

//         let mut eq_poly = ModPoly::from_int(Integer::from(u64::MAX), a.clone()); // X^n + a
//         eq_poly.set_coefficient((prime_candidate + 1).try_into().unwrap(), &Integer::from(1));
//         let mut mod_poly = ModPoly::from_int(Integer::from(u64::MAX), Integer::from(-1)); // X^r - 1
//         mod_poly.set_coefficient(r + 1, &Integer::from(1));
//         // if (X+a)^n ≠ X^n+a (mod X^r − 1,n), then output composite
//         if polynomial.rem(&mod_poly) != eq_poly.rem(&mod_poly)
//             || polynomial.rem(&ModPoly::from_int(
//                 Integer::from(u64::MAX),
//                 prime_int.clone(),
//             )) != eq_poly.rem(&ModPoly::from_int(
//                 Integer::from(u64::MAX),
//                 prime_int.clone(),
//             ))
//         {
//             return false;
//         }
//     }

//     return true;
// }
pub fn BigIntAKS_list(limit: Integer) -> Vec<Integer>{
    let mut array: Vec<Integer> = Vec::new();
    let mut i = Integer::from(5);
    while i < limit {
        if BigIntAKS(i.clone()) {
            array.push(i.clone());
        }
        i += 2;
    }
    return array;
}

pub fn BigIntAKS(prime_candidate: Integer) -> bool {
    //check if n is a perfect power
    if prime_candidate.is_perfect_power() {
        return false;
    }
    let mut prime_float = Float::new(prime_candidate.clone().significant_bits());
    prime_float.assign_round(prime_candidate.clone(), Round::Nearest);
    let log2n = prime_float.clone().log2();
    let mut r_true = Integer::from(0);
    //find smallest r such that the multiplicative order of prime modulo r is greater than (log2n)^2
    for mut r in num_iter::range(Integer::from(0), prime_candidate.clone()) {
        r_true = r.clone();
        for k in num_iter::range(Integer::from(0), r.clone()) {
            if prime_candidate.clone().pow_mod(&k, &r.clone()) == Ok(Integer::from(1)) {
                if log2n.clone() * log2n.clone() < r.clone() {
                    if prime_candidate.clone().gcd(&r.clone()) == Integer::from(1) {
                        break;
                    }
                    return false;
                }
            }
        }
    } // checking for all 2<=a>= min(r,prime_candidate-1) that a does not divide n
    if prime_candidate.clone() - Integer::from(0) < r_true.clone() {
        for a in num_iter::range_inclusive(Integer::from(2), prime_candidate.clone()) {
            if prime_candidate.clone() % a == 0 {
                return false;
            }
        }
    } else {
        for a in num_iter::range_inclusive(Integer::from(2), r_true.clone()) {
            if prime_candidate.clone() % a == 0 {
                return false;
            }
        }
    }
    // if n =< r output prime
    if prime_candidate <= r_true {
        return true;
    }
    // check finite ring
    let mut totient = Float::new(53);
    for i in num_iter::range(Integer::from(1), prime_candidate.clone()) {
        if prime_candidate.clone().gcd(&i) == Integer::from(1) {
            totient += 1;
        }
    }
    let roof = (totient.sqrt() * log2n)
        .to_integer_round(Round::Down)
        .unwrap();
    for a in num_iter::range_inclusive(Integer::from(1), roof.0) {
        let mut polynomial: Polynomial = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            Integer::from(1),
        );
        polynomial.set_polynomial_coef(Integer::from(0), a.clone());
        polynomial.set_polynomial_coef(Integer::from(1), Integer::from(1)); // X + a

        let mut p = polynomial.polynomial_clone();
        for _ in num_iter::range_inclusive(Integer::from(1), prime_candidate.clone() - 1) {
            polynomial = polynomial.polynomial_multiplication(&p);
            // (X + a)^n not modular // working
        }
        let mut eq_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            prime_candidate.clone(),
        ); // X^n + a
        eq_poly.set_polynomial_coef(Integer::from(0), a);
        eq_poly.set_polynomial_coef(prime_candidate.clone(), Integer::from(1));
        let mut mod_poly = Polynomial::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            r_true.clone(),
        ); // X^r - 1
        mod_poly.set_polynomial_coef(Integer::from(0), Integer::from(-1));
        mod_poly.set_polynomial_coef(r_true.clone(), Integer::from(1)); // X^r -1
        if !(polynomial
            .clone()
            .polynomial_remainder(&eq_poly, &mod_poly, prime_candidate.clone()))
        {
            return false;
        }
    }
    return true;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    coef: HashMap<Integer, HashMap<Integer, Integer>>,
    deg: Integer,
}

impl Polynomial {
    pub fn initialize_polynomial(
        coef: HashMap<Integer, HashMap<Integer, Integer>>,
        deg: Integer,
    ) -> Polynomial {
        Polynomial { coef, deg }
    }

    pub fn polynomial_clone(&self) -> Polynomial {
        Polynomial {
            coef: self.coef.clone(),
            deg: self.deg.clone(),
        }
    }
    // pub fn is_equal_polynomial(&self, poly_2: &Polynomial) -> bool {
    //     println!("Figuring out if polynomials are equal");
    //     if self.deg != poly_2.deg {
    //         println!("uh oh: {}, {}", self.deg, poly_2.deg);
    //         return false;
    //     }
    //     for i in range_inclusive(Integer::from(0), self.deg.clone()) {
    //         if self.get_polynomial_coef(i.clone()) != poly_2.get_polynomial_coef(i.clone()) {
    //             println!(
    //                 "bang: {}, {}",
    //                 self.get_polynomial_coef(i.clone()),
    //                 poly_2.get_polynomial_coef(i.clone())
    //             );
    //             return false;
    //         }
    //         println!("the polynomials' values at key {} are equal", &i);
    //     }
    //     return true;
    // }

    pub fn set_polynomial_coef(&mut self, order: Integer, coefficient: Integer) {
        if order <= Integer::from(&self.deg) {
            let a: &mut HashMap<Integer, Integer> = self
                .coef
                .get_mut(&(order.clone() / Integer::from(2).pow(64) + 1))
                .unwrap();
            a.insert(order.clone(), coefficient);
            return;
        } else {
            for index in range(Integer::from(self.deg.clone()), order.clone()) {
                if index > Integer::from(2).pow(64) * (self.coef.len() + 1) {
                    self.coef
                        .insert((self.coef.len() + 1).into(), HashMap::new());
                }
                let a: &mut HashMap<Integer, Integer> = self
                    .coef
                    .get_mut(&(index.clone() / Integer::from(2).pow(64) + 1))
                    .unwrap();
                a.insert(index, Integer::from(0));
            }
            let a: &mut HashMap<Integer, Integer> = self
                .coef
                .get_mut(&(order.clone() / Integer::from(2).pow(64) + 1))
                .unwrap();
            a.insert(order.clone(), coefficient);
            self.deg = order;
        }
    }
    pub fn get_polynomial_coef(&self, order: Integer) -> Integer {
        if order > self.deg {
            return Integer::from(0);
        }
        let inner_hash = self
            .coef
            .get(&Integer::from(&(&order / Integer::from(2).pow(64)) + 1))
            .unwrap();
        if inner_hash.contains_key(&order.clone()) {
            return inner_hash.get(&order).unwrap().clone();
        }
        return Integer::from(0);
    }

    pub fn polynomial_modular_multiplication(
        &self,
        poly_2: &Polynomial,
        mod_n: Integer,
    ) -> Polynomial {
        let max_deg = self.deg.clone() + poly_2.deg.clone();
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            max_deg.clone(),
        );
        for i in range_inclusive(
            Integer::from(1),
            Integer::from(((max_deg.clone() + 1) / (Integer::from(2).pow(64))) + 1),
        ) {
            poly_res.coef.insert(i, HashMap::new());
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            for j in range_inclusive(Integer::from(0), poly_2.deg.clone()) {
                let mut coef = self.get_polynomial_coef(i.clone())
                    * poly_2.get_polynomial_coef(j.clone())
                    + poly_res.get_polynomial_coef(i.clone() + j.clone());
                coef = coef % mod_n.clone();
                poly_res.set_polynomial_coef(i.clone() + j, coef);
            }
        }
        return poly_res;
    }
    pub fn polynomial_modular_power(&self, exp: Integer) -> Polynomial {
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            Integer::from(0),
        );
        poly_res.set_polynomial_coef(Integer::from(0), Integer::from(1));
        for i in range_inclusive(exp.significant_bits() + 1, 1).rev() {
            poly_res = self.polynomial_modular_multiplication(&poly_res, exp.clone());
            if exp.get_bit(i - 1) {
                poly_res.polynomial_modular_multiplication(&self, exp.clone());
            }
        }
        return poly_res;
    }
    pub fn polynomial_remainder(
        &mut self,
        poly_2: &Polynomial,
        mod_poly: &Polynomial,
        mod_n: Integer,
    ) -> bool {
        for i in range_inclusive(Integer::from(0), Integer::from(self.deg.clone())) {
            self.set_polynomial_coef(
                i.clone(),
                self.get_polynomial_coef(i.clone()) - poly_2.get_polynomial_coef(i.clone()),
            );
        }
        loop {
            for i in range_inclusive(Integer::from(1), self.deg.clone()).rev() {
                if self.get_polynomial_coef(i.clone()) == Integer::from(0) {
                    self.deg = self.deg.clone() - 1;
                } else {
                    break;
                }
            }
            let mut multiple_poly_degree = Integer::from(-1);
            if self.deg < mod_poly.deg {
                break;
            }
            for i in
                range_inclusive(self.deg.clone() - mod_poly.deg.clone(), self.deg.clone()).rev()
            {
                if multiple_poly_degree == !Integer::from(-1) {
                    break;
                }
                if self.get_polynomial_coef(i.clone())
                    > mod_poly.get_polynomial_coef(mod_poly.deg.clone())
                {
                    multiple_poly_degree = i - mod_poly.deg.clone();
                    break;
                } else if self.get_polynomial_coef(i.clone())
                    == mod_poly.get_polynomial_coef(mod_poly.deg.clone())
                {
                    for j in range_inclusive(self.deg.clone() - mod_poly.deg.clone(), i.clone() - 1)
                        .rev()
                    {
                        if self.get_polynomial_coef(j.clone())
                            > poly_2.get_polynomial_coef(mod_poly.deg.clone() - j.clone())
                        {
                            multiple_poly_degree = j - mod_poly.deg.clone();
                            break;
                        }
                    }
                }
            }
            if multiple_poly_degree <= Integer::from(-1) {
                break;
            }
            let mut multiple_poly = Self::initialize_polynomial(
                HashMap::from([(Integer::from(1), HashMap::new())]),
                multiple_poly_degree.clone(),
            );

            multiple_poly.set_polynomial_coef(
                multiple_poly_degree,
                self.get_polynomial_coef(self.deg.clone())
                    / mod_poly.get_polynomial_coef(mod_poly.deg.clone()),
            );
            let mut long_division_poly = mod_poly.polynomial_multiplication(&multiple_poly);
            for i in range_inclusive(
                Integer::from(long_division_poly.deg.clone()),
                Integer::from(0),
            )
            .rev()
            {
                if long_division_poly.get_polynomial_coef(long_division_poly.deg.clone())
                    == Integer::from(0)
                {
                    long_division_poly.deg = long_division_poly.deg.clone() - 1;
                } else {
                    break;
                }
            }
            if long_division_poly.deg < Integer::from(0) {
                break;
            }
            for i in range_inclusive(Integer::from(0), Integer::from(self.deg.clone())) {
                self.set_polynomial_coef(
                    i.clone(),
                    self.get_polynomial_coef(i.clone())
                        - long_division_poly.get_polynomial_coef(i.clone()),
                );
            }
            for i in range_inclusive(Integer::from(1), self.deg.clone()).rev() {
                if self.get_polynomial_coef(i.clone()) == Integer::from(0) {
                    self.deg = self.deg.clone() - 1;
                } else {
                    break;
                }
            }
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            if !(self.get_polynomial_coef(i.clone()).is_divisible(&mod_n)) {
                return false;
            }
        }
        return true;
    }
    pub fn polynomial_multiplication(&self, poly_2: &Polynomial) -> Polynomial {
        let max_deg = self.deg.clone() + poly_2.deg.clone();
        let mut poly_res = Self::initialize_polynomial(
            HashMap::from([(Integer::from(1), HashMap::new())]),
            max_deg.clone(),
        );
        for i in range_inclusive(
            Integer::from(1),
            Integer::from(((max_deg.clone() + 1) / (Integer::from(2).pow(64))) + 1),
        ) {
            poly_res.coef.insert(i, HashMap::new());
        }
        for i in range_inclusive(Integer::from(0), self.deg.clone()) {
            for j in range_inclusive(Integer::from(0), poly_2.deg.clone()) {
                let coef = self.get_polynomial_coef(i.clone())
                    * poly_2.get_polynomial_coef(j.clone())
                    + poly_res.get_polynomial_coef(i.clone() + j.clone());
                poly_res.set_polynomial_coef(i.clone() + j, coef);
            }
        }
        return poly_res;
    }
}

pub fn threaded_AKS_prime(num_threads: u64, limit: Integer) -> Vec<Integer> {
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
                if BigIntAKS(thread_min.clone()) {
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
