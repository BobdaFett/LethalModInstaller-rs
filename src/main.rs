use std::io::{ self, Write };
use std::process;

pub mod functions;
use functions::{
  install::install_mods,
  repair,
  structs::*
};

fn main() {
  match get_user_input() {
    0 => {
      println!("Exiting...");
      process::exit(0);
    },
    1 => {
      println!("Run install");
      install_mods().unwrap();
    },
    2 => println!("Run repair"),
    3 => println!("Run uninstall"),
    9 => println!("Run reset"),
    _ => println!("Invalid input")
  };
}

fn get_user_input() -> i8 {
  let mut user_input: i8;
  loop {
    println!("1) Install/Update mods (default)");
    println!("2) Repair mods");
    println!("3) Uninstall mods");
    println!("9) Reset configuration");
    println!("0) Exit");
    print!("\n\nHello! Please enter the number of the function you would like to perform (1-3): ");
    io::stdout().flush().expect("Failed to flush");
    let mut user_input_string = String::new();
    io::stdin().read_line(&mut user_input_string).expect("Failed to read line");
    // Attempt to parse user input into an integer
    user_input = match user_input_string.trim().parse::<i8>() {
      Ok(num) => num,
      Err(_) => 100,
    };

    // Verify user input is within the correct range
    match user_input {
      0..=3 => break,
      4..=8 => {
        println!("Please enter a number the corresponds to the options provided.");
      }
      9 => break,
      _ => {
        println!("Please enter a number that corresponds to the options provided.")
      }
    };
  }

  user_input
}
