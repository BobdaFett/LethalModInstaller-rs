// Define modules
mod controls;
mod forms;
mod utils;

use std::process;
use std::io::{self, Write};

fn main() {
  // Load configuration
  let mut config = controls::get_config();

  // Get user input and run correct function
  let input = forms::get_user_input();
  println!("Current input value: {:?}", input);
  match input {
    forms::FunctionType::Install => {
      println!("Attempting to run installation.");
      controls::install_mods(&mut config)
    },
    _ => {
      println!("Not implemented yet, exiting...");
      process::exit(1);
    }
  }.unwrap();
}
