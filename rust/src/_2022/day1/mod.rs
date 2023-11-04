use crate::utils;

pub fn day_1() {
  println!("Hello, 2022 day 1!");

  // Read input.txt file
  let input = utils::get_input_file("./src/_2022/_1/input.txt");
  // Split the input and parse it
  let mut elf_calories = calculate_elf_calories(&input);
  // Sort the array by calories
  elf_calories.sort_by(compare_calories);
  // Get the total calories carried by the elf carrying the calories.
  let total_calories_carried_by_the_elf_carrying_the_most_calories = elf_calories[0].1;

  println!(
    "The total calories carried by the elf carrying the most calories is: {}",
    total_calories_carried_by_the_elf_carrying_the_most_calories
  );

  let calories_carried_by_the_top_3_elves =
    elf_calories[0].1 + elf_calories[1].1 + elf_calories[2].1;
  println!(
    "The total calories carried by the top 3 elves is: {}",
    calories_carried_by_the_top_3_elves
  );
}

fn calculate_elf_calories(input: &str) -> Vec<(usize, i32)> {
  let mut elf_calories: Vec<(usize, i32)> = [].to_vec();

  for (index, elf) in input.split("\n\n").enumerate() {
    let mut calories: i32 = 0;
    for line in elf.split('\n') {
      calories += parse_line(line);
    }
    elf_calories.push((index, calories));
  }

  elf_calories
}

fn parse_line(line: &str) -> i32 {
  if line.trim().is_empty() {
    return 0;
  }

  let number = line.parse::<i32>();

  match number {
    Ok(number) => number,
    Err(error) => {
      println!(
        "Whoops, could not parse the line {:?}? error: {:?}",
        line, error
      );
      std::process::exit(1);
    }
  }
}

fn compare_calories(a: &(usize, i32), b: &(usize, i32)) -> std::cmp::Ordering {
  b.1.cmp(&a.1)
}
