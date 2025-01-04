// cargo bench --bench=skill_points
extern crate wynn_build_tools;
use criterion::{criterion_group, criterion_main, Criterion};
use wynn_build_tools::tests::*;
use wynn_build_tools::*;

fn criterion_benchmark(c: &mut Criterion) {
    let apparels: Vec<[Apparel; 8]> = gen_test_apparels()
        .into_iter()
        .map(|v| v.apparels)
        .collect();
    let apparels: Vec<[&Apparel; 8]> = apparels
        .iter()
        .map(|array| std::array::from_fn(|i| &array[i]))
        .collect();
    let mut group = c.benchmark_group("skill point");
    group.bench_function("full_put", |b| {
        b.iter(|| {
            for v in apparels.clone() {
                SkillPoints::full_put_calculate(&v);
            }
        })
    });
    group.bench_function("fast_gap", |b| {
        b.iter(|| {
            for v in apparels.clone() {
                SkillPoints::fast_gap(&v);
            }
        })
    });
    group.finish();
}

criterion_group!(b1, criterion_benchmark);
criterion_main!(b1);
