use crate::utils::get_input_file;

mod round;
use round::Round;

#[derive(Debug)]
struct Score {
  strategy_1: i32,
  strategy_2: i32,
}

pub fn day_2() {
  let input = get_input_file("./src/_2022/_2/input.txt");

  let mut score = Score {
    strategy_1: 0,
    strategy_2: 0,
  };

  for line in input.split('\n') {
    if line.is_empty() {
      println!("Empty line, skipping");
    } else {
      let mut strategy_1 = Round::using_strategy_1(line);
      score.strategy_1 += strategy_1.calculate_score();

      let mut strategy_2 = Round::using_strategy_2(line);
      score.strategy_2 += strategy_2.calculate_score();
    }
  }

  println!("Final score using strategy 1: {}", score.strategy_1);
  println!("Final score using strategy 2: {}", score.strategy_2);
}
