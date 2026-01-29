
fn is_prime(num: u64, primes: &[u64]) -> bool {

    if num < 2 {
        return false;
    }

    for &p in primes {
        if p * p > num {
            break;
        }
        if num % p == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(n: usize) -> u64 {
    if n == 0 { 
        panic!("n must be >= 1");
    }

    if n == 1 { 
        return 2;
    }

    let mut primes: Vec<u64> = vec![2];
    let mut candidate: u64 = 3;

    while primes.len() < n {
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 2;
    }

    *primes.last().unwrap()
}

fn main() {
    let result = nth_prime(10001);
    println!("{result}");
}