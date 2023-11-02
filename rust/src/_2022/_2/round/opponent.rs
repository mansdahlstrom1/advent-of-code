#[derive(Debug)]
pub enum Opponent {
  Rock,
  Paper,
  Scissors,
}

impl Opponent {
  pub fn from_str(str: &str) -> Opponent {
    match str {
      "A" => Opponent::Rock,
      "B" => Opponent::Paper,
      "C" => Opponent::Scissors,
      &_ => panic!("Unknown opponent: {}", str),
    }
  }
}
