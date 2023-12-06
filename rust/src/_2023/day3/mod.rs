use crate::utils;
use regex::{self};

pub fn main() {
  let input = utils::get_input_file("src/_2023/day3/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

pub fn pt1(input: &str) -> i32 {
  let mut sum = 0;
  let lines: Vec<&str> = input.split('\n').collect();

  let converted_lines: Vec<Vec<String>> = lines
    .iter()
    .filter(|line: &&&str| !line.is_empty())
    .map(|line| convert_to_array(line))
    .collect();

  let re = regex::Regex::new(r"[^\d.]").unwrap();
  for (line_index, _) in converted_lines.iter().enumerate() {
    let matches = re.find_iter(lines[line_index]);

    for m in matches {
      let grid = get_grid(&converted_lines, line_index, m.start());
      let part_numbers = find_part_number(grid);
      sum += part_numbers.iter().sum::<i32>();
    }
  }

  sum
}

pub fn pt2(input: &str) -> i32 {
  let mut sum = 0;
  let lines: Vec<&str> = input.split('\n').collect();

  let converted_lines: Vec<Vec<String>> = lines
    .iter()
    .filter(|line: &&&str| !line.is_empty())
    .map(|line| convert_to_array(line))
    .collect();

  let re = regex::Regex::new(r"[^\d.]").unwrap();
  for (line_index, _) in converted_lines.iter().enumerate() {
    let matches: regex::Matches<'_, '_> = re.find_iter(lines[line_index]);

    for m in matches {
      let grid = get_grid(&converted_lines, line_index, m.start());
      let found_part_numbers = find_part_number(grid);

      println!("m: {}", m.as_str());
      if m.as_str() == "*" && found_part_numbers.len() == 2 {
        sum += found_part_numbers.iter().product::<i32>();
      }
    }
  }

  sum
}

fn convert_to_array(line: &str) -> Vec<String> {
  let re = regex::Regex::new(r"[0-9]+").unwrap();

  let array = line.chars().collect::<Vec<char>>();
  let matches: Vec<_> = re.find_iter(line).collect();

  array
    .iter()
    .enumerate()
    .map(|(i, char)| {
      let mut return_string = char.to_string();
      for m in matches.iter() {
        if m.range().contains(&i) {
          return_string = m.as_str().to_owned();
        }
      }
      return_string
    })
    .collect()
}

fn get_grid(converted_lines: &[Vec<String>], line_index: usize, index: usize) -> Vec<Vec<&String>> {
  let top_line: &Vec<&String> = &converted_lines[line_index - 1]
    .iter()
    .take(index + 2)
    .skip(index - 1)
    .collect();
  let middle_line: &Vec<&String> = &converted_lines[line_index]
    .iter()
    .take(index + 2)
    .skip(index - 1)
    .collect();
  let bottom_line: &Vec<&String> = &converted_lines[line_index + 1]
    .iter()
    .take(index + 2)
    .skip(index - 1)
    .collect();
  vec![top_line.clone(), middle_line.clone(), bottom_line.clone()]
}

fn find_part_number(grid: Vec<Vec<&String>>) -> Vec<i32> {
  let mut found_part_numbers: Vec<(usize, i32)> = Vec::new();

  // Go through each adjacent line (y => -1, 0, 1)
  for (current_line_index, line) in grid.iter().enumerate() {
    // Check the adjacent number (x => -1, 0, 1)
    for point in line.iter() {
      let part_number: Result<i32, std::num::ParseIntError> = point.parse::<i32>();
      if let Ok(part_number) = part_number {
        // Keep track of duplicates since we've duplicated the numbers in convert_to_array
        if found_part_numbers.contains(&(current_line_index, part_number)) {
          continue;
        }
        found_part_numbers.push((current_line_index, part_number));
      }
    }
  }

  found_part_numbers
    .iter()
    .map(|(_, part_number)| *part_number)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day3/example.txt");
    assert_eq!(pt1(&input), 4361);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day3/example.txt");
    assert_eq!(pt2(&input), 467835);
  }
}
