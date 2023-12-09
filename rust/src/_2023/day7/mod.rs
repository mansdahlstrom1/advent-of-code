use crate::utils;

pub fn main() {
  let input = utils::get_input_file("src/_2023/day7/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

fn pt1(input: &str) -> i32 {
  println!("Input [{}]", input);
  0
}

fn pt2(input: &str) -> i32 {
  println!("Input [{}]", input);
  0
}

#[cfg(test)]
mod tests {
  use crate::utils;

  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day7/example.txt");
    assert_eq!(pt1(&input), 6440);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day7/example.txt");
    assert_eq!(pt2(&input), 0);
  }
}
