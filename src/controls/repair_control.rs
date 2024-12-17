// This function should uninstall, and then reinstall the mods that are in the configuration file,
// regardless of whether they are up to date or not. This is useful for when the user wants to
// reinitialize the mods, or if the mods are corrupted.
// Note that this will not wipe the configuration file, so the user will not have to reconfigure
// the mods entirely. They would need to run the ResetConfig command to do that.

use std::io;
use crate::models::*;

use super::{install_mods, uninstall_mods};


pub fn repair_mods(config: &mut Configuration) -> io::Result<()> {
  // Run the uninstall function, without wiping the configuration
  uninstall_mods(config, false)?;
  // Run the install function, and force the installation
  install_mods(config, true, true)?;
  Ok(())
}
