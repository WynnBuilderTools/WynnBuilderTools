// cargo bench --bench=skill_points
extern crate wynn_build_tools;
use criterion::{criterion_group, criterion_main, Criterion};

use wynn_build_tools::calculate::*;
use wynn_build_tools::items::*;
use wynn_build_tools::tests::*;

fn criterion_benchmark(c: &mut Criterion) {
    let apparels: Vec<([Apparel; 8], Weapon)> = gen_test_apparels()
        .into_iter()
        .map(|v| (v.apparels, v.weapon))
        .collect();
    let apparels_ref: Vec<([&Apparel; 8], &Weapon)> = apparels
        .iter()
        .map(|array| (std::array::from_fn(|i| &array.0[i]), &array.1))
        .collect();

    let mut group = c.benchmark_group("skill point");
    group.bench_function("full_put", |b| {
        b.iter(|| {
            for v in apparels_ref.clone() {
                #[allow(deprecated)]
                SkillPoints::full_put_calculate(&v.0);
            }
        })
    });
    group.bench_function("prune_put", |b| {
        b.iter(|| {
            for v in apparels_ref.clone() {
                #[allow(deprecated)]
                SkillPoints::prune_put_calculate(&v.0);
            }
        })
    });
    group.bench_function("scc_put", |b| {
        b.iter(|| {
            for v in apparels_ref.clone() {
                SkillPoints::scc_put_calculate(&v.0, v.1);
            }
        })
    });
    group.bench_function("fast_gap", |b| {
        b.iter(|| {
            for v in apparels_ref.clone() {
                let _ = SkillPoints::fast_gap(&v.0).only_negative().sum().abs();
            }
        })
    });
    group.finish();
}

criterion_group!(b1, criterion_benchmark);
criterion_main!(b1);
