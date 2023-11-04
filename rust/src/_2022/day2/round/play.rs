#[derive(Debug)]
pub enum Play {
  Rock,
  Paper,
  Scissors,
}

impl Play {
  pub fn from_str(input: &str) -> Play {
    match input {
      "A" => Play::Rock,
      "B" => Play::Paper,
      "C" => Play::Scissors,
      "X" => Play::Rock,
      "Y" => Play::Paper,
      "Z" => Play::Scissors,
      &_ => panic!("Unknown Me: {}", input),
    }
  }

  pub fn based_on_opponents_play(input: &str, opponents_play: &Play) -> Play {
    match input {
      // We need to lose
      "X" => match opponents_play {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
      },
      // We need to end the round in a draw
      "Y" => match opponents_play {
        Play::Rock => Play::Rock,
        Play::Paper => Play::Paper,
        Play::Scissors => Play::Scissors,
      },
      // We need to win
      "Z" => match opponents_play {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
      },
      &_ => panic!("Unknown Me: {}", input),
    }
  }
}
