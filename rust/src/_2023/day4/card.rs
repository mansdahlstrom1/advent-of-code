use crate::utils::exit_with_message;

#[derive(Clone, Debug)]
pub struct Card {
  pub id: i32,
  pub winning_numbers: Vec<i32>,
  pub my_numbers: Vec<i32>,
  pub is_copy: bool,
  pub score: Option<i32>,
  pub matches: Option<i32>,
}

impl Card {
  pub fn new(card: &str, winning_numbers: Vec<i32>, my_numbers: Vec<i32>, is_copy: bool) -> Card {
    let id = card.replace("Card ", "").trim().parse::<i32>();
    let id = match id {
      Ok(id) => id,
      Err(_err) => {
        exit_with_message("Failed to parse card id...");
      }
    };

    Card {
      id,
      winning_numbers,
      my_numbers,
      is_copy,
      score: None,
      matches: None,
    }
  }

  pub fn calculate_card_score(&mut self) -> &mut Self {
    let mut score = 0;
    let mut matches = 0;
    for number in &self.my_numbers {
      if self.winning_numbers.contains(number) {
        if score == 0 {
          score = 1;
        } else {
          score *= 2;
        }
        matches += 1;
      }
    }

    self.score = Some(score);
    self.matches = Some(matches);
    self
  }
}
