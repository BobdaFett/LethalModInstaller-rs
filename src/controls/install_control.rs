use std::{
  fs,
  io::{
    self,
    copy,
    Write
  },
  path::Path
};
use crate::models::*;
use tempfile::Builder;
use std::fs::File;
use crate::utils::flush;
use colored::*;
use zip_extract;
use glob::glob;


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
          install_mod_from_url(&remote_info, &remote_name, &config.lethal_path).unwrap();
        }
      },
      None => {
        // Install mod
        install_mod_from_url(&remote_info, &remote_name, &config.lethal_path).unwrap();
      }
    }
  });

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
  let response = reqwest::blocking::get(url).unwrap();

  // Create a file in the temp directory...
  let mut file = File::create(&filename).unwrap();

  // ...and copy the response body into it
  let mut content: &[u8] = &response.bytes().unwrap();
  copy(&mut content, &mut file)?;  // Hopefully this works, it's a little jank
  println!("{}", "Done.".green());

  // Unzip the file into the lethal directory
  // Attempt to open the file again
  file = File::open(&filename).unwrap();
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
        let filepaths = fs::read_dir(path.parent().unwrap()).unwrap();

        // Attempt to delete the previously created directory
        // If it doesn't exist, that's fine
        println!("Deleting previous directory...");
        let _ = fs::remove_dir_all(&lethal_mod_dir);
        // Create the directory
        println!("Creating directory {}", lethal_mod_dir);
        fs::create_dir_all(&lethal_mod_dir).unwrap();

        filepaths.for_each(|path| {
          let path = path.unwrap().path();
          // println!("Found path {}", path.display());

          // Generate the new filepath
          let filename = path.file_name().unwrap().to_str().unwrap();
          let single_filepath = format!("{}\\{}", lethal_mod_dir, filename);

          println!("Moving file {:?} to {}", path.file_name().unwrap(), single_filepath);
          let rename_result = fs::rename(path, single_filepath);
          if let Err(e) = rename_result {
            eprintln!("{}", e.to_string().red());
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
