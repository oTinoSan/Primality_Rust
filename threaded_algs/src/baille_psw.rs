use rug::ops::Pow;
use rug::{Complete, Integer};
use std::{fs::OpenOptions, io::{BufWriter, Write}, ops::Deref, sync::Mutex};

pub fn baillie_psw_array(limit: Integer) -> Vec<Integer> {
    let mut array: Vec<Integer> = Vec::new();
    let mut i = Integer::from(5);
    while i < limit {
        if baillie_psw_test(&i) {
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
        let mut thread_max: Integer = (i + 1) * Integer::from(&block_size) + &lower_limit + 5;

        if Integer::from(&thread_min) % 2 == Integer::ZERO {
            thread_min += 1;
        }

        if i == num_threads - 1 {
            thread_max = upper_limit.clone();
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
    // if return_vector.len() > 0 {
    //     println!("{:?}", return_vector);
    // }

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

    let result: bool = lucas_test(&n);

    return result;
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

pub fn lucas_test(n: &Integer) -> bool {
    // Lucas test: Find the first D in the sequence 5, −7, 9, −11, 13, −15, ...
    // for which the Jacobi symbol (D/n) is −1. Set P = 1 and Q = (1 − D) / 4.

    // if Integer::from(n % 3) == 0 || Integer::from(n % 5) == 0 || Integer::from(n % 7) == 0 {
    //     return false;
    // }
    let mut d: i32 = 0;
    let mut sign = true;

    for i in (5..100).step_by(2) {
        let result: i32;
        if sign {
            result = Integer::from(i).jacobi(&n);
        } else {
            result = Integer::from(-i).jacobi(&n);
        }
        if result == 0 {
            //then D and n have a prime factor in common, quit
            return false;
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
        // println!("got through 100 jacobi iterations without finding a D s.t. (D/n)== -1.
        // {} is probably a square number, use newtons method of detecting them", n);
        return false;
    }

    let p: Integer = Integer::from(1);
    let q = Integer::from((1 - d) / 4);
    // println!("n: {}, p: {}, q: {}, d: {}, (d/n): {}, gcd(n, 2pqd): {}", n, p, q, d, evaluate_jacobi(d, n.clone()), n.clone().gcd(&((2 * (&p * &q).complete()) * d)));

    // println!("(5/n): {}", evaluate_jacobi(5, n.clone()));
    // println!("(-7/n): {}", evaluate_jacobi(-7, n.clone()));
    // println!("(9/n): {}", evaluate_jacobi(9, n.clone()));
    // println!("(-11/n): {}", evaluate_jacobi(-11, n.clone()));

    let q_inv = modular_inverse(&q, &n);
    if q_inv == Integer::ZERO {
        // println!("there is no inverse for {} modulo {}", q, n);
        return false;
    }

    // println!("q is {}, q inverse is {}", q, q_inv);

    // Auxiliary parameters
    let a = Integer::from(p.clone().pow(2) * q_inv - 2)
        .pow_mod(&Integer::from(1), &n)
        .unwrap();
    let m: Integer = (n - evaluate_jacobi(d, n.clone())).complete() / 2;

    // lucas chain
    let (mut u, mut v) = (Integer::from(2), a.clone());

    // let mut x = 0;
    // let mut y = 1;
    for i in (0..=m.significant_bits()).rev() {
        if m.get_bit(i) {
            (u, v) = (
                (&u * &v - &a)
                    .complete()
                    .pow_mod(&Integer::from(1), &n)
                    .unwrap(),
                (&v * &v - Integer::from(2))
                    .pow_mod(&Integer::from(1), &n)
                    .unwrap(),
            );
            // x = 2*x+1;
            // y = 2*y;
        } else {
            (u, v) = (
                (&u * &u - Integer::from(2))
                    .pow_mod(&Integer::from(1), &n)
                    .unwrap(),
                (&u * &v - &a)
                    .complete()
                    .pow_mod(&Integer::from(1), &n)
                    .unwrap(),
            );
            // x = 2*x;
            // y = 2*y-1;
        }
        // println!("x: {}, y: {}, u: {}, v: {}",x,y,u,v);
    }

    if (a * u).pow_mod(&Integer::from(1), n).unwrap()
        == (Integer::from(2) * v)
            .pow_mod(&Integer::from(1), &n)
            .unwrap()
    {
        return true;
    }

    return false;
}

pub fn calculate_parameters(n: Integer) {
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
        println!(
            "got through 100 jacobi iterations without finding a D s.t. (D/n)== -1.
        {} is probably a square number, use newtons method of detecting them",
            n
        );
    }

    let p: Integer = Integer::from(1);
    let q = Integer::from((1 - d) / 4);
    println!("n: {}, p: {}, q: {}, d: {}", n, p, q, d);
}

fn evaluate_jacobi(d: i32, prime_candidate: Integer) -> i32 {
    let mut a = Integer::from(d);
    let mut n = prime_candidate.clone();
    a = a.pow_mod(&Integer::from(1), &n).unwrap(); // step 1
    let mut result = 1;
    let mut r: Integer;

    //step 3
    while a != Integer::from(0) {
        //step 2
        while Integer::from(
            a.clone()
                .pow_mod(&Integer::from(1), &Integer::from(2))
                .unwrap(),
        ) == Integer::from(0)
        {
            a /= 2;
            r = Integer::from(
                n.clone()
                    .pow_mod(&Integer::from(1), &Integer::from(8))
                    .unwrap(),
            );
            if r == Integer::from(3) || r == Integer::from(5) {
                result = -result;
            }
        }

        //step 4
        r = n;
        n = a;
        a = r;
        if Integer::from(
            a.clone()
                .pow_mod(&Integer::from(1), &Integer::from(4))
                .unwrap(),
        ) == Integer::from(3)
            && Integer::from(
                n.clone()
                    .pow_mod(&Integer::from(1), &Integer::from(4))
                    .unwrap(),
            ) == Integer::from(3)
        {
            result = -result;
        }
        a = a.pow_mod(&Integer::from(1), &n).unwrap();
    }
    if n != Integer::from(1) {
        result = 0;
    }
    // println!("n: {}, d: {}, result: {}", prime_candidate, d, result);
    return result;
}

fn modular_inverse(input: &Integer, modulus: &Integer) -> Integer {
    let result = input.clone().pow_mod(&Integer::from(1), &modulus).unwrap();

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
