use num_prime::nt_funcs::factorize128;
use rug::Integer;



pub mod search_e;
pub mod search_s;

pub fn sieve_of_eratosthenes(int: u128) -> Vec<bool> {
    let mut array: Vec<bool> = vec![true; (int + 1) as usize];

    let mut i: usize = 2;
    array[0] = false;
    array[1] = false;
    while i <= ((f64::sqrt(int as f64)) as u128 + 1) as usize {
        if array[i] {
            for n in (i * i..int as usize).step_by(i) {
                array[n] = false;
            }
        }
        i = i + 1;
    }
    array[int as usize] = false;
    return array;
}



pub fn calculate_order_prime (p: u128) -> u128 {
    let t = p-1;

    let mut qs = vec![1];
    let factors = factorize128(t);

    for (prime,exponent) in factors {
        let mut qs_new: Vec<u128> = Vec::new();

        for q in qs { // make power set of prime factors
            for j in 0..=exponent {
                qs_new.push(q * prime.pow(j.try_into().unwrap()))
            }
            println!("{:?}", qs_new);
        }
        qs = qs_new;
        println!("qs: {:?}", qs);
    }

    qs.sort();
    println!("final qs: {:?}", qs);

    for q in qs.clone() {
        if Integer::from(2).pow_mod(&Integer::from(q), &Integer::from(p)).unwrap() == 1{
            return q;
        }
    }
    return qs.pop().unwrap();
}

pub fn modular_inverse(input: i128, modulus: i128) -> i128 {
    let result = input % modulus;

    let y = extended_gcd(modulus, result);

    if y.2 == 1 {
        if y.1 < Integer::ZERO {
            let r = modulus + y.1;
            return r;
        } else {
            return y.1;
        }
    }

    // panic!("there is no inverse for {} modulo {}", input, modulus);
    return 0;
}

fn extended_gcd(x: i128, y: i128) -> (i128, i128, i128) {
    let (mut a, mut b, mut g, mut u, mut v, mut w) = (
        1,
        0,
        x,
        0,
        1,
        y,
    );
    while w.clone() > Integer::ZERO {
        let q = g / w;
        (a, b, g, u, v, w) = (
            u.clone(),
            v.clone(),
            w.clone(),
            a - q * u,
            b - q * v,
            g - q * w,
        );
    }

    return (a, b, g);
}

pub fn fermat_primality (input: u128) -> bool{

    Integer::from(2).pow_mod(&Integer::from(input-1), &Integer::from(input)).unwrap() == 1

}