use num::BigUint;
use num::FromPrimitive;
use num::bigint::RandBigInt;
use super::mod_pow_big::mod_pow_bigint;

pub fn miller_rabin_bignum(candidate: BigUint, iterations: usize) -> bool {
    if candidate == BigUint::from(2u8) || candidate == BigUint::from(3u8) {
        return true;
    }
    if &candidate % &BigUint::from(2u8) == BigUint::from(0u8) || candidate == BigUint::from(1u8) {
        return false;
    }

    let mut rng = rand::thread_rng();
    let one = BigUint::from_u64(1).unwrap();
    let two = BigUint::from_u64(2).unwrap();

    let mut d = &candidate - &one;
    let mut r = 0usize;
    while &d % &two == BigUint::from_u64(0).unwrap() {
        d >>= 1;
        r += 1;
    }

    'witness_loop: for _ in 0..iterations {
        let a = rng.gen_biguint_range(&two, &(&candidate - &one));
        let mut x = mod_pow_bigint(a, d.clone(), candidate.clone());

        if x == one || x == &candidate - &one {
            continue;
        }

        for _ in 0..r - 1 {
            x = mod_pow_bigint(x.clone(), two.clone(), candidate.clone());
            if x == &candidate - &one {
                continue 'witness_loop;
            }
        }

        return false;
    }

    true
}