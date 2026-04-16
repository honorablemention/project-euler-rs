use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const GRID_SIZE: u32 = 20;

fn bench_memoization(c: &mut Criterion) {
    c.bench_function("solve_with_memoization", |b| {
        b.iter(|| euler015::solve_with_memoization(black_box(GRID_SIZE)))
    });
}

fn bench_combinatorics(c: &mut Criterion) {
    c.bench_function("solve_with_combinatorics", |b| {
        b.iter(|| euler015::solve_with_combinatorics(black_box(GRID_SIZE as u64)))
    });
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(10))
}

criterion_group!(
    name = lattice_paths;
    config = criterion_config();
    targets = bench_memoization, bench_combinatorics
);
criterion_main!(lattice_paths);
