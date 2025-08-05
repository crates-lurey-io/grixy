use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use grixy::prelude::*;

#[inline]
fn iter_vec(w: usize, h: usize) {
    let vec: Vec<usize> = (0..w * h).collect();
    for i in &vec {
        black_box(i);
    }
}

#[inline]
fn iter_grid(w: usize, h: usize) {
    let grid = GridBuf::<_, _, RowMajor>::new_filled(w, h, 0);
    for i in grid.iter() {
        black_box(i);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Iteration");
    group.bench_function("iter_vec 100x100", |b| {
        b.iter(|| iter_vec(black_box(100), black_box(100)));
    });
    group.bench_function("iter_grid 100x100", |b| {
        b.iter(|| iter_grid(black_box(100), black_box(100)));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
