use rug::{Assign, Integer};

pub fn mod_pow_bigrug(mut base: Integer, mut exp: Integer, modulus: Integer) -> Integer {
    let mut result = Integer::from(1);
    base %= &modulus;
    while exp > Integer::from(0) {
        if &exp % 2 == Integer::from(1) {
            result = (result * &base) % &modulus;
        }
        exp >>= 1;
        base.assign(((&base) * &base) % &modulus);
    }
    result
}