use divan::black_box;
use rug::Integer;
use primality_tests::bigint_algorithms::{miller_rabin, solovay_strassen};

fn main() {
    divan::main();
}

#[divan::bench(args=[Integer::from(999999937)])]
fn bigint_solovay_strassen_single(arg: &Integer) {
    solovay_strassen(black_box(10), black_box(arg.clone()));
}