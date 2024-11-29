// Define modules
pub mod mod_info;
pub mod configuration;
pub mod remote_mod_list;


// Export modules
pub use self::{
  mod_info::ModInfo,
  configuration::Configuration,
  remote_mod_list::RemoteModList
};

use crate::utils::flush;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use toml;
use std::process;
use colored::*;

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

  // Load configuration file
  // TODO Add extra checking
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

pub fn save_config(config: Configuration) -> io::Result<()> {
  // Open configuration file - the path is saved in the config object.
  let mut config_file = File::open(&config.config_path)?;

  // Serialize the configuration object
  let toml_string = toml::to_string_pretty(&config).unwrap_or_else(|_| {
    eprintln!("{}", "Failed to serialize configuration object.".red());
    process::exit(1);
  });

  // Write the serialized object to the file
  config_file.write(&toml_string.as_bytes()).unwrap_or_else(|_| {
    eprintln!("{}", "Failed to write to configuration file.".red());
    process::exit(1);
  });

  Ok(())
}
