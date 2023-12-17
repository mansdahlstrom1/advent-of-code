use crate::utils::{self};

pub fn main() {
  let input = utils::get_input_file("src/_2023/day9/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

fn pt1(input: &str) -> i32 {
  let dataset = get_dataset(input);
  let mut sum = 0;
  for history in dataset {
    sum += extrapolate_history(history);
  }

  sum
}

fn pt2(input: &str) -> i32 {
  let dataset = get_dataset(input);
  let mut sum = 0;
  for mut history in dataset {
    history.reverse();
    sum += extrapolate_history(history);
  }

  sum
}

fn get_dataset(input: &str) -> Vec<Vec<i32>> {
  let mut dataset: Vec<Vec<i32>> = Vec::new();

  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    let history: Vec<i32> = line
      .split_whitespace()
      .map(|n| n.parse::<i32>().unwrap())
      .collect();
    dataset.push(history);
  }

  dataset
}

fn extrapolate_history(history: Vec<i32>) -> i32 {
  let mut next_value = *history.last().unwrap();
  let mut current_level = history;

  println!("########################");
  println!("Running {:?}", current_level,);
  loop {
    println!("Starting loop {:?}", current_level);
    let mut next = Vec::new();
    for (i, value) in current_level.iter().enumerate() {
      if i + 1 == current_level.len() {
        continue;
      }
      let diff = current_level[i + 1] - value;
      println!("{} - {} = {}", current_level[i + 1], value, diff);
      next.push(diff);
    }
    println!("Next {:?}", next);
    let last_value = next.last().unwrap();
    next_value += last_value;
    current_level = next.clone();

    if current_level.iter().all(|v| v == &0) {
      println!(
        "Breaking {:?}, should loop: {}, answer ",
        current_level,
        current_level.iter().all(|v| {
          println!("{} != 0", v);
          v != &0
        })
      );
      break;
    }
  }

  println!("Answer: {}", next_value);

  next_value
}

#[cfg(test)]
mod tests {
  use crate::utils;

  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day9/example.txt");
    assert_eq!(pt1(&input), 114);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day9/example.txt");
    assert_eq!(pt2(&input), 2,);
  }
}
