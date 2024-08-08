use divan::black_box;
use primality::advanced_primality_tests::*;
use primality::{miller_list, mult_prime, sieve, solovay_strassen_list, wheel_mult_prime};
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

// #[divan::bench(args = [1000, 10000, 100000, 1000000])]
// fn miller_primes(arg: u32) {
//     black_box(miller_list(black_box(arg)));
// }

// #[divan::bench(args = [1000, 10000, 100000, 1000000])]
// fn solovay_strassen_list_test(arg: u64) {
//     black_box(solovay_strassen_list(10, black_box(arg)));
// }

// #[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
// fn bigint_miller_primes(arg: &Integer) {
//     black_box(bigint_miller_rabin_list(10, black_box(arg.clone())));
// }

// #[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
// fn bigint_solovay_strassen_primes(arg: &Integer) {
//     black_box(bigint_solovay_strassen_list(10, black_box(arg.clone())));
// }

// #[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
// fn threaded_miller_primes_8(arg: &Integer){
//     black_box(threaded_miller_rabin((black_box(arg.clone())), 8u64));
// }

// #[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
// fn threaded_miller_primes_16(arg: &Integer){
//     black_box(threaded_miller_rabin((black_box(arg.clone())), 16u64));
// }
// args = [Integer::from(173),Integer::from(233), Integer::from(307), Integer::from(409)
#[divan::bench(args = [Integer::from(541),Integer::from(809), Integer::from(1069), Integer::from(1223)],  sample_count=1, sample_size=1)]
fn AKS_prime(arg: &Integer){
    black_box(BigIntAKS(black_box(arg.clone())));
}

#[divan::bench(args = [Integer::from(541),Integer::from(809), Integer::from(1069), Integer::from(1223)])]
fn bigint_miller_rabin_single (arg: &Integer){
    black_box(bigint_miller_rabin_ethan(black_box(arg), 10));
}

#[divan::bench(args = [Integer::from(541),Integer::from(809), Integer::from(1069), Integer::from(1223)])]
fn bigint_solovay_strassen_single (arg: &Integer){
    black_box(bigint_solovay_strassen(10, black_box(arg.clone())));
}

#[divan::bench(args = [541, 809, 1069, 1223])]
fn wheel_factoring_single_test (arg: u64) {
    black_box(wheel_factoring_single(black_box(arg)));
}

#[divan::bench(args = [Integer::from(541),Integer::from(809), Integer::from(1069), Integer::from(1223)])]
fn baillie_psw_test_single (arg: &Integer){
    black_box(baillie_psw_test(black_box(arg)));
}
