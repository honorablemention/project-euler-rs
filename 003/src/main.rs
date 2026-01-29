// use std::env;
// use std::process;

// #[derive(Debug)]
// enum Node {
//     Leaf(u64),
//     Branch {
//         value: u64,
//         left: Box<Node>,
//         right: Box<Node>,
//     },
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <number>", args[0]);
//         process::exit(1);
//     }

//     let n: u64 = match args[1].parse() {
//         Ok(num) => num,
//         Err(_) => {
//             eprintln!("Please provide a valid integer.");
//             process::exit(1);
//         }
//     };

//     // Build the factor tree
//     let tree = factor_tree(n);
//     println!("Factor tree for {n}:\n{tree:#?}");

//     // Collect primes from the tree
//     let mut primes = Vec::new();
//     collect_primes(&tree, &mut primes);

//     println!("Prime factors: {primes:?}");
//     println!("Largest prime factor: {}", primes.iter().max().unwrap());
// }

// // Recursively build a binary tree of factors
// fn factor_tree(n: u64) -> Node {
//     if let Some(f) = smallest_factor(n) {
//         Node::Branch {
//             value: n,
//             left: Box::new(factor_tree(f)),
//             right: Box::new(factor_tree(n / f)),
//         }
//     } else {
//         Node::Leaf(n)
//     }
// }

// // Find the smallest factor > 1 (if any)
// fn smallest_factor(n: u64) -> Option<u64> {
//     if n <= 3 {
//         return None;
//     }
//     let limit = (n as f64).sqrt() as u64;
//     for i in 2..=limit {
//         if n % i == 0 {
//             return Some(i);
//         }
//     }
//     None
// }

// // Recursively collect all primes (Leaf nodes) into a vector
// fn collect_primes(node: &Node, acc: &mut Vec<u64>) {
//     match node {
//         Node::Leaf(v) => acc.push(*v),
//         Node::Branch { left, right, .. } => {
//             collect_primes(left, acc);
//             collect_primes(right, acc);
//         }
//     }
// }

use std::env;

/// Generate all factors of `n` lazily
fn factors(n: u64) -> impl Iterator<Item = u64> {
    (2..)
        .take_while(move |&i| i * i <= n)
        .filter(move |&i| n % i == 0)
}

/// Prime predicate
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    (2..)
        .take_while(|&i| i * i <= n)
        .all(|i| n % i != 0)
}

/// Lazily yield the prime factors of n
fn prime_factors(n: u64) -> impl Iterator<Item = u64> {
    factors(n).flat_map(move |f| {
        if is_prime(f) {
            Some(f)
        } else {
            None
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <number>");
        return;
    }

    let n: u64 = args[1].parse().expect("Please provide a valid integer");

    let largest_prime_factor = prime_factors(n)
        .chain(
            // include the complementary factor if it’s prime
            factors(n)
                .map(|f| n / f)
                .filter(|&f| is_prime(f))
        )
        .max()
        .unwrap_or(n);

    println!("The largest prime factor of {} is {}", n, largest_prime_factor);
}
