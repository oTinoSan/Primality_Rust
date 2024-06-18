use primality::{
    miller_rabin::miller_rabin, sieve::sieve, wheel::wheel,
    solovay_strassen::solovay_strassen, trial, trial_2,
};
use divan; // Add the divan crate as a dependency

fn main() {
    // Run registered benchmarks.
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

#[divan::bench(args = [1000, 2149, 2334])]
fn miller_rabin_test(args: u64) {
    miller_rabin(args);
}

#[divan::bench(args = [1000, 2149, 2334])]
fn solovay_strassen_test(args: i64) {
    solovay_strassen(args);
}
