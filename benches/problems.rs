use contains_duplicate::{contains_duplicate_v1, contains_duplicate_v2};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_practice::problems::*;

fn bench_all(c: &mut Criterion) {
    bench_contains_duplicate(c);
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
            b.iter(|| contains_duplicate_v1(nums.clone()))
        });

        group.bench_with_input(ahash_id, &nums, |b, nums| {
            b.iter(|| contains_duplicate_v2(nums.clone()))
        });
    }
    group.finish();
}

fn _bench_two_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_sum");
    let test_cases = vec![
        ("small", (vec![2, 7, 11, 15], 9)),
        ("large", ((1..1000).collect(), 42)),
    ];

    for (size, (nums, target)) in test_cases {
        group.bench_with_input(BenchmarkId::new("v1_hashmap", size), &nums, |b, nums| {
            b.iter(|| two_sum::two_sum(black_box(nums.clone()), black_box(target)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
