use divan::black_box;
use primality_tests::bigint_algorithms::{
    miller_rabin, miller_rabin_range, miller_rabin_threaded, solovay_strassen, solovay_strassen_range, solovay_strassen_threaded
};
use rug::Integer;

fn main() {
    divan::main();
}

/// Benchmark single integer `999999937` with bigint solovay-strassen
// #[divan::bench(args=[Integer::from(999999937)])]
// fn bigint_solovay_strassen_single(arg: &Integer) {
//     solovay_strassen(black_box(10), black_box(arg.clone()));
// }

// #[divan::bench()]
// fn bigint_miller_rabin_range() {
//     miller_rabin_range(black_box(10), black_box(Integer::from(1000000)));
// }

#[divan::bench(sample_count = 10, sample_size = 10, args=[1, 2, 4, 8, 16, 32, 64, 128])]
fn bigint_miller_rabin_threaded_range(num_threads: u64) {
    miller_rabin_threaded(
        black_box(10),
        black_box(Integer::from(100000000)),
        black_box(num_threads),
    );
}

// #[divan::bench()]
// fn bigint_solovay_strassen_range() {
//     solovay_strassen_range(black_box(10), black_box(Integer::from(1000000)));
// }

#[divan::bench(sample_count = 10, sample_size = 10, args=[1, 2, 4, 8, 16, 32, 64, 128])]
fn bigint_solovay_strassen_threaded_range(num_threads: u64) {
    solovay_strassen_threaded(
        black_box(10),
        black_box(Integer::from(100000000)),
        black_box(num_threads),
    );
}