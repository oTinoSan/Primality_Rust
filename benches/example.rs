use divan;
use num::BigUint;
use primality::{
    bigint_num::{
        miller_rabin_big::miller_rabin_bignum,
        miller_rabin_threaded::{miller_rabin_array, miller_rabin_threaded},
    },
    bigint_rug::miller_rabin_rug::miller_rabin_bigrug,
    miller_rabin::miller_rabin,
    sieve::sieve,
    solovay_strassen::solovay_strassen,
    trials::{trial, trial_2},
    wheel::wheel,
};
use rug::Integer;

fn main() {
    divan::main();
}

// #[divan::bench(args = [1000, 2149, 2334])]
// fn trial_test(arg: u32) {
//     trial(arg);
// }

// #[divan::bench(args = [1000, 2149, 2334])]
// fn trial_test_2(args: u32) {
//     trial_2(args);
// }

// // Register a `fibonacci` function and benchmark it over multiple cases.
// #[divan::bench(args = [1000, 2149, 2334])]
// fn sieve_test(arg: u32) {
//     sieve(arg);
// }
// #[divan::bench(args = [1000, 2149, 2334])]
// fn wheel_test(arg: u64) {
//     wheel(arg);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 3u64.pow(43), 4u64.pow(26)])]
// fn miller_rabin_test(args: u64) {
//     miller_rabin(args, 5);
// }

// #[divan::bench(args = [1000, 2111, 2149, 2334])]
// fn solovay_strassen_test(args: i64) {
//     solovay_strassen(args);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn miller_rabin_array_test (arg: u32) {
//     miller_rabin_array(arg);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 3u64.pow(43), 4u64.pow(26)])]
// fn miller_rabin_bigint_num_test (arg: u64) {
//     miller_rabin_bignum(BigUint::from(arg), 5);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 3u64.pow(43), 4u64.pow(26)])]
// fn miller_rabin_bigint_rug_test (arg: u64) {
//     miller_rabin_bigrug(Integer::from(arg), 5);
// }

#[divan::bench(args = [1_000_000i64])]
fn check_all_primes_bignum(arg: i64) {
    let list: Vec<i64> = (5..=arg).collect();
    for num in list {
        miller_rabin_bignum(BigUint::from(num as u64), 5);
        // let result = bigint_rug::miller_rabin_rug::miller_rabin_bigrug(Integer::from(num), 5);
        // println!("Is {} prime? {}", num, result);
    }
}

#[divan::bench(args = [1_000_000i64])]
fn check_all_primes_bigrug(arg: i64) {
    let list: Vec<i64> = (5..=arg).collect();
    for num in list {
        miller_rabin_bignum(BigUint::from(num as u64), 5);
        // let result = bigint_rug::miller_rabin_rug::miller_rabin_bigrug(Integer::from(num), 5);
        // println!("Is {} prime? {}", num, result);
    }
}
// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn threaded_miller_rabin_array_test_8 (arg: u32) {
//     miller_rabin_threaded(BigUint::from(arg), 3, 8);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn threaded_miller_rabin_array_test_16 (arg: u64) {
//     miller_rabin_threaded(BigUint::from(arg), 3, 16);
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn threaded_miller_rabin_array_test_128 (arg: u64) {
//     miller_rabin_threaded(BigUint::from(arg), 3, 128);
//}
