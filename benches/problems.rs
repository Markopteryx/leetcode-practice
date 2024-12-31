// benches/problems.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode_practice::problems::*;

fn bench_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("problems");

    // Contains Duplicate
    {
        let nums = vec![1, 2, 3, 1];
        group.bench_function("contains_duplicate", |b| {
            b.iter(|| contains_duplicate::contains_duplicate(black_box(nums.clone())))
        });
    }

    // Two Sum
    {
        let nums = vec![2, 7, 11, 15];
        group.bench_function("two_sum", |b| {
            b.iter(|| two_sum::two_sum(black_box(nums.clone()), black_box(9)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
