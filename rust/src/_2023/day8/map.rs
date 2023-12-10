use std::collections::HashMap;

use crate::utils::exit_with_message;

#[derive(Debug, Clone)]
pub struct Options {
  l: String,
  r: String,
}

pub struct Map {
  map: HashMap<String, Options>,
  direction: Vec<char>,
  current_index: usize,
}

impl Map {
  pub fn new(input: &str) -> Self {
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
          l: left_right[0].trim().to_string(),
          r: left_right[1].trim().to_string(),
        },
      );
    }

    Map {
      map,
      direction,
      current_index: 0,
    }
  }

  pub fn get_starting_positions(&self) -> Vec<String> {
    self
      .map
      .keys()
      .filter(|k| k.ends_with('A'))
      .map(|s| s.to_string())
      .collect::<Vec<String>>()
  }

  pub fn increment_index(&mut self) {
    if (self.current_index + 1) > self.direction.len() - 1 {
      self.current_index = 0;
    } else {
      self.current_index += 1;
    }
  }

  pub fn get_direction(&self) -> char {
    self.direction[self.current_index]
  }

  pub fn get_next_location(&self, current_location: String) -> String {
    let direction = self.get_direction();
    // print!(
    //   "{}\tAt: {}, going {} => ",
    //   self.current_index, current_location, direction
    // );
    let options = self.map.get(&current_location).unwrap();

    let next = match direction {
      'L' => options.l.to_string(),
      'R' => options.r.to_string(),
      _ => exit_with_message(format!("Unknown direction {}", direction).as_str()),
    };

    if next == current_location {
      exit_with_message(format!("Stuck at {}", current_location).as_str());
    }

    // println!("{}", next);
    next
  }
}
