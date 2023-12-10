use crate::utils::{self};

mod hand;
use hand::Hand;

pub fn main() {
  let input = utils::get_input_file("src/_2023/day7/input.txt");
  let results_pt1 = pt1(&input);
  let results_pt2 = pt2(&input);

  println!("Part 1: {}", results_pt1);
  println!("Part 2: {}", results_pt2);
}

fn pt1(input: &str) -> i32 {
  let mut hands: Vec<Hand> = input
    .split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| Hand::new(line, false))
    .collect();

  hands.sort_by(compare_hands);

  let sum = hands
    .iter()
    .enumerate()
    .fold(0, |acc, (i, hand)| acc + (hand.bid * (i as i32 + 1)));

  sum
}

fn pt2(input: &str) -> i32 {
  let mut hands: Vec<Hand> = input
    .split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| Hand::new(line, true))
    .collect();

  hands.sort_by(compare_hands);

  for (i, hand) in hands.iter().enumerate() {
    println!("{}: {:?} => {:?}", i, hand.cards, hand._type);
  }

  let sum = hands
    .iter()
    .enumerate()
    .fold(0, |acc, (i, hand)| acc + (hand.bid * (i as i32 + 1)));

  sum
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
  if a._type > b._type {
    return std::cmp::Ordering::Greater;
  }

  if a._type < b._type {
    return std::cmp::Ordering::Less;
  }

  // Type are the same, compare the cards
  for (i, card) in a.cards.iter().enumerate() {
    let other_card = &b.cards[i];

    if card == other_card {
      continue;
    }

    if card > other_card {
      return std::cmp::Ordering::Greater;
    }
    return std::cmp::Ordering::Less;
  }

  std::cmp::Ordering::Equal
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
    assert_eq!(pt2(&input), 5905);
  }
}
