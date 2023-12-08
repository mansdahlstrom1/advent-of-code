use crate::utils;

mod almanac;
mod custom_range;

use self::almanac::Almanac;

pub fn main() {
  let input = utils::get_input_file("src/_2023/day5/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

fn pt1(input: &str) -> i64 {
  let almanac: Almanac = Almanac::new(input);

  let mut locations = Vec::<i64>::new();

  for seed in &almanac.seeds {
    let location = almanac.get_location_by_seed(*seed);
    locations.push(location);
  }

  locations.sort();

  locations[0]
}

fn pt2(input: &str) -> i64 {
  let almanac: Almanac = Almanac::new(input);
  let mut locations = Vec::<i64>::new();

  for (index, range_of_seeds) in almanac.seeds_as_range.iter().enumerate() {
    println!("Range: {:?}", range_of_seeds);
    let mut range_location = Vec::<i64>::new();

    for seed in range_of_seeds.clone() {
      let location = almanac.get_location_by_seed(seed);
      range_location.push(location);
    }

    range_location.sort();
    locations.push(range_location[0]);
    println!(
      "Done with range {} / {}, result: {}",
      index,
      almanac.seeds_as_range.len(),
      range_location[0]
    );
  }

  // println!("Locations: {:#?}", locations);

  locations.sort();
  locations[0]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day5/example.txt");
    assert_eq!(pt1(&input), 35);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day5/example.txt");
    assert_eq!(pt2(&input), 46);
  }
}
