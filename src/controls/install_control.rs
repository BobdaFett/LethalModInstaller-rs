use std::{
  fs,
  io::{
    self, copy, Result, Write
  }
};
use crate::{models::*, save_config};
use tempfile::Builder;
use std::fs::File;
use crate::utils::flush;
use colored::*;
use zip_extract;
use glob::glob;
use dircpy::copy_dir;


pub fn install_mods(config: &mut Configuration, force_install: bool, local_config_only: bool) -> io::Result<()> {
  println!("Starting mod installation.");
  // Get remote mod configuration
  let url = "https://raw.githubusercontent.com/BobdaFett/LethalModInstaller-rs/refs/heads/main/modlist.toml";
  let modlist_string = if local_config_only {
    // If we're forcing the installation, we don't need to check the remote modlist
    // We can just use the one we have
    // This is useful for when the user wants to reinstall all mods
    println!("Using local configuration...");
    let modlist_string = fs::read_to_string("modlist.toml").unwrap();
    modlist_string
  } else {
    print!("Getting remote mod config... ");
    flush!();
    let modlist_string = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("{}", "Done.".green());
    modlist_string
  };

  // Parse into RemoteModList
  let remote_modlist: RemoteModList = toml::from_str(&modlist_string).unwrap();

  // Compare local and remote mod configurations
  remote_modlist.mods.iter().for_each(|(remote_name, remote_info)| {
    let local_mod = config.mods.get(remote_name);
    let install_mod = match local_mod {
      Some(local_mod) => local_mod.version != remote_info.version,
      None => true
    };

    if install_mod == true || force_install == true {
      match install_mod_from_url(&remote_info, &remote_name, &config.lethal_path) {
        Ok(_) => {
          // Update the configuration
          config.mods.insert(remote_name.clone(), remote_info.clone());
        },
        Err(_) => {
          let fmt_string = format!("Error installing mod {}", remote_name);
          eprintln!("{}", fmt_string.red());
        }
      }
    } else {
      println!("{}", format!("Mod {} is up to date.", remote_name).green());
    }
  });

  save_config(&config).unwrap();

  Ok(())
}

// TODO Put in extra error handling
fn install_mod_from_url(info: &ModInfo, name: &String, lethal_dir: &String) -> io::Result<()> {
  let url = &info.url;
  // Get a temporary directory
  let temp_dir = Builder::new().prefix("LethalModInstaller").tempdir()?;
  // Filename for the downloaded zip file
  let filename = temp_dir.path().join(format!("{}.zip", name));
  // Mod directory specific to this mod
  let lethal_mod_dir = format!("{}\\{}\\{}", lethal_dir, "BepInEx\\plugins", name);

  print!("Downloading mod {}... ", name);
  flush!();

  // Get file from url with GET request
  let response = match reqwest::blocking::get(url) {
    Ok(response) => response,
    Err(e) => {
      eprintln!("{}", "Failed.".red());
      eprintln!("{}", e.to_string().red());
      return Result::Err(io::Error::new(io::ErrorKind::Other, "Failed to get response"));
    }
  };

  // Create a file in the temp directory...
  let mut file = File::create(&filename)?;

  // ...and copy the response body into it
  let mut content: &[u8] = &response.bytes().unwrap();
  copy(&mut content, &mut file)?;  // Hopefully this works, it's a little jank
  println!("{}", "Done.".green());

  // Unzip the file into the lethal directory
  // Attempt to open the file again
  file = File::open(&filename)?;
  println!("Installing {}...", name);
  zip_extract::extract(&file, temp_dir.path(), false).unwrap();

  // Check if there is a top-level directory in the zip
  // If there is, check the name and decide what to do from there
  // If there isn't, just move the files into the lethal directory with a new folder.
  // We'll do this by checking for dirs first
  let check_files = format!("{}\\**\\{}.dll", temp_dir.path().to_str().unwrap(), name);
  // println!("Finding dll's with pattern {}", check_files);
  for entry in glob(&check_files).expect("Failed to read glob") {
    match entry {
      Ok(path) => {
        // println!("{}", path.display());
        // Get a list of files at the parent of this path
        let filepaths = fs::read_dir(path.parent().unwrap())?;

        // Attempt to delete the previously created directory
        // If it doesn't exist, that's fine
        println!("Deleting previous directory...");
        let _ = fs::remove_dir_all(&lethal_mod_dir);
        // Create the directory
        println!("Creating directory {}", lethal_mod_dir);
        fs::create_dir_all(&lethal_mod_dir)?;

        filepaths.for_each(|path| {
          let path = path.unwrap().path();
          // println!("Found path {}", path.display());

          // Generate the new filepath
          let filename = path.file_name().unwrap().to_str().unwrap();
          let single_filepath = format!("{}\\{}", lethal_mod_dir, filename);

          println!("Copying file {:?} to {}", path.file_name().unwrap(), single_filepath);
          if path.is_dir() {
            // The fs::copy function doesn't (easily) work with directories
            if let Err(e) = copy_dir(path, &single_filepath) {
              eprintln!("{}", e.to_string().red());
            }
          }
          else {
            if let Err(e) = fs::copy(path, single_filepath) {
              eprintln!("{}", e.to_string().red());
            }
          }
        });
      },
      Err(e) => {
        eprintln!("{}", e);
      }
    }
  }

  let success_string = format!("Mod {} installed successfully!", name);

  println!("{}", success_string.green());

  Ok(())
}
