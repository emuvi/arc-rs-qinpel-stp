extern crate serde;
extern crate serde_json;

use super::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStp {
  pub files: Vec<FileStp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStp {
  pub name: String,
  pub verifier: String,
}

impl IndexStp {
  pub fn new(from_json: String) -> Result<IndexStp> {
    match serde_json::from_str::<IndexStp>(&from_json) {
      Ok(result) => return Ok(result),
      Err(e) => return Err(Box::new(e))
    }
  }
}
