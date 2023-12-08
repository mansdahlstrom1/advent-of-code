use std::ops::Range;

use crate::utils::exit_with_message;

#[derive(Debug)]
pub struct CustomRange {
  pub source: Range<i64>,
  pub destination: Range<i64>,
  pub diff: i64,
}

impl CustomRange {
  pub fn new(numbers: &str) -> CustomRange {
    let parts: Vec<i64> = numbers
      .split_whitespace()
      .map(|n| {
        let value = n.trim().parse::<i64>();
        match value {
          Ok(v) => v,
          Err(_) => {
            println!("Failed to parse this line [{}]", n);
            exit_with_message("failed_to_parse_numbers_into_an_array");
          }
        }
      })
      .collect();

    CustomRange {
      source: parts[1]..parts[1] + parts[2],
      destination: parts[0]..parts[0] + parts[2],
      diff: parts[0] - parts[1],
    }
  }
}
