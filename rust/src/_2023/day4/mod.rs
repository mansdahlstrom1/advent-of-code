use crate::utils::{self, exit_with_message};

mod card;
use self::card::Card;

pub fn main() {
  let input = utils::get_input_file("src/_2023/day4/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
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

fn parse_input_to_card(input: &str) -> Vec<Card> {
  let cards = input.split('\n');
  let mut vec: Vec<Card> = Vec::new();

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

    let card = Card::new(card, winning_numbers, my_numbers, false)
      .calculate_card_score()
      .to_owned();
    vec.push(card)
  }

  vec
}

fn pt1(input: &str) -> i32 {
  let mut total_score = 0;
  let cards: Vec<Card> = parse_input_to_card(input);

  for card in cards {
    total_score += card.score.unwrap();
    println!(
      "{}: Score: {} ->  {:?} | {:?}",
      card.id,
      card.score.unwrap(),
      card.winning_numbers,
      card.my_numbers,
    );
  }
  total_score
}

fn duplicated_copies(
  original_list: Vec<Card>,
  matched_cards: Vec<Card>,
  unmatched_cards: Vec<Card>,
) -> Vec<Card> {
  let mut new_copies: Vec<Card> = Vec::new();

  // search the unmatched cards
  for card in &unmatched_cards {
    let m_indices = card.matches.unwrap();
    for i in 0..m_indices {
      let matching_card = original_list.iter().find(|c| c.id == card.id + i + 1);

      if let Some(c) = matching_card {
        new_copies.push(c.clone());
      }
    }
  }

  // Add the cards we just went over to the list of matched cards...
  let matches_cards = [matched_cards, unmatched_cards].concat();

  if !new_copies.is_empty() {
    println!("Running next itteration...");
    duplicated_copies(original_list, matches_cards, new_copies)
  } else {
    [matches_cards, new_copies].concat()
  }
}

fn pt2(input: &str) -> usize {
  let original_cards: Vec<Card> = parse_input_to_card(input);
  let cards_and_copies = duplicated_copies(original_cards.clone(), Vec::new(), original_cards);

  cards_and_copies.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pt1() {
    let input = utils::get_input_file("src/_2023/day4/example.txt");
    assert_eq!(pt1(&input), 13);
  }

  #[test]
  fn test_pt2() {
    let input = utils::get_input_file("src/_2023/day4/example.txt");
    assert_eq!(pt2(&input), 30);
  }
}
