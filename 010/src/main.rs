use bitvec::prelude::*;

fn within_sieve_limit(limit: usize) -> impl Fn(&usize) -> bool {
    move |&i| {
        let p = 2 * i + 3;
        p * p < limit
    }
}

fn mark_multiples(bits: &mut BitVec, len: usize, i: usize) {
    let p = 2 * i + 3;
    let p2 = p * p;

    let mut j = (p2 - 3) / 2;
    while j < len {
        bits.set(j, true);
        j += p;
    } 
}

fn index_to_prime(i: usize) -> usize {
    2 * i + 3
}

fn sum_primes_below(limit: usize) -> u64 {
    if limit <= 2 {
        return 0;
    }
    let len = (limit - 2) / 2;
    let mut bits: BitVec = bitvec![0; len];

    // Sieve-marking phase
    // Mutates - one closure that owns &mut bits
    (0..len)
        .take_while(within_sieve_limit(limit))
        .for_each(|i| {
            if !bits[i] {
                mark_multiples(&mut bits, len, i);
            }
        });

    // Summation
    // Pure
    2_u64
        + (0..len)
            .filter(|&i| !bits[i])
            .map(|i| index_to_prime(i) as u64)
            .sum::<u64>()
}

fn main() {
    let result = sum_primes_below(2_000_000);
    println!("{result}");
}