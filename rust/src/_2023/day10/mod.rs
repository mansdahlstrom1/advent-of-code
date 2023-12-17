use std::collections::HashMap;

use crate::utils::{self, exit_with_message};

/**
 * | is a vertical pipe connecting north and south.
 * - is a horizontal pipe connecting east and west.
 * L is a 90-degree bend connecting north and east.
 * J is a 90-degree bend connecting north and west.
 * 7 is a 90-degree bend connecting south and west.
 * F is a 90-degree bend connecting south and east.
 * . is ground; there is no pipe in this tile.
 * S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */

// Direction east, west, north, south
// scan 3x3

//  - L | F 7
//  7 S - 7 |
//  L | 7 | |
//  - L - J |
//  L | - J F

// going west   => S => 7 => L => | => cannot go L to | => Dead end
// going east   => S => - => 7 => | => J =>  => - => L => | => S => Back to start again, loop completed.
// going north  => S =>  L => | => cannot go L to | => Dead end
// going south  => S => | => L => - => J => | => 7 => - => S => Back to start again, loop completed.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  East,
  West,
  North,
  South,
  None,
}

#[derive(Debug)]
struct Cord {
  x: usize,
  y: usize,
}

/**
 * | is a vertical pipe connecting north and south.
 * - is a horizontal pipe connecting east and west.
 * L is a 90-degree bend connecting north and east.
 * J is a 90-degree bend connecting north and west.
 * 7 is a 90-degree bend connecting south and west.
 * F is a 90-degree bend connecting south and east.
 * . is ground; there is no pipe in this tile.
 * S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */
fn match_symbol(direction: &Direction, symbol: char) -> Direction {
  match direction {
    Direction::East => {
      if symbol == '-' {
        return Direction::East;
      }
      if symbol == 'J' {
        return Direction::North;
      }
      if symbol == '7' {
        return Direction::South;
      }
    }
    Direction::West => {
      if symbol == '-' {
        return Direction::West;
      }
      if symbol == 'L' {
        return Direction::North;
      }
      if symbol == 'F' {
        return Direction::South;
      }
    }
    Direction::North => {
      if symbol == '|' {
        return Direction::North;
      }
      if symbol == 'F' {
        return Direction::East;
      }

      if symbol == '7' {
        return Direction::West;
      }
    }
    Direction::South => {
      if symbol == '|' {
        return Direction::South;
      }
      if symbol == 'J' {
        return Direction::West;
      }
      if symbol == 'L' {
        return Direction::East;
      }
    }
    Direction::None => exit_with_message("Got Direction None, something has gone wrong..."),
  }

  Direction::None
}

fn follow_pipe(map: &Vec<Vec<char>>, direction: &Direction, cord: &Cord) -> (bool, i32) {
  let mut x = cord.x;
  let mut y = cord.y;
  let mut steps = 0;
  let mut current_direction = *direction;

  loop {
    // Make sure we are not out of bounds.
    if (current_direction == Direction::West && x == 0)
      || (current_direction == Direction::North && y == 0)
      || (current_direction == Direction::East && x + 1 >= map[0].len())
      || (current_direction == Direction::South && y + 1 >= map.len())
    {
      println!(
        "Out of bounds at x:{}, y:{}, direction: {:?}",
        x, y, current_direction
      );
      return (false, steps);
    }

    match current_direction {
      Direction::East => x += 1,
      Direction::West => x -= 1,
      Direction::North => y -= 1,
      Direction::South => y += 1,
      Direction::None => exit_with_message("Got Direction None, something has gone wrong..."),
    }

    steps += 1;

    let symbol = map[y][x];
    println!("Moving to x:{}, y:{}, symbol is: {}", x, y, symbol);

    // If we are back to the starting spot = S, then we have completed a loop.
    if symbol == 'S' {
      println!("Loop completed at x:{}, y:{} after {} steps", x, y, steps);
      return (true, steps);
    }

    let new_direction = match_symbol(&current_direction, symbol);
    println!("New direction is {:?}", new_direction);

    // Break if the next symbol is a dead end.
    if new_direction == Direction::None {
      println!("Dead end at x:{}, y:{}", x, y);
      return (false, steps);
    }

    current_direction = new_direction;
  }
}

pub fn main() {
  let input = utils::get_input_file("src/_2023/day10/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

fn get_map(input: &str) -> Vec<Vec<char>> {
  let mut map: Vec<Vec<char>> = Vec::new();

  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    let row: Vec<char> = line.chars().collect();
    map.push(row);
  }

  map
}

fn find_starting_cords(map: &[Vec<char>]) -> Cord {
  let mut x = 0;
  let mut y = 0;

  for (i, row) in map.iter().enumerate() {
    for (j, symbol) in row.iter().enumerate() {
      if symbol == &'S' {
        x = j;
        y = i;
      }
    }
  }

  Cord { x, y }
}

fn pt1(input: &str) -> i32 {
  let map: Vec<Vec<char>> = get_map(input);
  let starting_cords = find_starting_cords(&map);

  let mut results = HashMap::<Direction, (bool, i32)>::new();

  println!("Starting map {:#?}", input);
  println!("Starting cords {:?}", starting_cords);
  for direction in [
    Direction::East,
    Direction::West,
    Direction::North,
    Direction::South,
  ] {
    println!("Starting direction {:?}", direction);
    let res = follow_pipe(&map, &direction, &starting_cords);
    results.insert(direction, res);
  }

  println!("{:#?}", results);

  for (_, (is_complete, steps)) in results {
    if is_complete {
      return steps / 2;
    }
  }

  0
}

fn pt2(input: &str) -> i32 {
  let map: Vec<Vec<char>> = get_map(input);
  let starting_cords = find_starting_cords(&map);
  println!("Starting map {:#?}", input);
  println!("Starting cords {:?}", starting_cords);

  0
}

#[cfg(test)]
mod tests {
  use crate::utils;

  use super::*;

  #[test]
  fn test_pt1() {
    let input_1 = utils::get_input_file("src/_2023/day10/example_1.txt");
    assert_eq!(pt1(&input_1), 4);

    let input_2 = utils::get_input_file("src/_2023/day10/example_2.txt");
    assert_eq!(pt1(&input_2), 8);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day10/example.txt");
    assert_eq!(pt2(&input), 0);
  }
}
