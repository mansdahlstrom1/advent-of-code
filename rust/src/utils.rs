use reqwest::Error;
use std::{env, fs};

use crate::constants;

pub fn get_input_file(path: &str) -> String {
  let input_file = std::fs::read_to_string(path);
  match input_file {
    Ok(file) => file,
    Err(error) => {
      println!("Whoops, could not find the input file? error: {:?}", error);
      std::process::exit(1);
    }
  }
}

pub fn exit_with_message(message: &str) -> ! {
  println!("{}", message);
  print!("Exiting...");
  std::process::exit(1);
}

pub async fn create_day(year: i32, day: u8) {
  println!("Creating new day {} for year {}", day, year);
  let path = format!("src/_{}/day{}", year, day);

  let response = download_input_file(year, day).await;
  let input = match response {
    Ok(input) => {
      println!("Retrieved input for day {} {}", day, year);
      input
    }
    Err(error) => {
      if error.status().unwrap() == reqwest::StatusCode::NOT_FOUND {
        exit_with_message(&format!(
          "Input for day {} {} not found, maybe it hasn't been released yet?",
          day, year
        ));
      }

      exit_with_message(&format!("Error retrieving input: {}", error));
    }
  };

  let create_dir: Result<(), std::io::Error> = fs::create_dir_all(&path);
  match create_dir {
    Ok(_) => println!("Created directory: {}", path),
    Err(error) => exit_with_message(&format!("Error creating directory: {}", error)),
  }

  let create_input = fs::write(format!("{}/input.txt", path), input);
  match create_input {
    Ok(_) => println!("Created input.txt"),
    Err(error) => exit_with_message(&format!("Error creating input.txt: {}", error)),
  }
}

async fn download_input_file(year: i32, day: u8) -> Result<String, Error> {
  let endpoint = format!("{}/{}/day/{}/input", constants::BASE_URL, year, day);
  let session = env::var("AOC_SESSION").unwrap();
  let client = reqwest::Client::new();
  let response = client
    .get(&endpoint)
    .header("Cookie", session)
    .send()
    .await?;

  match response.error_for_status() {
    Ok(_res) => _res.text().await,
    Err(err) => Err(err),
  }
}
