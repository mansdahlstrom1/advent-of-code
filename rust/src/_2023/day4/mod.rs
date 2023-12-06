use crate::utils::{self, exit_with_message};

pub fn main() {
  let input = utils::get_input_file("src/_2023/day4/input.txt");
  let results_pt1 = pt1(&input);
  // let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  // println!("Part 2: {}", results_pt2);
}

fn convert_to_numeric_array(numbers: &str) -> Vec<i32> {
  let result = numbers
    .split_whitespace()
    .map(|n| n.trim().parse::<i32>())
    .collect();

  match result {
    Ok(v) => v,
    Err(_) => exit_with_message("Failed to parse numbers into an array"),
  }
}

fn calculate_card_score(winning_numbers: &Vec<i32>, my_numbers: &Vec<i32>) -> i32 {
  let mut score = 0;
  for number in my_numbers {
    if winning_numbers.contains(number) {
      if score == 0 {
        score = 1;
      } else {
        score *= 2;
      }
    }
  }
  score
}

fn pt1(input: &str) -> i32 {
  let mut total_score = 0;
  let cards = input.split('\n');
  for card in cards {
    if card.is_empty() {
      continue;
    }
    let parts: Vec<&str> = card.split(':').collect();
    let (card, _numbers) = (parts[0], parts[1]);
    let numbers: Vec<&str> = _numbers.split('|').collect();
    let (winning_numbers, my_numbers) = (numbers[0], numbers[1]);
    let winning_numbers = convert_to_numeric_array(winning_numbers);
    let my_numbers = convert_to_numeric_array(my_numbers);

    let card_score = calculate_card_score(&winning_numbers, &my_numbers);
    total_score += card_score;
    println!(
      "{}: Score: {} ->  {:?} | {:?}",
      card, card_score, winning_numbers, my_numbers,
    );
  }
  total_score
}
// fn pt2(&input: &str) -> i32 {
//   0
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day4/example.txt");
    assert_eq!(pt1(&input), 13);
  }

  // #[test]
  // fn test_pt2() {
  //   let input = utils::get_input_file("src/_2023/day3/example.txt");
  //   assert_eq!(pt2(&input), 467835);
  // }
}
