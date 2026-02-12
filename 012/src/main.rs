#[derive(Debug, Clone)]
struct PrimeState {
    limit: u64,
    sieve: Vec<u8>,
    primes: Vec<u64>,
}

fn new_prime_state() -> PrimeState {
    PrimeState {
        limit: 1,
        sieve: vec![0, 0],
        primes: Vec::new(),
    }
}

fn triangle_value(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn triangle_parts(n: u64) -> (u64, u64) {
    if n % 2 == 0 { 
        (n/2, n + 1)
    } else {
        (n, (n + 1) / 2)
    }
}

fn extend_sieve_to(new_limit: u64, state: &mut PrimeState) {
    let old = state.limit as usize;
    let new = new_limit as usize;

    // 1) Grow sieve, initialize new entries as "potential prime"
    state.sieve.resize((new + 1), 1);
    state.sieve[0] = 0;
    if new >= 1 {
        state.sieve[1] = 0;
    }
    // 2) Mark composites, but only in the newly added range (old+1..=new)
    let old_plus_1 = old + 1;

    // Make sieve consistent: evens > 2 are composite
    if new >= 2 {
        state.sieve[2] = 1;
    }

    // Zero out even numbers in the newly added range
    let mut e = old_plus_1.max(4);
    if e % 2 == 1 { e += 1; }
    while e <= new {
        state.sieve[e] = 0;
        e += 2;
    }

    // Only add p
    let mut p = 3_usize;
    while p * p <= new {
        if state.sieve[p] == 1 {
            // first multiple >= old+1
            let mut m = (old_plus_1 + p - 1) / p * p;
            let pp = p * p;
            if m < pp {
                m = pp;
            }

            // ensure m is odd (so we only mark odd multiples)
            if m % 2 == 0 {
                m += p;
            }

            // step by 2*p to stay on odd multiples
            let step = 2 * p;
            while m <= new {
                state.sieve[m] = 0;
                m += step;
            }
        }
        p += 2;
    }


    // 3) Append newly discovered primes to cached list (avoid duplicates)
    let start = old_plus_1.max(2);
    if start <= 2 && new >= 2 && state.sieve[2] == 1 {
        if state.primes.first().copied() != Some(2) {
            state.primes.push(2);
        }
    }


    // then only scan odds
    let mut i = start.max(3);
    if i % 2 == 0 { i += 1; }
    while i <= new {
        if state.sieve[i] == 1 {
            state.primes.push(i as u64);
        }
        i += 2;
    }


    // 4) Update the coverage limit
    state.limit = new_limit;
}

fn ensure_primes_up_to(needed: u64, state: &mut PrimeState) {
    if needed <= state.limit {
        return;
    }
    let new_limit = needed.max(state.limit.saturating_mul(2));
    extend_sieve_to(new_limit, state);
}

fn num_divisors(n: u64, s: &mut PrimeState) -> u64 {
    if n == 1 { return 1; }

    let mut x = n;
    let needed = (x as f64).sqrt() as u64;
    ensure_primes_up_to(needed, s);

    let mut total = 1_u64;

    for &p in s.primes.iter() {
        if p * p > x { break; }
        if x % p != 0 { continue; }

        let mut exp = 0u64;
        while x % p == 0 {
            x /= p;
            exp += 1;
        }

        total *= exp + 1;

        if x == 1 {
            break;
        }
    }

    if x > 1 { total * 2 } else { total }

}

fn solve(target: u64) -> u64 {
    let mut state = new_prime_state();
    let mut n: u64 = 1;
    loop {
        let (a, b) = triangle_parts(n);
        let da = num_divisors(a, &mut state);
        let db = num_divisors(b, &mut state);

        if da * db > target {
            return triangle_value(n);
        }

        n += 1;
    }
}

fn main() {
    println!("{}", solve(500));
}