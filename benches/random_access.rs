use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        1 | 0 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
