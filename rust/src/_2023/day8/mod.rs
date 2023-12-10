use std::collections::HashMap;

use crate::utils::{self, exit_with_message};

pub fn main() {
  let input = utils::get_input_file("src/_2023/day8/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

fn pt1(input: &str) -> i32 {
  let (direction, map) = parse_map(input);

  let mut current_index = 0;
  let mut current_location = String::from("AAA");
  let get_next_index = |index: usize| -> usize {
    if (index + 1) > direction.len() - 1 {
      0
    } else {
      index + 1
    }
  };

  println!("Starting at: {}", current_location);
  let mut steps = 0;
  while current_location != "ZZZ" {
    steps += 1;
    let direction = direction[current_index];

    current_location = get_next_location(current_index, current_location, direction, &map);
    current_index = get_next_index(current_index);
  }

  steps
}

fn pt2(input: &str) -> i32 {
  println!("{}", input);
  0
}

fn get_next_location(
  current_index: usize,
  current_location: String,
  direction: char,
  map: &HashMap<String, Options>,
) -> String {
  print!(
    "{}\tAt: {}, going {} => ",
    current_index, current_location, direction
  );
  let options = map.get(&current_location).unwrap();

  match direction {
    'L' => options.l.to_string(),
    'R' => options.r.to_string(),
    _ => exit_with_message(format!("Unknown direction {}", direction).as_str()),
  }
}

#[derive(Debug, Clone)]
struct Options {
  l: String,
  r: String,
}

fn parse_map(input: &str) -> (Vec<char>, HashMap<String, Options>) {
  let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

  let direction: Vec<char> = lines[0].chars().collect();

  let mut map = HashMap::new();

  for line in lines.iter().skip(1) {
    let sides: Vec<&str> = line.split(" = ").collect();
    let left_right = sides[1].replace(['(', ')'], "");
    let left_right: Vec<&str> = left_right.split(", ").collect();

    map.insert(
      String::from(sides[0]),
      Options {
        l: left_right[0].to_string(),
        r: left_right[1].to_string(),
      },
    );
  }

  (direction, map)
}

#[cfg(test)]
mod tests {
  use crate::utils;

  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day8/example.txt");
    assert_eq!(pt1(&input), 2);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day8/example_2.txt");
    assert_eq!(pt2(&input), 6);
  }
}
