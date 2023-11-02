use crate::utils::get_input_file;
mod round; // This declares the round module
use round::Round; // This imports the Round struct from the round module

pub fn day_2() {
  let input = get_input_file("./src/_2022/_2/input.txt");
  println!("input: {:?}", input);

  let mut rounds: Vec<Round> = Vec::new();

  for round in input.split('\n') {
    if round.is_empty() {
      println!("Empty line, skipping");
    } else {
      let round = Round::new(round);
      rounds.push(round);
    }
  }

  let mut score = 0;
  for mut round in rounds {
    round.calculate_score();
    println!("round: {:?}", round);
    score += round.get_score();
  }

  println!("Final score: {}", score);
}
