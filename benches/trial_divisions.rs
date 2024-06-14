use criterion::{black_box, criterion_group, criterion_main, Criterion};
use primality_tests::trial_divisions;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("trial division 2^10", |b| {
        b.iter(|| trial_divisions(black_box(2_u64.pow(10))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
