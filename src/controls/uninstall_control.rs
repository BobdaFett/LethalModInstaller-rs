use std::io::{self, Write};
use crate::models::*;
use colored::*;
use crate::utils::flush;

use super::save_config;


// This function should completely uninstall all mods.
// The boolean indicates whether the configuration file should be altered or not.
pub fn uninstall_mods(config: &mut Configuration, alter_config: bool) -> io::Result<()> {
  // Run the uninstall function
  uninstall_mods_from_path(&config)?;
  // Wipe the configuration file if alter_config is true
  if alter_config {
    uninstall_mods_from_config(config)?;
  }
  println!("Uninstallation complete.");
  Ok(())
}

// This function should completely delete all configured mods from the lethal path
fn uninstall_mods_from_path(config: &Configuration) -> io::Result<()> {
  // Get the list of mods from the configuration file
  let mods = config.mods.keys();
  mods.for_each(|mod_name| {
    // Delete the mod from the lethal path
    let mod_path = format!("{}\\Bepinex\\plugins\\{}", config.lethal_path, mod_name);
    print!("Deleting mod {} from path {}... ", mod_name, mod_path);
    flush!();
    if let Err(e) = std::fs::remove_dir_all(mod_path) {
      println!("{}", "Failed.".red());
      eprintln!("{}", e.to_string().red());
      println!("{}", "Maybe it was already deleted?".yellow());
    } else {
      println!("{}", "Done.".green());
    }
  });
  Ok(())
}

// This function should alter the configuration file to remove the mods
// Note that this doesn't actually delete the mods from the lethal path itself
// and must be run in conjunction with uninstall_mods_from_path
fn uninstall_mods_from_config(config: &mut Configuration) -> io::Result<()> {
  // Remove all mods from the configuration file
  config.mods = HashMap::new();
  save_config(&config)?;
  Ok(())
}
