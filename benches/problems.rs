use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode_practice::problems::contains_duplicate::contains_duplicate;

fn bench_contains_duplicates(c: &mut Criterion) {
    let nums = vec![1, 2, 3, 1];
    c.bench_function("contains_duplicates", |b| {
        b.iter(|| contains_duplicate(black_box(nums.clone())))
    });
}

criterion_group!(benches, bench_contains_duplicates);
criterion_main!(benches);
