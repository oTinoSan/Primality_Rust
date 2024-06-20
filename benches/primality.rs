use primality_tests::{
    miller_rabin::miller_rabin, miller_rabin::miller_rabin_list, sieve_primes, trial_division,
    trial_divisions, solovay_strassen::solovay_strassen, solovay_strassen::solovay_strassen_list
};

fn main() {
    divan::main();
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn trial_divisions_test(arg: u64) {
    trial_divisions(arg);
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn sieve_primes_test(arg: u64) {
    sieve_primes(arg);
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn miller_rabin_test(arg: u64) {
    miller_rabin_list(10, arg);
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn solovay_strassen_list_test(arg: u64) {
    solovay_strassen_list(10, arg);
}

#[divan::bench(args=[999999937])]
fn trial_division_single_test(arg: u64) {
    trial_division(arg);
}

#[divan::bench(args=[999999937])]
fn miller_rabin_single_test(arg: u64) {
    miller_rabin(10, arg);
}

#[divan::bench(args=[999999937])]
fn solovay_strassen_single_test(arg: u64) {
    solovay_strassen(10, arg);
}