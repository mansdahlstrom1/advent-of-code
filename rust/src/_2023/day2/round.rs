use crate::utils;

use super::constants::{MAX_BLUE, MAX_GREEN, MAX_RED};

#[derive(Debug)]
pub struct Round {
  pub red: i32,
  pub green: i32,
  pub blue: i32,
}

impl Round {
  pub fn from_str(input: &str) -> Self {
    let colors: Vec<&str> = input.split(',').collect();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color in colors {
      let amount = utils::get_i32_from_string(color);
      if color.contains("blue") {
        blue += amount;
      } else if color.contains("green") {
        green += amount;
      } else if color.contains("red") {
        red += amount;
      }
    }

    Round { red, green, blue }
  }

  pub fn is_valid(&self) -> bool {
    self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
  }
}
