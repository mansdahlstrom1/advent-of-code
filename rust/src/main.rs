use aoc::{
  utils::create_day,
  _2022::{day1::day_1, day2::day_2},
};
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

    create_day(args.year, args.day).await;
    return;
  }

  match args.year {
    2022 => match args.day {
      1 => day_1(),
      2 => day_2(),
      _ => println!("Invalid day: {}", args.day),
    },
    _ => println!("Invalid year: {}", args.year),
  }
}
