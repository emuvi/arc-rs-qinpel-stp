extern crate url;
extern crate simple_error;

use super::files;
use super::utils;
use std::fs;
use std::path::PathBuf;
use url::Url;


use super::data::IndexStp;
use super::Result;

const ATTEMPTS: usize = 3;

pub struct Boss {
    url: Url,
    destiny: PathBuf,
    temp: PathBuf,
    destiny_parent: PathBuf,
    destiny_backup: PathBuf,
}

impl Boss {
    pub fn new(url: Url, destiny: PathBuf, temp: PathBuf) -> Boss {
        let destiny_name = destiny
            .file_name()
            .expect("error: could not get the name of the destiny")
            .to_str()
            .expect("error: could not convert to str the file name of the destiny");
        let destiny_parent = destiny
            .parent()
            .expect("error: could not get the parent of the destiny")
            .to_owned();
        let mut destiny_backup = destiny_name.to_owned();
        destiny_backup.push_str("_bkp");
        let destiny_backup = destiny_parent.join(destiny_backup);
        Boss {
            url,
            destiny,
            temp,
            destiny_parent,
            destiny_backup,
        }
    }

    pub fn run(&self) {
        self.start_temp();
        match self.get_index_stp() {
            Ok(index_stp) => {
                match self.setup(index_stp) {
                    Err(e) => println!("{}", e),
                    _ => (),
                }
            },
            Err(e) => println!("{}", e),
        }
        self.finish();
    }

    fn start_temp(&self) {
        if self.temp.exists() {
            panic!("error: there's already a temp path");
        }
        fs::create_dir_all(&self.temp).expect("error: could not create the temp path");
    }

    fn get_index_stp(&self) -> Result<IndexStp> {
        let url = self.url.join("index-stp.json")?;
        let body = get_text(url)?;
        Ok(IndexStp::new(body)?)
    }

    fn setup(&self, index_stp: IndexStp) -> Result<()> {
        self.backup_actual()?;
        Ok(())
    }

    fn backup_actual(&self) -> Result<()> {
        if self.destiny_backup.exists() {
            return Err(utils::get_err("// TODO"))
        }
        Ok(())
    }

    fn restore_actual(&self) -> Result<()> {
        Ok(())
    }

    fn finish(&self) {
        fs::remove_dir_all(&self.temp).expect("error: could not delete the temp path");
    }
}

fn get_text(url: Url) -> Result<String> {
    let mut attempt = 1;
    loop {
        println!("getting text attempt {} from {}", attempt, url);
        match files::download_text(url.as_ref()) {
            Ok(text) => return Ok(text),
            Err(e) => {
                println!("{}", e);
                if attempt > ATTEMPTS {
                    return Err(e);
                }
            }
        }
        attempt += 1;
    }
}

fn get_file(url: Url, file: String) -> Result<()> {
    let mut attempt = 1;
    loop {
        println!("getting file attempt {} from {} to {}", attempt, url, file);
        match files::download_file(url.as_ref(), &file) {
            Ok(()) => return Ok(()),
            Err(e) => {
                println!("{}", e);
                if attempt > ATTEMPTS {
                    return Err(e);
                }
            }
        }
        attempt += 1;
    }
}
