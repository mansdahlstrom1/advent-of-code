use super::round::Round;

#[derive(Debug)]
pub struct Game {
  pub id: i32,
  pub rounds: Vec<Round>,
}

impl Game {
  pub fn is_valid(&self) -> bool {
    self.rounds.iter().all(|round| round.is_valid())
  }

  pub fn get_game_power(&self) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for round in &self.rounds {
      if round.red > min_red {
        min_red = round.red;
      }
      if round.green > min_green {
        min_green = round.green;
      }
      if round.blue > min_blue {
        min_blue = round.blue;
      }
    }

    min_red * min_green * min_blue
  }
}
