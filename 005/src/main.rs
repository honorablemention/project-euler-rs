use std::env;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    match (a,b) {
        (0,_) | (_,0) => 0,
        _ => (a / gcd(a,b)) * b,
    }
}

fn lcm_up_to(max: u64) -> u64 {
    if max < 2 { 
        return 1;
    }
    
    (2..=max).fold(1_u64, |acc, n| lcm(acc, n))
}
fn main() {
    let result = lcm_up_to(20);
    println!("{result}");
}