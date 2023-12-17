use aoc::{utils, _2022, _2023};
use clap::Parser;
use std::env;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
  #[arg(short, long)]
  year: i32,

  #[arg(short, long)]
  day: u8,

  #[arg(short, long)]
  create: bool,
}

#[tokio::main]
async fn main() {
  let args = Cli::parse();

  if args.create {
    // Check if AOC_SESSION environment variable is set
    if env::var("AOC_SESSION").is_err() {
      println!("AOC_SESSION environment variable not set");
      println!("Exiting...");
      std::process::exit(1);
    }

    utils::create_day(args.year, args.day).await;
    return;
  }

  match args.year {
    2022 => match args.day {
      1 => _2022::day1::main(),
      2 => _2022::day2::main(),
      _ => println!("Invalid day: {}", args.day),
    },
    2023 => match args.day {
      1 => _2023::day1::main(),
      2 => _2023::day2::main(),
      3 => _2023::day3::main(),
      4 => _2023::day4::main(),
      5 => _2023::day5::main(),
      6 => _2023::day6::main(),
      7 => _2023::day7::main(),
      8 => _2023::day8::main(),
      9 => _2023::day9::main(),
      10 => _2023::day10::main(),
      _ => println!("Invalid day: {}", args.day),
    },
    _ => println!("Invalid year: {}", args.year),
  }
}
