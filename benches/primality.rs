use primality_tests::{trial_divisions, wheel_primes, sieve_primes};

fn main() {
    divan::main();
}

#[divan::bench]
fn trial_divisions_test() {
    trial_divisions(1000000);
}

#[divan::bench]
fn sieve_primes_test() {
    sieve_primes(1000000);
}