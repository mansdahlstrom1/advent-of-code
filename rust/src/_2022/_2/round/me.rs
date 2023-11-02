#[derive(Debug)]
pub enum Me {
  Rock,
  Paper,
  Scissors,
}

impl Me {
  pub fn from_str(str: &str) -> Me {
    match str {
      "X" => Me::Rock,     // X means you need to lose,
      "Y" => Me::Paper,    // Y means you need to end the round in a draw
      "Z" => Me::Scissors, // Z means you need to win.
      &_ => panic!("Unknown Me: {}", str),
    }
  }
}
