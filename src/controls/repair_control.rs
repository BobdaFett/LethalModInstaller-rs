// This function should uninstall, and then reinstall the mods that are in the configuration file,
// regardless of whether they are up to date or not. This is useful for when the user wants to
// reinitialize the mods, or if the mods are corrupted.
// Note that this will not wipe the configuration file, so the user will not have to reconfigure
// the mods entirely. They would need to run the ResetConfig command to do that.

use std::io;
use crate::models::*;


pub fn repair_mods() -> io::Result<()> {
  // Run the uninstall function, without wiping the configuration
  // Run the install function
  Ok(())
}
