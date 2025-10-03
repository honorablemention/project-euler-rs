fn is_multiple_of3_or5(n: i32) -> bool {
    n % 3 == 0 || n % 5 == 0
}

fn main() {
    let sum: i32 = (0..1000)
        .filter(|&x| is_multiple_of3_or5(x))
        .sum();
    println!("Result is {}", sum);
}