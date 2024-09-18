// cargo bench --bench=random_number
extern crate wynn_build_tools;
use std::sync::{atomic::AtomicUsize, Arc};

use criterion::{criterion_group, criterion_main, Criterion};
use wynn_build_tools::*;

fn criterion_benchmark(c: &mut Criterion) {
    let max = 10000000000;
    let counter = Arc::new(AtomicUsize::new(0));
    let mut x_1 = segmented_random_numbers(max, 10000, counter.clone(), None);
    let mut x_2 = segmented_random_numbers(max, 1000000, counter.clone(), None);
    let mut x_3 = segmented_random_numbers(max, 1000, counter.clone(), None);
    let mut y = random_numbers(10000000000);
    let rng = fastrand::Rng::default();

    let mut group = c.benchmark_group("random_number");
    group.bench_function("segmented_random_numbers, size: 1000", |b| {
        b.iter(|| x_3.next())
    });
    group.bench_function("segmented_random_numbers, size: 10000", |b| {
        b.iter(|| x_1.next())
    });
    group.bench_function("segmented_random_numbers, size: 1000000", |b| {
        b.iter(|| x_2.next())
    });
    group.bench_function("old random", |b| b.iter(|| y.next()));
    group.bench_function("single rng", |b| b.iter(|| rng.usize(0..max)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
