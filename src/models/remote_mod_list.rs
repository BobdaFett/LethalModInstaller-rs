use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
pub use super::ModInfo;


#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteModList {
  pub mods: HashMap<String, ModInfo>
}
