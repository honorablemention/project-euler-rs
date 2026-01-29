

fn max_product_rolling(digits: &[u64], k: usize) -> u64 {
    if k == 0 || digits.len() < k {
        return 0;
    }

    let mut best: u64 = 0;
    let mut prod: u64 = 1;
    let mut zero_count: usize = 0;

    for i in 0..digits.len() {
        let incoming = digits[i];
        if incoming == 0 {
            zero_count += 1;
        } else {
            prod *= incoming;
        }

        if i >= k {
            let outgoing = digits[i - k];
            if outgoing == 0 {
                zero_count -= 1;
            } else {
                prod /= outgoing;
            }
        }

        if i + 1 >= k && zero_count == 0 {
            if prod > best {
                best = prod;
            }
        }
    }
    return best;
}

fn main() {
    let magic_number = concat!(
        "73167176531330624919225119674426574742355349194934",
        "96983520312774506326239578318016984801869478851843",
        "85861560789112949495459501737958331952853208805511",
        "12540698747158523863050715693290963295227443043557",
        "66896648950445244523161731856403098711121722383113",
        "62229893423380308135336276614282806444486645238749",
        "30358907296290491560440772390713810515859307960866",
        "70172427121883998797908792274921901699720888093776",
        "65727333001053367881220235421809751254540594752243",
        "52584907711670556013604839586446706324415722155397",
        "53697817977846174064955149290862569321978468622482",
        "83972241375657056057490261407972968652414535100474",
        "82166370484403199890008895243450658541227588666881",
        "16427171479924442928230863465674813919123162824586",
        "17866458359124566529476545682848912883142607690042",
        "24219022671055626321111109370544217506941658960408",
        "07198403850962455444362981230987879927244284909188",
        "84580156166097919133875499200524063689912560717606",
        "05886116467109405077541002256983155200055935729725",
        "71636269561882670428252483600823257530420752963450",
    );
    let seq_length: usize = 13;
    let digits: Vec<u64> = magic_number
        .bytes()
        .map(|b| (b - b'0') as u64)
        .collect();

    // Imperative style
    // let mut best: u64 = 0;
    // for w in digits.windows(seq_length) {
    //     let mut prod: u64 = 1;

    //     for &d in w {
    //         if d == 0 {
    //             prod = 0;
    //             break;
    //         }
    //         prod *= d;
    //     }

    //     if prod > best {
    //         best = prod;
    //     }
    // }

    // More functional
    // let best = digits
    //     .windows(seq_length)
    //     .filter_map(|w| {
    //         w.iter().try_fold(1_u64, |acc, &d| {
    //             if d == 0 {
    //                 None
    //             } else {
    //                 Some(acc * d)
    //             }
    //         })
    //     })
    //     .max()
    //     .unwrap_or(0);

    // More performant for large seq_length
    let best = max_product_rolling(&digits, seq_length);

    println!("{best}");
}