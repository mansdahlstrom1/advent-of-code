use crate::utils;

mod almanac;
mod custom_range;

use self::almanac::Almanac;

pub fn main() {
  let input = utils::get_input_file("src/_2023/day5/input.txt");
  let results_pt1 = pt1(&input);
  // let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  // println!("Part 2: {}", results_pt2);
}

fn pt1(input: &str) -> i64 {
  let almanac = Almanac::new(input);

  let mut locations = Vec::<i64>::new();

  for seed in &almanac.seeds {
    let location = almanac.get_location_by_seed(*seed);
    println!("========\n");
    locations.push(location);
  }

  println!("Locations: {:#?}", locations);
  locations.sort();

  locations[0]
}

// fn pt2(input: &str) -> i32 {
//   0
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day5/example.txt");
    assert_eq!(pt1(&input), 35);
  }

  // #[test]
  // fn test_pt2() {
  //   let input = utils::get_input_file("src/_2023/day5/example.txt");
  //   assert_eq!(pt2(&input), 0);
  // }
}
