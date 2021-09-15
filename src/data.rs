use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::path::PathBuf;
use super::Result;

#[derive(Serialize, Deserialize)]
pub struct IndexStp {
  pub files: Vec<FileStp>,
}

#[derive(Serialize, Deserialize)]
pub struct FileStp {
  pub path: String,
  pub verifier: String,
}

impl IndexStp {
  pub fn new(from_json: String) -> Result<IndexStp> {
    match serde_json::from_str::<IndexStp>(&from_json) {
      Ok(result) => return Ok(result),
      Err(e) => return Err(Box::new(e)),
    }
  }

  pub fn save(&self, destiny: PathBuf) -> Result<()> {
    Ok(serde_json::to_writer(File::create(destiny)?, self)?)
  }
}
