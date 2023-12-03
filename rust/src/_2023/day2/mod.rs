use crate::utils;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn main() {
  let input = utils::get_input_file("./src/_2023/day2/input.txt");
  let sum = run(&input);

  println!("Sum of valid ID's: {}", sum);
}

#[derive(Debug)]
struct Round {
  red: i32,
  green: i32,
  blue: i32,
}

impl Round {
  fn from_str(input: &str) -> Self {
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

  fn is_valid(&self) -> bool {
    self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
  }
}

#[derive(Debug)]
struct Game {
  id: i32,
  rounds: Vec<Round>,
}

impl Game {
  fn is_valid(&self) -> bool {
    self.rounds.iter().all(|round| round.is_valid())
  }
}

fn parse_input(input: &str) -> Vec<Game> {
  println!("Parse input");
  let lines: Vec<&str> = input.split('\n').collect();

  let result: Vec<Game> = lines
    .into_iter()
    .filter(|line| !line.is_empty())
    .map(|line| {
      let vec: Vec<&str> = line.split(':').collect();
      let (game_id, round) = match vec.as_slice() {
        [left, round] => {
          let game_id = utils::get_i32_from_string(left);
          (game_id, round)
        }
        _ => panic!("Expected a line in the format 'game_id:round'"),
      };

      let rounds: Vec<Round> = round.split(';').map(Round::from_str).collect();
      Game {
        id: game_id,
        rounds,
      }
    })
    .collect();

  result
}

fn run(input: &str) -> i32 {
  println!("Input: {}", input);
  let parsed_input = parse_input(input);

  let result = parsed_input.iter().fold(
    0,
    |acc, game| if game.is_valid() { acc + game.id } else { acc },
  );
  println!("Parsed input: {:#?}", parsed_input);

  result
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_main() {
    let input = utils::get_input_file("./src/_2023/day2/example.txt");
    assert_eq!(run(&input), 8);
  }
}
