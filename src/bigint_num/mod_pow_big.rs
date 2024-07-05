use num::{BigInt, FromPrimitive};

pub fn mod_pow_bigint(mut base: BigInt, mut exp: BigInt, modulus: BigInt) -> BigInt {
    let mut result = BigInt::from_u64(1).unwrap();
    base %= &modulus;
    while exp > BigInt::from_u64(0).unwrap() {
        if &exp % &BigInt::from_u64(2).unwrap() == BigInt::from_u64(1).unwrap() {
            result = (result * &base) % &modulus;
        }
        exp >>= 1;
        base = (&base * &base) % &modulus;
    }
    result
}
