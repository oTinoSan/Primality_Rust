use num::{BigUint, FromPrimitive};

pub fn mod_pow_bigint(mut base: BigUint, mut exp: BigUint, modulus: BigUint) -> BigUint {
    let mut result = BigUint::from_u64(1).unwrap();
    base %= &modulus;
    while exp > BigUint::from_u64(0).unwrap() {
        if &exp % &BigUint::from_u64(2).unwrap() == BigUint::from_u64(1).unwrap() {
            result = (result * &base) % &modulus;
        }
        exp >>= 1;
        base = (&base * &base) % &modulus;
    }
    result
}