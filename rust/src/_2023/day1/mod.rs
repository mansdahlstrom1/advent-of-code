use crate::utils;

pub fn main() {
  let input = utils::get_input_file("./src/_2023/day1/input.txt");
  let lines = input.split('\n');
  let mut result: i32 = 0;

  for line in lines {
    if line.is_empty() {
      continue;
    }
    let numbers: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    let s: String = numbers.iter().collect();
    let digits = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
    let value = digits.parse::<i32>().unwrap();
    result += value;
    println!("{} => {} => {}", line, s, digits);
  }

  println!("Result: {}", result)
}
