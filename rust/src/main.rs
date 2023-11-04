use aoc::_2022::{day1::day_1, day2::day_2};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
  #[arg(short, long)]
  year: i32,

  #[arg(short, long)]
  day: u8,
}

fn main() {
  let args = Cli::parse();

  match args.year {
    2022 => match args.day {
      1 => day_1(),
      2 => day_2(),
      _ => println!("Invalid day: {}", args.day),
    },
    _ => println!("Invalid year: {}", args.year),
  }
}
