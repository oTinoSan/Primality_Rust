use super::mod_pow_big::mod_pow_bigint;
use num::bigint::RandBigInt;
use num::BigInt;
use num::FromPrimitive;

pub fn miller_rabin_bignum(candidate: BigInt, iterations: usize) -> bool {
    if candidate == BigInt::from(2u8) || candidate == BigInt::from(3u8) {
        return true;
    }
    if &candidate % &BigInt::from(2u8) == BigInt::from(0u8) || candidate == BigInt::from(1u8) {
        return false;
    }

    let mut rng = rand::thread_rng();
    let one = BigInt::from_u64(1).unwrap();
    let two = BigInt::from_u64(2).unwrap();

    let mut d = &candidate - &one;
    let mut r = 0usize;
    while &d % &two == BigInt::from_u64(0).unwrap() {
        d >>= 1;
        r += 1;
    }

    'witness_loop: for _ in 0..iterations {
        let a = rng.gen_bigint_range(&two, &(&candidate - &one));
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
