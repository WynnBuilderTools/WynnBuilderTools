// cargo bench --bench=select_from_arrays
extern crate wynn_build_tools;

use criterion::{criterion_group, criterion_main, Criterion};
use wynn_build_tools::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("select_from_arrays");

    let long_array = (0..1000)
        .into_iter()
        .map(|_| TestStruct {})
        .collect::<Vec<TestStruct>>();
    let binding = (0..8)
        .into_iter()
        .map(|_| long_array.as_slice())
        .collect::<Vec<&[TestStruct]>>();
    let long_arrays: &[&[TestStruct]; 8] = binding.as_slice().try_into().unwrap();
    let index = [700, 200, 100, 700, 200, 100, 700, 200];
    group.bench_function("select_from_arrays", |b| {
        b.iter(|| unsafe { select_from_arrays(&index, long_arrays) })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[derive(Debug, Copy, Clone)]
struct TestStruct {}
impl AsRef<TestStruct> for TestStruct {
    fn as_ref(&self) -> &TestStruct {
        &self
    }
}
