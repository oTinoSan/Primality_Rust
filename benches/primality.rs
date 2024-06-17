use primality_tests::{sieve_primes, trial_divisions, wheel_primes, miller_rabin::miller_rabin_list};

fn main() {
    divan::main();
}

#[divan::bench]
fn trial_divisions_test() {
    trial_divisions(1000);
}

#[divan::bench]
fn sieve_primes_test() {
    sieve_primes(1000);
}

// #[divan::bench]
// fn wheel_primes_test() {
//     wheel_primes(vec![2, 3], 10000);
// }

#[divan::bench]
fn miller_rabin_test() {
    miller_rabin_list(5, 1000);
}