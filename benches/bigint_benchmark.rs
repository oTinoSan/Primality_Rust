use divan::black_box;
use primality_tests::bigint_algorithms::*;
use rug::Integer;

const THREAD_ARGS: [u64; 3] = [32, 64, 128];

fn main() {
    divan::main();
}

// #[divan::bench(args=[Integer::from(999999937)])]
// fn bigint_solovay_strassen_single(arg: &Integer) {
//     solovay_strassen(black_box(10), black_box(arg.clone()));
// }

// #[divan::bench()]
// fn bigint_miller_rabin_range() {
//     miller_rabin_range(black_box(10), black_box(Integer::from(1000000)));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn bigint_miller_rabin_threaded_range(num_threads: u64) {
//     miller_rabin_threaded(
//         black_box(10),
//         black_box(Integer::from(u32::MAX)),
//         black_box(num_threads),
//     );
// }

// #[divan::bench()]
// fn bigint_solovay_strassen_range() {
//     solovay_strassen_range(black_box(10), black_box(Integer::from(1000000)));
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn bigint_solovay_strassen_threaded_range(num_threads: u64) {
//     solovay_strassen_threaded(
//         black_box(10),
//         black_box(Integer::from(u32::MAX)),
//         black_box(num_threads),
//     );
// }

// #[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
// fn solovay_strassen_wheel_bench(num_threads: u64) {
//     solovay_strassen_wheel(
//         black_box(10),
//         black_box(Integer::from(u32::MAX)),
//         black_box(num_threads),
//     );
// }

#[divan::bench(sample_count = 1, sample_size = 1, args=THREAD_ARGS)]
fn miller_rabin_wheel_bench(num_threads: u64) {
    miller_rabin_wheel(
        black_box(10),
        black_box(Integer::from(1000000000)),
        black_box(num_threads),
    );
}

#[divan::bench(sample_count = 1, sample_size = 1)]
fn miller_rabin_rayon_wheel_bench() {
    miller_rabin_rayon(black_box(10), black_box(Integer::from(1000000000)));
}

// Ethan's tests

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn miller_rabin_array_test(arg: u32) {
//     miller_rabin_range(black_box(10), black_box(Integer::from(arg)));
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn threaded_miller_rabin_array_test_8(arg: u32) {
//     miller_rabin_threaded(black_box(10), black_box(Integer::from(arg)), black_box(8));
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn threaded_miller_rabin_array_test_16(arg: u32) {
//     miller_rabin_threaded(black_box(10), black_box(Integer::from(arg)), black_box(16));
// }

// #[divan::bench(args = [1000, 2000, 4000, 8000, 16000, 32000])]
// fn threaded_miller_rabin_array_test_128(arg: u32) {
//     miller_rabin_threaded(black_box(10), black_box(Integer::from(arg)), black_box(128));
// }
