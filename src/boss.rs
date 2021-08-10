extern crate url;

use super::files;
use std::path::Path;
use std::path::PathBuf;
use url::Url;
use std::fs;

use super::Result;
use super::data::IndexStp;

const ATTEMPTS: usize = 3;

pub struct Boss {
  url: Url,
  destiny: PathBuf,
  temp: PathBuf,
  placed: Vec<String>,
}

impl Boss {
  pub fn new(url: Url, destiny: PathBuf, temp: PathBuf) -> Boss {
    Boss { url, destiny, temp, placed: Vec::new() }
  }

  pub fn run(&self) {
    println!("URL Path     : {}", self.url);
    println!("Destiny Path : {}", self.destiny.display());
    println!("Temp Path    : {}", self.temp.display());
    self.start_temp();
    match self.get_index_stp() {
      Ok(index_stp) => self.setup(index_stp),
      Err(e) => println!("{}", e),
    }
    self.finish();
  }

  fn start_temp(&self) {
    if self.temp.exists() {
      panic!("There's already a temp path. This implies that another instance of this program is running or that was an crash in some previous execution. You must check-it.");
    }
    fs::create_dir_all(&self.temp).expect("Could not create the temp path.");
  }

  fn get_index_stp(&self) -> Result<IndexStp> {
    let url = self.url.join("index-stp.json")?;
    let body = get_text(url)?;
    Ok(IndexStp::new(body)?)
  }

  fn setup(&self, index_stp: IndexStp) {
    println!("IndexStp {:?}", index_stp);
    let mut ancestors = self.destiny.ancestors();
    println!("{:?}", ancestors.next());
    println!("{:?}", ancestors.next());
  }

  fn finish(&self) {
    fs::remove_dir_all(&self.temp).expect("Could not delete the temp path.");
  }
}

fn get_text(url: Url) -> Result<String> {
  let mut attempt = 1;
  loop {
    println!("Getting text attempt {} from {}", attempt, url);
    match files::download_text(url.as_ref()) {
      Ok(text) => return Ok(text),
      Err(e) => {
        println!("{}", e);
        if attempt > ATTEMPTS {
          return Err(e)
        }
      },
    }
    attempt += 1; 
  }
}

fn get_file(url: Url, file: String) -> Result<()> {
  let mut attempt = 1;
  loop {
    println!("Getting file attempt {} from {} to {}", attempt, url, file);
    match files::download_file(url.as_ref(), &file) {
      Ok(()) => return Ok(()),
      Err(e) => {
        println!("{}", e);
        if attempt > ATTEMPTS {
          return Err(e)
        }
      },
    }
    attempt += 1;
  }
}