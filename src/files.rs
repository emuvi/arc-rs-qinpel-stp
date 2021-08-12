extern crate reqwest;
extern crate sha2;

use std::io;
use std::fs;
use std::io::Cursor;
use std::fs::File;
use sha2::{Sha256, Digest};
use std::path::Path;

use super::Result;

pub fn download_text(url: &str) -> Result<String> {
  let response = reqwest::blocking::get(url)?;
  let body = response.text()?;
  Ok(body)
}

pub fn download_file(url: &str, file: &str) -> Result<()> {
  let response = reqwest::blocking::get(url)?;
  fs::remove_file(file).ok();
  let mut file = File::create(file)?;
  let mut content =  Cursor::new(response.bytes()?);
  io::copy(&mut content, &mut file)?;
  Ok(())
}

pub fn get_verifier(file: &str) -> Result<String> {
  let mut file = File::open(file)?;
  let mut sha256 = Sha256::new();
  io::copy(&mut file, &mut sha256)?;
  let hash = format!("{:x}", sha256.finalize());
  Ok(hash)
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
      let entry = entry?;
      let file_type = entry.file_type()?;
      if file_type.is_dir() {
          copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
      } else {
          fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
      }
  }
  Ok(())
}