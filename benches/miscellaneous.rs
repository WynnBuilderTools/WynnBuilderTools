// cargo bench --bench=miscellaneous
extern crate wynn_build_tools;

use criterion::{criterion_group, criterion_main, Criterion};
use wynn_build_tools::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("miscellaneous");

    let apparel = Apparel::default();
    let apparels: Vec<&Apparel> = (0..8).into_iter().map(|_| &apparel).collect();
    let weapon: Weapon = Default::default();
    group.bench_function("Statistics::new", |b| {
        b.iter(|| CommonStat::sum_max_stats(apparels.as_slice(), &weapon))
    });

    let max_indexes = [100, 200, 300, 400, 500, 600, 700];
    group.bench_function("map_to_index_space", |b| {
        b.iter(|| map_to_index_space(&max_indexes, 1))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
