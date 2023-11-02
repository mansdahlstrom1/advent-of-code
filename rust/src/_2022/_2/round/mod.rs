mod me;
mod opponent;
use self::me::Me;
use self::opponent::Opponent;

#[derive(Debug)]
pub struct Round {
  opponent: Opponent,
  me: Me,
  score: Option<i32>,
}

impl Round {
  fn set_score(&mut self, score: i32) {
    self.score = Some(score);
  }

  pub fn get_score(&self) -> i32 {
    match self.score {
      Some(score) => score,
      None => panic!("Score not set for round: {:?}", self),
    }
  }

  pub fn new(round: &str) -> Self {
    let chars: Vec<&str> = round.split_whitespace().collect();
    let opponent = chars[0];
    let me = chars[1];

    Self {
      opponent: Opponent::from_str(opponent),
      me: Me::from_str(me),
      score: None,
    }
  }

  pub fn calculate_score(&mut self) {
    let game_score = match self.me {
      Me::Rock => match self.opponent {
        Opponent::Paper => 0,
        Opponent::Rock => 3,
        Opponent::Scissors => 6,
      },
      Me::Paper => match self.opponent {
        Opponent::Scissors => 0,
        Opponent::Paper => 3,
        Opponent::Rock => 6,
      },
      Me::Scissors => match self.opponent {
        Opponent::Rock => 0,
        Opponent::Scissors => 3,
        Opponent::Paper => 6,
      },
    };

    let play_score = match self.me {
      Me::Rock => 1,
      Me::Paper => 2,
      Me::Scissors => 3,
    };

    self.set_score(game_score + play_score);
  }
}
