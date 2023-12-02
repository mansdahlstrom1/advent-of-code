use std::collections::HashMap;

use crate::utils;

#[derive(Debug)]
struct Value {
  index: usize,
  value: u32,
}

pub fn main() {
  let input = utils::get_input_file("./src/_2023/day1/input.txt");
  let lines = input.split('\n');
  let mut result: i32 = 0;

  let map = HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
  ]);

  for line in lines {
    if line.is_empty() {
      continue;
    }

    // Create a new vector to hold all our found values
    let mut vector: Vec<Value> = Vec::new();

    // Iterate over each character in the line and store out the digits + their index
    for (index, c) in line.chars().enumerate() {
      if c.is_ascii_digit() {
        let value = c.to_digit(10).unwrap();
        vector.push(Value { index, value });
      }
    }

    // Go through each word in the map and see if it exists in the line
    map.iter().for_each(|(word, number)| {
      if line.contains(word) {
        // find all the indices of where the word appears in the line
        let indices: Vec<_> = line.match_indices(word).collect();
        for (index, _) in indices {
          vector.push(Value {
            index,
            value: *number,
          });
        }
      }
    });

    // Sort the list by index
    vector.sort_by(|a, b| a.index.cmp(&b.index));

    let first = vector.first().unwrap();
    let last = vector.last().unwrap();
    let s = vector
      .iter()
      .fold(String::new(), |acc, x| acc + &x.value.to_string());

    let digits = format!("{}{}", first.value, last.value);
    let value = digits.parse::<i32>().unwrap();
    result += value;
    println!("{} => {} => {}", line, s, digits);
  }

  println!("Result: {}", result)
}
