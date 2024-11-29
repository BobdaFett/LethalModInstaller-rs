use std::io::{ self, copy, Write };
use crate::controls::{ Configuration, RemoteModList };
use tempfile::Builder;
use std::fs::File;
use crate::utils::flush;
use colored::*;

pub fn install_mods(config: &mut Configuration) -> io::Result<()> {
  println!("Starting mod installation.");
  // Get remote mod configuration
  let url = "https://raw.githubusercontent.com/BobdaFett/LethalModInstaller-rs/refs/heads/main/modlist.toml";
  print!("Getting remote mod config... ");
  flush!();
  let modlist_string = reqwest::blocking::get(url).unwrap().text().unwrap();
  println!("{}", "Done.".green());

  // Parse into RemoteModList
  let remote_modlist: RemoteModList = toml::from_str(&modlist_string).unwrap();

  // Compare local and remote mod configurations
  remote_modlist.mods.iter().for_each(|(remote_name, remote_info)| {
    let local_mod = config.mods.get(remote_name);
    match local_mod {
      Some(local_mod) => {
        if local_mod.version != remote_info.version {
          // Install mod
          install_mod_from_url(&remote_info.url).unwrap();
        }
      },
      None => {
        // Install mod
        install_mod_from_url(&remote_info.url).unwrap();
      }
    }
  });

  Ok(())
}

// TODO Put in extra error handling
fn install_mod_from_url(url: &String) -> io::Result<()> {
  // Get file from url with GET request
  let response = reqwest::blocking::get(url).unwrap();

  // Get a temporary directory, create a file in it...
  let temp_dir = Builder::new().prefix("LethalModInstaller").tempdir()?;
  let mut file = File::create(temp_dir.path()).unwrap();

  // ...and copy the response body into it
  let mut content: &[u8] = &response.bytes().unwrap();
  copy(&mut content, &mut file)?;  // Hopefully this works, it's a little jank


  Ok(())
}
