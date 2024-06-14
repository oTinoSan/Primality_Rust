use criterion::{black_box, criterion_group, criterion_main, Criterion};
use primality_tests::wheel_primes;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("wheel primes 2^10", |b| {
        b.iter(|| wheel_primes(vec![2, 3], black_box(2_u64.pow(10))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
