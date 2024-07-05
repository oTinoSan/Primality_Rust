use bigint_num;
use bigint_rug;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use primality::{bigint_num::miller_rabin_threaded, 
    bigint_rug::threaded_bigint_rug, *};
use rug::{ops::Pow, Integer};

fn main() {
    // let result = trial_vect(1000);
    // println!("{:?}", result);
    // let result = trial(1000);
    // println!("{:?}", result);

    // let result = trials::trial_vect_2(1000);
    // println!("{:?}", result);
    // let result = trials::trial_2(1000);
    // println!("{:?}", result);

    // let result = sieve::sieve(1000);
    // println!("{:?}", result);
    // let result = seive::sieve_vect(1000);
    // println!("{:?}", result);

    // let result = wheel::wheel_facts(1000);
    // println!("{:?}", result);
    // let result = wheel::wheel(1000);
    // println!("{:?}", result);

    // let result = miller_rabin::miller_rabin(3, 5);
    // println!("{:?}", result);
    // let result = solovay_strassen::solovay_strassen(479001599);
    // println!("{:?}", result);

    // let num_list: [BigUint; 14] = [
    //     BigUint::from(0u8),
    //     BigUint::from(1u8),
    //     BigUint::from(2u8),
    //     BigUint::from(3u8),
    //     BigUint::from(4u8),
    //     BigUint::from(17u32),
    //     BigUint::from(1000u32),
    //     BigUint::from(2000u32),
    //     BigUint::from(4000u32),
    //     BigUint::from(8000u32),
    //     BigUint::from(16000u32),
    //     BigUint::from(32000u32),
    //     BigUint::from(57973u32),
    //     BigUint::from(898945346530344442u64),
    // ];
    // for item in num_list {
    //     let tested_num = item.clone();
    //     let result = bigint_num::miller_rabin_big::miller_rabin_bignum(item, 5);
    //     println!("Is {} prime? {}", tested_num, result);
    // }

    // let num_list: [Integer; 14] = [
    //     Integer::from(0u8),
    //     Integer::from(1u8),
    //     Integer::from(2u8),
    //     Integer::from(3u8),
    //     Integer::from(4u8),
    //     Integer::from(17u32),
    //     Integer::from(1000u32),
    //     Integer::from(2000u32),
    //     Integer::from(4000u32),
    //     Integer::from(8000u32),
    //     Integer::from(16000u32),
    //     Integer::from(32000u32),
    //     Integer::from(57973u32),
    //     Integer::from(898945346530344442u64),
    // ];
    // for item in num_list {
    //     let tested_num = item.clone();
    //     let result = bigint_rug::miller_rabin_rug::miller_rabin_bigrug(item, 5);
    //     println!("Is {} prime? {}", tested_num, result);
    // }

    let result = threaded_bigint_rug::threaded_bigrug(100, 2);
    println!("{:?}", result);

    //     check_primes_up_to(1_000_000);
    //     pub fn check_primes_up_to(candidate: i64) {
    //         let list: Vec<i64> = (5..=candidate).collect();
    //         for num in list {
    //             let result = bigint_num::miller_rabin_big::miller_rabin_bignum(BigUint::from(num as u64), 5);
    //             // let result = bigint_rug::miller_rabin_rug::miller_rabin_bigrug(Integer::from(num), 5);
    //             println!("Is {} prime? {}", num, result);
    //         }
    //     }
}
