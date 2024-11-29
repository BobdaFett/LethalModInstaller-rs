use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct ModInfo {
  pub version: String,
  pub url: String
}

impl PartialEq for ModInfo {
  fn eq(&self, other: &ModInfo) -> bool {
    self.version == other.version && self.url == other.url
  }
}
