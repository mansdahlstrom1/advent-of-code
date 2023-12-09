use crate::utils::{self};

pub fn main() {
  let input = utils::get_input_file("src/_2023/day6/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

struct Race {
  time: i64,
  distance: i64,
}

fn parse_input(input: &str) -> Vec<Race> {
  println!("Parsing input [{}]", input);
  let lines: Vec<Vec<i64>> = input
    .split('\n')
    .filter(|l| !&l.is_empty())
    .map(|l| {
      l.replace("Time:", "")
        .replace("Distance:", "")
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
    })
    .collect();

  let mut races = Vec::new();

  for x in 0..lines[0].len() {
    races.push(Race {
      time: lines[0][x],
      distance: lines[1][x],
    });
  }

  races
}

fn calculate_wins(races: Vec<Race>) -> i64 {
  let mut wins_per_rounds = Vec::<i64>::new();

  for race in races {
    println!("Record is: {}mm in {}ms", race.time, race.distance);
    let mut winning_rounds = 0;
    for i in 0..race.time {
      let time_left = race.time - i;
      let mm_per_ms = i;

      let distance = time_left * mm_per_ms;
      // println!("After {}ms, the distance is {}mm", i, distance);
      if distance > race.distance {
        winning_rounds += 1;
      }
    }
    wins_per_rounds.push(winning_rounds);
  }

  wins_per_rounds.iter().product()
}

fn pt1(input: &str) -> i64 {
  let races = parse_input(input);
  calculate_wins(races)
}

fn pt2(input: &str) -> i64 {
  let races = parse_input(input.replace(' ', "").as_str());
  calculate_wins(races)
}

#[cfg(test)]
mod tests {
  use crate::utils;

  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day6/example.txt");
    assert_eq!(pt1(&input), 288);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day6/example.txt");
    assert_eq!(pt2(&input), 71503);
  }
}
