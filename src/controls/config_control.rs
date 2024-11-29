use crate::utils::flush;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use toml;
use std::process;
use colored::*;
use glob::glob;
use crate::models::Configuration;

pub fn get_config() -> Configuration {
  // Find configuration file
  let mut path = dirs::config_local_dir().unwrap();
  path.push("AlphaFetus");
  path.push("LethalModInstaller");
  path.push("config.toml");

  print!("Getting config file at path {}... ", path.to_str().unwrap());
  flush!();
  let mut config_file = File::open(&path).unwrap_or_else(|_| {
    // Create directories
    fs::create_dir_all(&path.parent().unwrap().to_str().unwrap()).unwrap();
    File::create(&path).expect("Failed to create configuration file");

    // Write a default configuration to the file.
    // Default is simply an empty Configuration
    let mut default_config = Configuration::new();
    default_config.config_path = path.to_str().unwrap().to_string();
    default_config.lethal_path = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Lethal Company".to_string();
    let toml_string = toml::to_string_pretty(&default_config).unwrap();
    fs::write(&path, toml_string.as_str()).unwrap();
    File::open(&path).expect("Failed to open configuration file")
  });
  println!("{}", "Done.".green());

  // Load configuration file
  print!("Loading configuration... ");
  flush!();
  let mut config_contents = String::new();
  config_file.read_to_string(&mut config_contents).expect("Failed to read configuration file");

  // Return configuration
  match toml::from_str(config_contents.as_str()) {
    Ok(values) => {
      println!("{}", "Done.".green());
      values
    },
    Err(_) => {
      eprintln!("{}", "Failed to parse data from configuration file. Exiting...".red());
      process::exit(1);
    }
  }
}

pub fn verify_paths(config: &mut Configuration) {
  // Check lethal mods path
  // If it doesn't exist, ask the user to input it
  print!("Looking for Lethal Company directory... ");
  flush!();

  let mut valid_path = false;
  while !valid_path {
    let lethal_exe_path = format!("{}\\Lethal Company.exe", config.lethal_path);
    for entry in glob(&lethal_exe_path).expect("Failed to read glob") {
      if let Ok(_) = entry {
        println!("{}", "Found.".green());
        valid_path = true;
      }
    }

    if valid_path {
      break;
    }

    println!("{}", "Not found.".red());
    print!("Please enter the path to your Lethal Company directory: ");
    flush!();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    config.lethal_path = input.trim().to_string();
  }

  // Check bepinex path relative to lethal mods path
  // TODO Implement this - this will become an optional feature.

  // Write configuration to file
  save_config(&config).expect("Couldn't save configuration file");
}

pub fn save_config(config: &Configuration) -> io::Result<()> {
  // Open configuration file - the path is saved in the config object.
  let config_file = File::open(&config.config_path)?;

  // Serialize the configuration object
  let toml_string = toml::to_string_pretty(&config).unwrap_or_else(|_| {
    eprintln!("{}", "Failed to serialize configuration object.".red());
    process::exit(1);
  });

  // Write the serialized object to the file
  let mut writer = BufWriter::new(config_file);
  writer.write_all(&toml_string.as_bytes()).unwrap_or_else(|_| {
    eprintln!("{}", "Failed to write to configuration file.".red());
    process::exit(1);
  });

  Ok(())
}
