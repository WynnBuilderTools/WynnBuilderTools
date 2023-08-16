// cargo bench --bench=skill_points
extern crate wynn_build_tools;
use criterion::{criterion_group, criterion_main, Criterion};
use wynn_build_tools::*;

fn criterion_benchmark(c: &mut Criterion) {
    let apparels = gen_test_apparels();
    let apparels: [&Apparel; 8] = apparels
        .iter()
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let mut group = c.benchmark_group("skill point");
    group.bench_function("fast_put", |b| {
        b.iter(|| SkillPoints::fast_put_calculate(&apparels))
    });
    group.bench_function("full_put", |b| {
        b.iter(|| SkillPoints::full_put_calculate(&apparels))
    });
    group.bench_function("fast_gap", |b| b.iter(|| SkillPoints::fast_gap(&apparels)));
    group.finish();
}

criterion_group!(b1, criterion_benchmark);
criterion_main!(b1);
