use primality_tests::{
    miller_rabin::miller_rabin, miller_rabin::miller_rabin_list, sieve_primes, trial_division,
    trial_divisions, solovay_strassen::solovay_strassen, solovay_strassen::solovay_strassen_list
};
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn trial_divisions_test(arg: u64) {
    black_box(trial_divisions(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn sieve_primes_test(arg: u64) {
    black_box(sieve_primes(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn miller_rabin_test(arg: u64) {
    black_box(miller_rabin_list(black_box(10), black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn solovay_strassen_list_test(arg: u64) {
    black_box(solovay_strassen_list(black_box(10), black_box(arg)));
}

#[divan::bench(args=[999999937])]
fn trial_division_single_test(arg: u64) {
    black_box(trial_division(black_box(arg)));
}

#[divan::bench(args=[999999937])]
fn miller_rabin_single_test(arg: u64) {
    black_box(miller_rabin(black_box(10), black_box(arg)));
}

#[divan::bench(args=[999999937])]
fn solovay_strassen_single_test(arg: u64) {
    black_box(solovay_strassen(black_box(10), black_box(arg)));
}