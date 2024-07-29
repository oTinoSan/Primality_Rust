use divan::black_box;
use primality::advanced_primality_tests::*;
use primality::wheel_algos::*;
use rug::Integer;

const THREAD_ARGS: [u64; 3] = [32, 64, 128];

fn main() {
    divan::main();
}

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn solovay_strassen_bench(num_threads: u64) {
//     black_box(solovay_strassen(
//         10,
//         Integer::from(1000000000),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn miller_rabin_bench(num_threads: u64) {
//     black_box(miller_rabin(
//         10,
//         Integer::from(1000000000),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn solovay_strassen_general_bench(num_threads: u64) {
//     black_box(solovay_strassen_general(
//         10,
//         Integer::from(1000000000),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn miller_rabin_general_bench(num_threads: u64) {
//     black_box(miller_rabin_general(
//         black_box(10),
//         black_box(Integer::from(1000000000)),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn miller_solovay_bench(num_threads: u64){
//     black_box( miller_solovay(5, Integer::from(1000000000),black_box(num_threads)));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn general_miller_solovay_bench(num_threads: u64){
//     black_box( general_miller_solovay(5, Integer::from(1000000000),black_box(num_threads)));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn baillie_psw_bench(num_threads: u64) {
//     black_box(baillie_psw_wheel_threaded(
//         Integer::from(1000000000),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn general_baille_psw_bench(num_threads: u64) {
//     black_box(baillie_psw_general_wheel(
//         Integer::ZERO,
//         Integer::from(1000000000),
//         black_box(num_threads),
//         vec![2, 3, 5],
//         vec![1, 7, 11, 13, 17, 19, 23, 29],
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn threaded_baillie_psw_benchmark(num_threads: u64) {
//     black_box(threaded_baillie_psw(
//         Integer::ZERO,
//         Integer::from(1000000000),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn threaded_miller_rabin_benchmark(num_threads: u64) {
//     black_box(threaded_miller_rabin(
//         Integer::from(1000000000),
//         black_box(num_threads),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn threaded_solovay_strassen_benchmark(num_threads: u64) {
//     black_box(threaded_solovay_strassen(
//         black_box(num_threads),
//         Integer::from(1000000000),
//     ));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn threaded_AKS_prime_benchmark(num_threads: u64) {
//     black_box(threaded_AKS_prime(
//         black_box(num_threads),
//         Integer::from(1000000000),
//     ));
// }

#[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
fn AKS_prime(arg: &Integer){
    black_box(BigIntAKS(black_box(arg.clone())));
}

#[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
fn bigint_miller_rabin_single (arg: &Integer){
    black_box(bigint_miller_rabin(10, black_box(arg.clone())));
}

#[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
fn bigint_solovay_strassen_single (arg: &Integer){
    black_box(bigint_solovay_strassen(10, black_box(arg.clone())));
}

#[divan::bench(args = [Integer::from(1000),Integer::from(10000), Integer::from(100000), Integer::from(1000000)])]
fn baillie_psw_test (arg: &Integer){
    black_box(baillie_psw_test(black_box(arg)));
}