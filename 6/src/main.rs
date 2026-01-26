use std::env;

fn sum_of_squares(num: u64) -> u64 {
    (1..=num)
        .map(|i| i*i)
        .sum()
}

fn square_of_sum(num: u64) -> u64 {
    let r: u64 = (1..=num).sum();
    r * r
}

fn main() {
    let result = square_of_sum(100) - sum_of_squares(100);
    println!("{result}");
}