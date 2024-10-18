// cargo bench --bench=as_ref
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
extern crate wynn_build_tools;

use criterion::{criterion_group, criterion_main, Criterion};
use std::mem::MaybeUninit;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("miscellaneous");

    let array: [TestStruct; 1000] = [TestStruct {}; 1000];

    group.bench_function("as_ref", |b| b.iter(|| array_as_ref(&array)));
    group.bench_function("no_as_ref", |b| b.iter(|| array_no_as_ref(&array)));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn array_as_ref<TR, T, const LEN: usize>(array: &[TR; LEN]) -> [&T; LEN]
where
    TR: AsRef<T>,
{
    let mut result: [MaybeUninit<&T>; LEN] = MaybeUninit::uninit_array();
    for i in 0..LEN {
        result[i].write(array[i].as_ref());
    }
    unsafe { MaybeUninit::array_assume_init(result) }
}
fn array_no_as_ref<T, const LEN: usize>(array: &[T; LEN]) -> [&T; LEN] {
    let mut result: [&T; LEN] = unsafe { std::mem::zeroed() };
    for i in 0..LEN {
        result[i] = &array[i];
    }
    result
}

#[derive(Debug, Copy, Clone)]
struct TestStruct {}
impl AsRef<TestStruct> for TestStruct {
    fn as_ref(&self) -> &TestStruct {
        self
    }
}
