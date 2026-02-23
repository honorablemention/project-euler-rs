use std::fs::read_to_string;
use std::error::Error;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InputNumber {
    input: Vec<String>,
}

struct Valid {
    digits: Vec<Vec<u8>>,
    len: usize,
}

type ValidateResult = std::result::Result<Valid, String>;

fn validate(input: Vec<String>) -> ValidateResult {
    if input.is_empty() {
        return Err("Empty rows".to_string());
    }
    let len = input[0].len();
    let ok = 
        input
            .iter()
            .all(|s| s.len() == len);
    if !ok {
        return Err("Data is not valid; ragged array".to_string());
    }
    let digits = 
        input
            .into_iter()
            .map(|s| s.into_bytes())
            .collect();
    Ok(Valid { digits, len })
}

fn get_from_file(p: &Path) -> Result<InputNumber, Box<dyn Error>> {
    let json = read_to_string(p)?;
    let input: InputNumber = serde_json::from_str(&json)?;
    Ok(input)
}

fn digit_at(row: &[u8], idx: usize) -> u32 {
    (row[idx] - b'0') as u32
}

fn sum_column(input: &[Vec<u8>], col_idx: usize) -> u32 {
    input
        .iter()
        .map(|row| digit_at(row, col_idx))
        .sum()
}

fn next_carry(carry: u32, col_sum: u32) -> u32 {
    (carry + col_sum) / 10
}

fn get_carry_val(n: usize, input: &[Vec<u8>]) -> u32 {
    let len = input[0].len();
    (n..len)
        .rev()
        .fold(0_u32, |carry, col_idx| {
            let col_sum = sum_column(input, col_idx);
            next_carry(carry, col_sum)
        })
}

fn sum_left_n(input: &[Vec<u8>], n: usize) -> u64 {
    input
        .iter()
        .map(|row| {
            row
                .iter()
                .take(n)
                .fold(0u64, |acc, &b| acc * 10 + (b - b'0') as u64)
        })
        .sum()
}

fn get_first_n_from_sum_of(n: usize, input: &[Vec<u8>]) -> String {
    let carry = get_carry_val(n, input) as u64;
    let sum_high = sum_left_n(input, n);
    let total = sum_high + carry;
    total.to_string().chars().take(n).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read number from .json file
    let input_number = get_from_file(Path::new("src/number.json"))?;
    // Validate
    let valid = validate(input_number.input)
        .map_err(|e| format!("Validation error: {e}"))?;
    // Sum
    let n = 10;
    if n > valid.len {
        return Err(format!(
            "Requested digits ({n}) exceeds input length ({})",
            valid.len
        )
        .into());
    }
    let sum = get_first_n_from_sum_of(n, &valid.digits);
    println!("{sum}");
    Ok(())
}
