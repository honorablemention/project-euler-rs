use std::error::Error;
use std::path::Path;
use serde::{Deserialize, Serialize};

use euler013::get_first_n_from_sum_of;

#[derive(Serialize, Deserialize)]
struct InputNumber {
    input: Vec<String>,
}

fn get_from_file(p: &Path) -> Result<InputNumber, Box<dyn Error>> {
    let json = std::fs::read_to_string(p)?;
    let input: InputNumber = serde_json::from_str(&json)?;
    Ok(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read number from .json file
    let input_number = get_from_file(Path::new("src/number.json"))?;
    let sum = get_first_n_from_sum_of(10, &input_number.input)
        .map_err(|e| format!("Validation error: {e}"))?;
    println!("{sum}");
    Ok(())
}
