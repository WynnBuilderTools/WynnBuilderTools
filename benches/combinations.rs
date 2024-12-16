// cargo bench --bench=combinations
extern crate wynn_build_tools;

use std::sync::{atomic::AtomicUsize, Arc};

use criterion::{criterion_group, criterion_main, Criterion};
use wynn_build_tools::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("combinations");

    let binding = [&Apparel::default(), &Apparel::default()];
    let arrays: [&[&Apparel]; 6] = [&binding; 6];
    group.bench_function("index map 2", |b| {
        b.iter(|| {
            let counter = Arc::new(AtomicUsize::new(0));
            generate_full_combinations_with_random(100, counter, &arrays, |_| {});
        })
    });
    group.bench_function("get_combinations 2", |b| {
        b.iter(|| get_combinations(&arrays))
    });

    let binding = [
        &Apparel::default(),
        &Apparel::default(),
        &Apparel::default(),
    ];
    let arrays: [&[&Apparel]; 6] = [&binding; 6];
    group.bench_function("index map 3", |b| {
        b.iter(|| {
            let counter = Arc::new(AtomicUsize::new(0));
            generate_full_combinations_with_random(100, counter, &arrays, |_| {});
        })
    });
    group.bench_function("get_combinations 3", |b| {
        b.iter(|| get_combinations(&arrays))
    });

    let binding = [
        &Apparel::default(),
        &Apparel::default(),
        &Apparel::default(),
        &Apparel::default(),
    ];
    let arrays: [&[&Apparel]; 6] = [&binding; 6];
    group.bench_function("index map 4", |b| {
        b.iter(|| {
            let counter = Arc::new(AtomicUsize::new(0));
            generate_full_combinations_with_random(100, counter, &arrays, |_| {});
        })
    });
    group.bench_function("get_combinations 4", |b| {
        b.iter(|| get_combinations(&arrays))
    });

    let binding = [
        &Apparel::default(),
        &Apparel::default(),
        &Apparel::default(),
        &Apparel::default(),
        &Apparel::default(),
    ];
    let arrays: [&[&Apparel]; 6] = [&binding; 6];
    group.bench_function("index map 5", |b| {
        b.iter(|| {
            let counter = Arc::new(AtomicUsize::new(0));
            generate_full_combinations_with_random(100, counter, &arrays, |_| {});
        })
    });
    group.bench_function("get_combinations 5", |b| {
        b.iter(|| get_combinations(&arrays))
    });

    group.finish();
}
criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(5)); // Set the measurement time here
    targets = criterion_benchmark
}

criterion_main!(benches);

fn get_combinations<T: Clone, const LEN: usize>(arr: &[&[T]; LEN]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone, const LEN: usize>(
        arr: &[&[T]; LEN],
        start: usize,
        current_permutation: &mut Vec<T>,
        result: &mut Vec<Vec<T>>,
    ) {
        if start == arr.len() {
            result.push(current_permutation.clone());
            return;
        }

        for i in 0..arr[start].len() {
            current_permutation[start] = arr[start][i].clone();
            backtrack(arr, start + 1, current_permutation, result);
        }
    }

    let mut result: Vec<Vec<T>> = Vec::new();
    let mut current_permutation: Vec<T> = vec![arr[0][0].clone(); arr.len()];
    backtrack(arr, 0, &mut current_permutation, &mut result);
    result
}
