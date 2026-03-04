use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

const LIMIT: u64 = 1_000_000;
const STARTS: &[u64] = &[13, 27, 97, 871, 6171, 77031, 837799];

// Isolated Benchmarks
fn bench_get_len(
    c: &mut Criterion,
    name: &str,
    get_len: fn(u64, &mut euler014::Memo) -> usize,
) {
    c.bench_function(name, |b| {
        b.iter_batched(
            || euler014::Memo::new(LIMIT),
            |mut memo| {
                for &start in STARTS {
                    black_box(get_len(black_box(start), &mut memo));
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_get_len_iter(c: &mut Criterion) {
    bench_get_len(c, "get_len_iter", euler014::get_len_iter);
}

fn bench_get_len_loop(c: &mut Criterion) {
    bench_get_len(c, "get_len_loop", euler014::get_len_loop);
}

// Full solve Benchmarks
fn bench_solve_iter(c: &mut Criterion) {
    c.bench_function("solve_with_iter", |b| {
        b.iter(|| euler014::solve_with_iter(black_box(LIMIT)))
    });
}

fn bench_solve_loop(c: &mut Criterion) {
    c.bench_function("solve_with_loop", |b| {
        b.iter(|| euler014::solve_with_loop(black_box(LIMIT)))
    });
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(10))
}

criterion_group!(
    name = collatz;
    config = criterion_config();
    targets = bench_get_len_iter, bench_get_len_loop, bench_solve_iter, bench_solve_loop
);
criterion_main!(collatz);
