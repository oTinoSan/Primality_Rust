use divan::black_box;
use primality_tests::bigint_algorithms::{
    miller_rabin, miller_rabin_range, miller_rabin_threaded, solovay_strassen, solovay_strassen_range, solovay_strassen_threaded
};
use rug::Integer;

fn main() {
    divan::main();
}

#[divan::bench(args=[Integer::from(999999937)])]
fn bigint_solovay_strassen_single(arg: &Integer) {
    solovay_strassen(black_box(10), black_box(arg.clone()));
}

#[divan::bench()]
fn bigint_miller_rabin_range() {
    miller_rabin_range(black_box(10), black_box(Integer::from(1000000)));
}

#[divan::bench()]
fn bigint_miller_rabin_threaded_range() {
    miller_rabin_threaded(
        black_box(10),
        black_box(Integer::from(1000000)),
        black_box(4),
    );
}

#[divan::bench()]
fn bigint_solovay_strassen_range() {
    solovay_strassen_range(black_box(10), black_box(Integer::from(1000000)));
}

#[divan::bench()]
fn bigint_solovay_strassen_threaded_range() {
    solovay_strassen_threaded(
        black_box(10),
        black_box(Integer::from(1000000)),
        black_box(4),
    );
}