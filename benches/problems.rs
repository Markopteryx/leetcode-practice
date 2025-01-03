#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_practice::problems::*;

fn bench_all(c: &mut Criterion) {
    bench_contains_duplicate(c);
    // bench_two_sum(c);}
}

fn bench_contains_duplicate(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_duplicate");

    let test_cases = vec![
        ("large_no_dups", (1..10_000).collect()),
        (
            "large_with_dups",
            [
                &(1..5_000).collect::<Vec<_>>()[..],
                &(1..5_000).collect::<Vec<_>>()[..],
            ]
            .concat(),
        ),
    ];

    for (name, nums) in test_cases {
        let stdlib_id = BenchmarkId::new("stdlib", name);
        let ahash_id = BenchmarkId::new("ahash", name);

        group.bench_with_input(stdlib_id, &nums, |b, nums| {
            b.iter(|| arrays::contains_duplicate::contains_duplicate(nums.clone()))
        });

        group.bench_with_input(ahash_id, &nums, |b, nums| {
            b.iter(|| arrays::contains_duplicate::contains_duplicate_v2(nums.clone()))
        });
    }
    group.finish();
}

fn bench_two_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_sum");
    let test_cases = vec![("large", ((1..10_000).collect::<Vec<_>>(), 42))];

    for (name, (nums, target)) in test_cases {
        let stdlib_id = BenchmarkId::new("stdlib", name);
        let ahash_id = BenchmarkId::new("ahash", name);

        group.bench_with_input(stdlib_id, &nums, |b, nums| {
            b.iter(|| arrays::two_sum::two_sum(nums.clone(), target))
        });

        group.bench_with_input(ahash_id, &nums, |b, nums| {
            b.iter(|| arrays::two_sum::two_sum_v2(nums.clone(), target))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
