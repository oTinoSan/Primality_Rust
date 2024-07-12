use divan::black_box;
use primality::{
miller_list, mult_prime, sieve, solovay_strassen_list, wheel_mult_prime, u64AKS
};
use primality::wheel_algos::*;
use rug::Integer;

fn main() {
    divan::main();
}

// #[divan::bench(args = [1000, 10000, 100000, 1000000])]
// fn trial_division(arg: u32) {
//     black_box(mult_prime(black_box(arg)));
// }

// #[divan::bench(args = [1000, 10000, 100000, 1000000])]
// fn wheel_factorization(arg: u32) {
//     black_box(wheel_mult_prime(black_box(arg)));
// }

// #[divan::bench(args = [1000, 10000, 100000, 1000000])]
// fn sieve_primes(arg: u32) {
//     black_box(sieve(black_box(arg)));
// }

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn miller_primes(arg: u32) {
    black_box(miller_list(black_box(arg)));
}

#[divan::bench(args = [1000, 10000, 100000, 1000000])]
fn solovay_strassen_list_test(arg: u64) {
    black_box(solovay_strassen_list(10, black_box(arg)));
}

#[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
fn bigint_miller_primes(arg: &Integer) {
    black_box(bigint_miller_rabin_list(10, black_box(arg.clone())));
}

#[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
fn bigint_solovay_strassen_primes(arg: &Integer) {
    black_box(bigint_solovay_strassen_list(10, black_box(arg.clone())));
}

// #[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
// fn threaded_miller_primes_8(arg: &Integer){
//     black_box(threaded_miller_rabin((black_box(arg.clone())), 8u64));
// }

// #[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
// fn threaded_miller_primes_16(arg: &Integer){
//     black_box(threaded_miller_rabin((black_box(arg.clone())), 16u64));
// }

// #[divan::bench(args = [11, 12, 100, 101, 300, 307, 692, 693, 901, 902, 1003, 1013])]
// fn AKS_primes(arg: u64){
//     black_box(u64AKS(black_box(arg)));
// }