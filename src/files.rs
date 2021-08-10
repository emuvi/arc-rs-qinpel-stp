extern crate reqwest;
extern crate sha2;

use std::io;
use std::io::Cursor;
use std::fs::File;
use sha2::{Sha256, Digest};

use super::Result;

pub fn download_text(url: &str) -> Result<String> {
  let response = reqwest::blocking::get(url)?;
  let body = response.text()?;
  Ok(body)
}

pub fn download_file(url: &str, file: &str) -> Result<()> {
  let response = reqwest::blocking::get(url)?;
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