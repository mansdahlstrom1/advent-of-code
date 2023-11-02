mod play;
use self::play::Play;

#[derive(Debug)]
pub struct Round {
  opponents_play: Play,
  my_play: Play,
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

  pub fn using_strategy_1(line: &str) -> Self {
    let chars: Vec<&str> = line.split_whitespace().collect();
    let opponents_play = Play::from_str(chars[0]);
    let my_play = Play::from_str(chars[1]);

    Self {
      opponents_play,
      my_play,
      score: None,
    }
  }

  pub fn using_strategy_2(line: &str) -> Self {
    let chars: Vec<&str> = line.split_whitespace().collect();
    let opponents_play = Play::from_str(chars[0]);
    let my_play = Play::based_on_opponents_play(chars[1], &opponents_play);

    Self {
      opponents_play,
      my_play,
      score: None,
    }
  }

  pub fn calculate_score(&mut self) -> i32 {
    let game_score = match self.my_play {
      Play::Rock => match self.opponents_play {
        Play::Paper => 0,
        Play::Rock => 3,
        Play::Scissors => 6,
      },
      Play::Paper => match self.opponents_play {
        Play::Scissors => 0,
        Play::Paper => 3,
        Play::Rock => 6,
      },
      Play::Scissors => match self.opponents_play {
        Play::Rock => 0,
        Play::Scissors => 3,
        Play::Paper => 6,
      },
    };

    let play_score = match self.my_play {
      Play::Rock => 1,
      Play::Paper => 2,
      Play::Scissors => 3,
    };

    self.set_score(game_score + play_score);
    self.get_score()
  }
}
