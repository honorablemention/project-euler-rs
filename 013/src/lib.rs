pub fn get_first_n_from_sum_of(n: usize, input: &[String]) -> Result<String, String> {
    if input.is_empty() {
        return Err("Empty rows".to_string());
    }
    let len = input[0].len();
    if n > len {
        return Err(format!(
            "Requested digits ({n}) exceeds input length ({len})"
        ));
    }

    let mut sum_high: u64 = 0;
    let mut col_sums = vec![0_u32; len - n];

    for row in input {
        if row.len() != len {
            return Err("Data is not valid; ragged array".to_string());
        }
        let bytes = row.as_bytes();

        let mut acc = 0_u64;
        for (i, &b) in bytes.iter().enumerate() {
            let digit = (b - b'0') as u64;
            if i < n {
                acc = acc * 10 + digit;
            } else {
                col_sums[i - n] += digit as u32;
            }
        }
        sum_high += acc;
    }

    let carry = col_sums
        .iter()
        .rev()
        .fold(0_u32, |carry, &col_sum| next_carry(carry, col_sum));

    let total = sum_high + carry as u64;
    let total_str = total.to_string();
    Ok(total_str[..n].to_string())
}

fn next_carry(carry: u32, col_sum: u32) -> u32 {
    (carry + col_sum) / 10
}
