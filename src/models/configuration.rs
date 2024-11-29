use serde::{Deserialize, Serialize};
pub use super::ModInfo;
pub use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
  pub config_path: String,
  pub lethal_path: String,
  pub mods: HashMap<String, ModInfo>
}

impl Configuration {
  pub fn new() -> Configuration {
    Configuration {
      config_path: String::new(),
      lethal_path: String::new(),
      mods: HashMap::new()
    }
  }
}
