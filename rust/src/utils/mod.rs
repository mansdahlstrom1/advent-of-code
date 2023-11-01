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
