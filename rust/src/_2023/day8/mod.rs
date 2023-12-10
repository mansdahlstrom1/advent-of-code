mod map;
use std::collections::HashMap;

use map::Map;

use crate::utils::{self};
use num::integer::lcm;

pub fn main() {
  let input = utils::get_input_file("src/_2023/day8/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);
  let results_pt2_lcm = pt2_using_lcm(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
  println!("Part 2 (LCM): {}", results_pt2_lcm);
}

fn pt1(input: &str) -> i32 {
  let mut map = Map::new(input);
  let mut current_location: String = String::from("AAA");
  println!("Starting at: {}", current_location);

  let mut steps = 0;
  while current_location != "ZZZ" {
    steps += 1;
    current_location = map.get_next_location(current_location);
    map.increment_index();
  }

  steps
}

// Brute force way, never finished it...
// but would take 21165830176709 iterations
fn pt2(input: &str) -> i32 {
  let mut map = Map::new(input);

  let starting_positions: Vec<String> = map.get_starting_positions();
  println!("Following path paths {:?}", starting_positions);

  let mut steps = 0;
  let mut locations: Vec<String> = starting_positions;
  // let mut found_z_count: Vec<i32> = vec![0; locations.len()];

  loop {
    steps += 1;
    let next_locations: Vec<String> = locations
      .iter()
      .map(|l| map.get_next_location(l.to_string()))
      .collect();

    if next_locations.iter().all(|l| l.ends_with('Z')) {
      break;
    }

    // DEBUG LOGGING
    // for (i, l) in next_locations.iter().enumerate() {
    //   if l.ends_with('Z') {
    //     found_z_count[i] += 1;
    //     println!(
    //       "Iterator {} found a Z at step: {}",
    //       i,
    //       steps / found_z_count[i]
    //     );
    //   }
    // }

    // Increment the index for the next iteration
    locations = next_locations;
    map.increment_index();
  }

  steps
}

// Non brute force way with help of reddit...
fn pt2_using_lcm(input: &str) -> i32 {
  let mut map = Map::new(input);

  let starting_positions: Vec<String> = map.get_starting_positions();
  let locations: Vec<String> = starting_positions;

  let mut cycle_count = HashMap::<String, usize>::new();

  // We assume the length of the cycle is the same every time...
  for l in locations.iter() {
    let mut current_location = l.to_string();
    let mut steps = 0;

    while !current_location.ends_with('Z') {
      println!("Current location: {}", current_location);
      current_location = map.get_next_location(current_location);
      map.increment_index();
      steps += 1;
    }

    cycle_count.insert(l.to_string(), steps);
  }

  println!("List of loop: {:#?}", cycle_count);
  let counts = cycle_count.values().cloned().collect::<Vec<_>>();

  // List of loop: {
  //   "AAA": 21409,
  //   "XDA": 14363,
  //   "HPA": 19783,
  //   "CFA": 16531,
  //   "HJA": 19241,
  //   "XSA": 15989,
  // }
  // This LCM bellow did not work, but the one from the internet did...
  // https://www.calculator.net/lcm-calculator.html?numberinputs=15989%2C+19783%2C+14363%2C+21409%2C+16531%2C+19241&x=Calculate
  let lcm = counts.iter().fold(1, |prev, &x| lcm(prev, x));

  lcm as i32
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

  #[test]
  fn test_pt2_lcm() {
    let input = utils::get_input_file("src/_2023/day8/example_2.txt");
    assert_eq!(pt2_using_lcm(&input), 6);
  }

  #[test]
  fn test_all() {
    let next_locations = ["12Z", "12Z", "12Z", "12Z", "12Z"];
    assert!(next_locations.iter().all(|l| l.ends_with('Z')));
  }
}
