use divan::black_box;
use primality::{mult_prime, sieve, miller_list, solovay_strassen_list, wheel_mult_prime};

fn main() {
    divan::main();
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn trial_division(arg: u32){
    black_box(mult_prime(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn wheel_factorization (arg: u32){
    black_box(wheel_mult_prime(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn sieve_primes (arg: u32){
    black_box(sieve(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn miller_primes(arg:u32){
    black_box(miller_list(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn solovay_strassen_list_test(arg:u64){
    black_box(solovay_strassen_list(10, black_box(arg)));
}