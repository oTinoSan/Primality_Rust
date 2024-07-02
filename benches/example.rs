use num::BigUint;
use divan;
use primality::{
    bigint_num::{miller_rabin_big::miller_rabin_bigint, miller_rabin_threaded::{miller_rabin_array, miller_rabin_threaded}}, miller_rabin::miller_rabin, sieve::sieve, solovay_strassen::solovay_strassen, trials::{trial, trial_2}, wheel::wheel
};

fn main() {
    divan::main();
}

#[divan::bench(args = [1000, 2149, 2334])]
fn trial_test(arg: u32) {
    trial(arg);
}

#[divan::bench(args = [1000, 2149, 2334])]
fn trial_test_2(args: u32) {
    trial_2(args);
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = [1000, 2149, 2334])]
fn sieve_test(arg: u32) {
    sieve(arg);
}
#[divan::bench(args = [1000, 2149, 2334])]
fn wheel_test(arg: u64) {
    wheel(arg);
}

#[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 3u64.pow(43), 4u64.pow(26)])]
fn miller_rabin_test(args: u64) {
    miller_rabin(args, 5);
}

#[divan::bench(args = [1000, 2111, 2149, 2334])]
fn solovay_strassen_test(args: i64) {
    solovay_strassen(args);
}

#[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
fn miller_rabin_array_test (arg: u32) {
    miller_rabin_array(arg);
}

#[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000, 3u64.pow(43), 4u64.pow(26)])]
fn miller_rabin_bigint_test (arg: u64) {
    miller_rabin_bigint(BigUint::from(arg), 5);
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
