use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::Deserialize;

use euler013::get_first_n_from_sum_of;

#[derive(Deserialize)]
struct InputNumber {
    input: Vec<String>,
}

fn load_input() -> InputNumber {
    let json = include_str!("../src/number.json");
    serde_json::from_str(json).expect("valid json")
}

fn bench_first_n(c: &mut Criterion) {
    let input_number = load_input();
    let input = &input_number.input;
    c.bench_function("get_first_10", |b| {
        b.iter(|| {
            let result = get_first_n_from_sum_of(10, black_box(input)).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(benches, bench_first_n);
criterion_main!(benches);
