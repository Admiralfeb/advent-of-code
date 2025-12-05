use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_2025::days::day4::Day;
use common::{day::AdventDay, file::get_data_path};

fn benchmark_puzzle1(c: &mut Criterion) {
    let path = get_data_path(2025, "day4.txt");
    let day = Day;
    
    c.bench_function("day4_puzzle1", |b| {
        b.iter(|| {
            day.puzzle1(black_box(&path))
        });
    });
}

criterion_group!(benches, benchmark_puzzle1);
criterion_main!(benches);
