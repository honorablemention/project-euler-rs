use std::env;

fn is_palindromic(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut i = 0_usize;
    let mut j = bytes.len().saturating_sub(1);

    while i < j {
        if bytes[i] != bytes[j] {
            return false;
        }

        i += 1;
        j -= 1;
    }
    true
}

fn find_largest_palindrome(digits: u64) -> u64 {
    let max: u64 = 10_u64.pow(digits as u32) - 1;
    let min: u64 = 10_u64.pow((digits - 1) as u32);

    let mut best: u64 = 0_u64;

    for i in (min..=max).rev() {
        if i * max <= best {
            break;
        }

        for j in (min..=i).rev() {
            let candidate = i * j;

            if candidate <= best {
                break;
            }

            if is_palindromic(candidate) {
                best = candidate;
                break;
            }
        }
    }
    best
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <number>");
        return;
    }

    let digits: u64 = args[1].parse().expect("Please provide a valid integer");
    if digits == 0 {
        eprintln!("Digits must be >= 1");
        return;
    }

    let result = find_largest_palindrome(digits);

    println!("{result}");
}