use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc_2022::aoc_lib::day_03::{get_badge_priority_sum, get_badge_priority_sum_par};

fn criterion_benchmark(c: &mut Criterion) {
    let path = Path::new("./res/day_03/rucksack_list.csv");
    c.bench_function("bedge sum: ", |b| b.iter(|| get_badge_priority_sum(&path)));
    c.bench_function("bedge sum with Hashset: ", |b| {
        b.iter(|| get_badge_priority_sum(&path))
    });
    c.bench_function("bedge sum par: ", |b| {
        b.iter(|| get_badge_priority_sum_par(&path))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
