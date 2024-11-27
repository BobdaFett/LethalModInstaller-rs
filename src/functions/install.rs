use std::fs::{ self, File };
use std::path::PathBuf;
use std::io::{ self, Read, Write };
use std::process;
use toml;
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
  lethal_path: String,
  mod_list: HashMap<String, ModInfo>
}

impl Configuration {
  fn new() -> Configuration {
    Configuration {
      lethal_path: String::from(""),
      mod_list: HashMap::new()
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModInfo {
  version: String,
  url: String
}

pub fn install_mods() -> io::Result<()> {
  // Get application configuration
  let mut config = get_configuration().unwrap();
  // Find Lethal Company folder
  let lethal_path = find_lethal_folder(&mut config).unwrap();
  println!("Lethal Company path: {}", lethal_path.to_str().unwrap());
  // Update configuration from Github
  get_remote_config().unwrap();
  // Download mods
  // Install mods
  Ok(())
}

fn find_lethal_folder(config: &mut Configuration) -> io::Result<PathBuf> {
  print!("Searching for Lethal Company data folder ");
  // Check for Lethal Company folder in C: drive
  let folder_path = if config.lethal_path != "" {
    // If the user has a stored path, use that one
    print!("using stored path... ");
    PathBuf::from(&config.lethal_path.to_string())
  } else {
    // This is simply the default path to check
    // The user can still input a different path if this one is not correct
    print!("using default path... ");
    PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Lethal Company")
  };
  flush_stdio();

  // Verify that the folder exists, and if not then prompt the user to input the path
  if folder_path.exists() {
    println!("{}", "Found.".green());
    Ok(folder_path)
  } else {
    println!("{}", "Not found.".yellow());
    let mut user_path = String::new();
    let mut path_return: PathBuf;
    loop {
      print!("Please input the path to the Lethal Company folder: ");
      io::stdout().flush().expect("Failed to flush");
      io::stdin().read_line(&mut user_path).expect("Failed to read line");
      path_return = PathBuf::from(&user_path);
      if path_return.exists() {
        break;
      } else {
        println!("{}", "The path you entered does not exist. Please enter a valid path.".red());
      }
    }

    // When the program closes, the path will be stored in the configuration file
    config.lethal_path = user_path.trim().to_string();
    Ok(path_return)
  }
}

fn get_configuration() -> io::Result<Configuration> {
  // Get local configuration file path
  let mut path = dirs::config_local_dir().unwrap();
  path.push("AlphaFetus");
  path.push("LethalModInstaller");
  path.push("config.toml");

  // Check if configuration file exists
  // If not, create it
  print!("Getting config file at path {}... ", path.to_str().unwrap());
  flush_stdio();
  let mut config_file = File::open(&path).unwrap_or_else(|_| {
    // Create directory path as well
    fs::create_dir_all(&path.parent().unwrap().to_str().unwrap()).unwrap();
    File::create(&path).expect("Failed to create configuration file");

    // Write default configuration to file
    // Default configuration is an empty Configuration struct
    let base_config = Configuration::new();
    let toml_string = toml::to_string_pretty(&base_config).unwrap();
    fs::write(&path, toml_string.as_str()).unwrap();
    File::open(&path).expect("Failed to open configuration file")
  });
  println!("{}", "Done.".green());

  // Read configuration file into a string and generate
  // Configuration struct from it
  print!("Loading configuration... ");
  flush_stdio();
  let mut config_contents = String::new();
  config_file.read_to_string(&mut config_contents).expect("Could not read configuration file");
  let configuration: Configuration = match toml::from_str(config_contents.as_str()) {
    Ok(values) => values,
    Err(_) => {
      eprintln!("Failed to parse data from config file.");
      process::exit(1);
    }
  };

  // Print configuration values
  // println!("{:?}", configuration);
  println!("{}", "Done.".green());
  Ok(configuration)
}

fn get_remote_config() -> io::Result<()> {
  // Get remote configuration from Github with a GET request
  let url = "https://raw.githubusercontent.com/BobdaFett/LethalModInstaller/refs/heads/main/modlist.toml";
  // let url = "https://raw.githubusercontent.com/BobdaFett/advent-rust-23/refs/heads/main/src/util.rs";

  // Send GET reqwest
  let modlist_string = reqwest::blocking::get(url).unwrap().text().unwrap();
  println!("{}", modlist_string);

  Ok(())
}

fn download_mods() {
  // Download mods from Github
}

fn flush_stdio() {
  io::stdout().flush().expect("Failed to flush");
}
