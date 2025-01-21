// cargo bench --bench=permutations
extern crate wynn_build_tools;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;

use wynn_build_tools::util::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("permutations");

    let array: [usize; 8] = std::array::from_fn(|i| i);

    group.bench_function("itertools", |b| {
        b.iter(|| for _ in array.iter().permutations(8) {})
    });

    group.bench_function("lexicographic", |b| {
        b.iter(|| {
            let mut array: [usize; 8] = std::array::from_fn(|i| i);
            loop {
                if !next_permutation(black_box(&mut array)) {
                    break;
                }
            }
        })
    });

    group.bench_function("lexicographic with ref", |b| {
        b.iter(|| {
            let mut array: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
            loop {
                if !next_permutation(black_box(&mut array)) {
                    break;
                }
            }
        })
    });

    group.bench_function("lexicographic with ptr", |b| {
        b.iter(|| {
            let mut array: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
            loop {
                if !next_permutation_ptr(black_box(&mut array)) {
                    break;
                }
            }
        })
    });

    group.bench_function("lexicographic with ptr but usize", |b| {
        b.iter(|| {
            let mut array: [&usize; 8] = black_box([&0, &1, &2, &3, &4, &5, &6, &7]);
            loop {
                if !next_permutation_ptr(black_box(&mut array)) {
                    break;
                }
            }
        })
    });

    group.bench_function("bfs_permutation_with_pruning with ptr", |b| {
        b.iter(|| {
            let array: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
            let initial_context = ();
            fn example_pruning(context: (), _depth: usize, _item: &str) -> Option<()> {
                Some(context)
            }

            bfs_permutation_with_prune(&array, initial_context, example_pruning);
        })
    });

    group.finish();
}

criterion_group!(b1, criterion_benchmark);
criterion_main!(b1);
