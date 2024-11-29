use std::{fs, io::{ self, copy, Write }};
use crate::controls::{ Configuration, RemoteModList, ModInfo };
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
  // Get a temporary directory and filename
  let temp_dir = Builder::new().prefix("LethalModInstaller").tempdir()?;
  let filename = temp_dir.path().join(format!("{}.zip", name));
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
  print!("Unzipping file to {}... ", lethal_mod_dir.to_string());
  flush!();
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
        let filepath_split = path.to_str().unwrap().split("\\").collect::<Vec<&str>>();
        let single_filepath = filepath_split.last().unwrap();
        // Get a list of files at the parent of this path
        let filepaths = fs::read_dir(path.parent().unwrap()).unwrap();
        filepaths.for_each(|path| {
          let path = path.unwrap().path();
          // Move the files to the lethal mod directory
          let mod_path = format!("{}\\{}", lethal_mod_dir, single_filepath);
          // println!("Moving file from {} to {}", path.display(), mod_path);

          // Ensure the mod path exists
          fs::create_dir_all(&lethal_mod_dir).unwrap();

          fs::rename(path, mod_path).unwrap();
        });
      },
      Err(e) => {
        eprintln!("{}", e);
      }
    }
  }

  println!("{}", "Done.".green());

  Ok(())
}
