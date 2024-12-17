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

  loop {
    // Get user input and run correct function
    let input = get_user_input();
    println!("Current input value: {:?}", input);
    match input {
      FunctionType::Install => {
        println!("Attempting to run installation.");
        install_mods(&mut config, false, false).unwrap();
      },
      FunctionType::Uninstall => {
        println!("Attempting to run uninstallation.");
        uninstall_mods(&mut config, true).unwrap();
      },
      FunctionType::Repair => {
        println!("Attempting to run repair.");
        repair_mods(&mut config).unwrap();
      }
      FunctionType::Exit => {
        println!("Exiting...");
        break;
      }
      _ => {
        println!("Not implemented yet, exiting...");
        process::exit(1);
      }
    };
  };
}
