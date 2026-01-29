// Euclid's formula for Pythagorean triples:
//   a = m^2 - n^2
//   b = 2mn
//   c = m^2 + n^2
//
// Their sum is:
//   a + b + c = 2m(m + n)
//
// For a target sum S:
//   2m(m + n) = S
//   m(m + n) = S / 2 = s2
//
// So for each m:
//   - m must divide s2
//   - Let k = s2 / m
//   - Then n = k - m
//
// And we must enforce invariants:
//   - m >= 2
//   - n >= 1
//   - n < m
//   - m(m + 1) <= s2   (because n >= 1 ⇒ m(m+n) >= m(m+1))

fn triple(m: u64, n: u64) -> (u64, u64, u64) {
    // Invariant: Euclid's formula requires m > n
    debug_assert!(m > n);
    (
        m * m - n * n,
        2 * m * n,
        m * m + n * n,
    )
}

// Given s2 = sum / 2 and a candidate m,
// try to derive a valid n such that:
//
//   m(m + n) = s2  =>  n = s2/m - m
//
// Enforces invariants:
//   - m divides s2
//   - n = k - m is computable (no underflow)
//   - 1 <= n < m
fn candidate_n(s2: u64, m: u64) -> Option<u64> {
    // Invariant: m must divide s2
    if s2 % m != 0 {
        return None;
    }
    let k = s2 / m;
    // Invariant: n = k - m must be computable and positive
    let n = k.checked_sub(m)?;
    // Invariant: 1 <= n < m
    (n > 0 && n < m).then_some(n)

}

// More functional, pipeline-based approach
fn find_product_iter(sum: u64) -> Option<u64> {
    // Edge invariant: sum must be even (since sum = 2m(m+n))
    (sum % 2 == 0).then_some(())?;
    let s2 = sum / 2;
    (2..)
        // Bound invariant: since n >= 1, m(m+1) <= s2 must hold
        .take_while(|&m| m * (m + 1) <= s2)
        // Search for first m that can produce a valid triple
        .find_map(|m| {
            let n = candidate_n(s2, m)?;
            let (a, b, c) = triple(m, n);
            debug_assert_eq!(a + b + c, sum);
            Some(a * b * c)
        })
}       

// More imperative approach
fn find_product(sum: u64) -> Option<u64> {
    // Edge invariant: sum must be even
    if sum % 2 != 0 {
        return None;
    }
    let s2 = sum / 2_u64;
    for m in 2.. {
        // Bound invariant:
        // Since n >= 1, we must have m(m + 1) <= s2.
        if m * (m + 1) > s2 {
            break;
        }
        // Invariant: m must divide s2
        if s2 % m != 0 {
            continue;
        }
        let k: u64 = s2 / m;
        // Invariant: n = k - m must satisfy 1 <= n < m
        if k <= m || k >= 2 * m {
            continue;
        }
        let n: u64 = k - m;
        let (a, b, c) = triple(m, n);
        debug_assert_eq!(a + b + c, sum);
        return Some(a * b * c);
    }
    None
}

fn main() {
    let result_imp = find_product(1000_u64).unwrap();
    let result_fp = find_product_iter(1000_u64).unwrap();
    assert_eq!(result_imp, result_fp);
    println!("{result_imp}");
}