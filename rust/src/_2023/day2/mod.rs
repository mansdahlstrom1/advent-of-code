use crate::utils;

mod constants;
mod game;
mod round;
use self::{game::Game, round::Round};

pub fn main() {
  let input = utils::get_input_file("./src/_2023/day2/input.txt");
  let sum_pt1 = pt1(&input);
  println!("[pt1] - Sum of valid ID's: {}", sum_pt1);

  let sum_pt2 = pt2(&input);
  println!("[pt2] - Sum of valid ID's: {}", sum_pt2);
}

fn parse_input(input: &str) -> Vec<Game> {
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

fn pt1(input: &str) -> i32 {
  let games = parse_input(input);

  games.iter().fold(
    0,
    |acc, game| if game.is_valid() { acc + game.id } else { acc },
  )
}

fn pt2(input: &str) -> i32 {
  let games = parse_input(input);
  games
    .iter()
    .fold(0, |acc, game| acc + game.get_game_power())
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("./src/_2023/day2/example.txt");
    assert_eq!(pt1(&input), 8);
  }
  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("./src/_2023/day2/example.txt");
    assert_eq!(pt2(&input), 2286);
  }
}
