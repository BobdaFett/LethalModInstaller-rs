use std::io::{ self, Write };

#[derive(Debug)]
pub enum FunctionType {
  Install,
  Repair,
  Uninstall,
  ResetConfig,
  Exit
}


pub fn get_user_input() -> FunctionType {
  let mut selected_function: Option<FunctionType> = None;
  while let None = selected_function {
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
    let user_input = match user_input_string.trim().parse::<i8>() {
      Ok(num) => num,
      Err(_) => 100,
    };

    // Verify user input is within the correct range and return function type
    selected_function = match user_input {
      0 => Some(FunctionType::Exit),
      1 => Some(FunctionType::Install),
      2 => Some(FunctionType::Repair),
      3 => Some(FunctionType::Uninstall),
      9 => Some(FunctionType::ResetConfig),
      _ => {
        println!("Please enter an option that corresponds to the options provided.");
        None
      }
    };
  }

  selected_function.unwrap()
}
