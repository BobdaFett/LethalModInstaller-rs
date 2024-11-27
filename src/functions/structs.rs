use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
  pub lethal_path: String,
  pub mods: HashMap<String, ModInfo>
}

impl Configuration {
  pub fn new() -> Configuration {
    Configuration {
      lethal_path: String::from(""),
      mods: HashMap::new()
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModInfo {
  pub version: String,
  pub url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteModConfig {
  pub mods: HashMap<String, ModInfo>
}