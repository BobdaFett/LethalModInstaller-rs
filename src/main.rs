// Define modules
mod controls;
mod forms;
mod utils;
mod models;

use std::process;

use forms::*;
use controls::*;

fn main() {
  // Load configuration
  let mut config = get_config();
  verify_paths(&mut config);

  // Get user input and run correct function
  let input = get_user_input();
  println!("Current input value: {:?}", input);
  match input {
    FunctionType::Install => {
      println!("Attempting to run installation.");
      install_mods(&mut config).unwrap();
    },
    _ => {
      println!("Not implemented yet, exiting...");
      process::exit(1);
    }
  };

  println!("Done! Press enter to exit.");
  std::io::stdin().read_line(&mut String::new()).unwrap();
}
