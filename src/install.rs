use url;
use url::Url;
use std::fs;
use std::path::PathBuf;
use super::data::FileStp;
use super::data::IndexStp;
use super::files;
use super::utils;
use super::Result;

const ATTEMPTS: usize = 5;

pub struct Boss {
	url: Url,
	destiny: PathBuf,
	temp: PathBuf,
	destiny_backup: PathBuf,
}

impl Boss {
	pub fn new(url: Url, destiny: PathBuf, temp: PathBuf) -> Boss {
		let destiny_name = destiny
			.file_name()
			.expect("Error: Could not get the name of the destiny.")
			.to_str()
			.expect("Error: Could not convert to str the file name of the destiny.");
		let destiny_parent = destiny
			.parent()
			.expect("Error: Could not get the parent of the destiny.")
			.to_owned();
		let mut destiny_backup = destiny_name.to_owned();
		destiny_backup.push_str("_bkp");
		let destiny_backup = destiny_parent.join(destiny_backup);
		Boss {
			url,
			destiny,
			temp,
			destiny_backup,
		}
	}

	pub fn run(&self) {
		self.start();
		match self.get_index_stp() {
			Ok(index_stp) => match self.setup(index_stp) {
				Err(e) => eprintln!("{}", e),
				_ => (),
			},
			Err(e) => eprintln!("{}", e),
		}
		self.finish();
	}

	fn start(&self) {
		if self.temp.exists() {
			panic!("Error: There's already a temp path.");
		}
		fs::create_dir_all(&self.temp).expect("Error: Could not create the temp path.");
	}

	fn get_index_stp(&self) -> Result<IndexStp> {
		let url = self.url.join("index-stp.json")?;
		let body = try_get_text(url)?;
		println!("index_stp: {}", body);
		Ok(IndexStp::new(body)?)
	}

	fn setup(&self, index_stp: IndexStp) -> Result<()> {
		self.backup_actual()?;
		match self.get_files(&index_stp.files) {
			Err(e) => {
				eprintln!("{}", e);
				self.restore_backup()?;
				return Ok(());
			}
			_ => (),
		}
		match self.put_files() {
			Err(e) => {
				eprintln!("{}", e);
				self.restore_backup()?;
				return Ok(());
			}
			_ => (),
		}
		self.remove_backup()?;
		Ok(())
	}

	fn get_files(&self, files: &Vec<FileStp>) -> Result<()> {
		for file in files {
			self.get_file(file)?;
		}
		Ok(())
	}

	fn get_file(&self, file_stp: &FileStp) -> Result<()> {
		let mut origin_path = file_stp.path.clone();
		if origin_path.contains("\\") {
			origin_path = origin_path.replace("\\", "/");
		}
		let origin = self.url.join(&origin_path)?;
		let mut destiny_path = file_stp.path.clone();
		if super::SEPARATOR != '/' && destiny_path.contains("/") {
			destiny_path = destiny_path.replace("/", &super::SEPARATOR.to_string());
		}
		if super::SEPARATOR != '\\' && destiny_path.contains("\\") {
			destiny_path = destiny_path.replace("\\", &super::SEPARATOR.to_string());
		}
		let destiny_path = self.temp.join(destiny_path);
		let destiny = destiny_path.to_string_lossy();
		println!("Getting file from '{}' to '{}'...", origin, destiny);
		try_get_file(origin, destiny.as_ref(), &file_stp.verifier)?;
		Ok(())
	}

	fn put_files(&self) -> Result<()> {
		println!(
			"Copying the files from '{}' to '{}'...",
			self.temp.display(),
			self.destiny.display()
		);
		files::copy_dir_all(&self.temp, &self.destiny)?;
		Ok(())
	}

	fn backup_actual(&self) -> Result<()> {
		println!("Making backup of the actual installation...");
		if self.destiny_backup.exists() {
			return Err(utils::get_err("Error: The destiny backup already exists."));
		}
		if self.destiny.exists() {
			fs::rename(&self.destiny, &self.destiny_backup)?;
		}
		Ok(())
	}

	fn restore_backup(&self) -> Result<()> {
		println!("Restoring backup from the actual installation...");
		if !self.destiny_backup.exists() {
			return Err(utils::get_err("Error: Didn't have a backup to restore."));
		}
		if self.destiny.exists() {
			fs::remove_dir_all(&self.destiny)?;
		}
		fs::rename(&self.destiny_backup, &self.destiny)?;
		Ok(())
	}

	fn remove_backup(&self) -> Result<()> {
		println!("Removing backup of the actual installation...");
		if self.destiny_backup.exists() {
			fs::remove_dir_all(&self.destiny_backup)?;
		}
		Ok(())
	}

	fn finish(&self) {
		println!("Finishing install...");
		if self.temp.exists() {
			fs::remove_dir_all(&self.temp).expect("Error: Could not delete the temp path.");
		}
	}
}

fn try_get_text(url: Url) -> Result<String> {
	let mut attempt = 1;
	loop {
		println!("Getting text attempt {} from '{}'...", attempt, url);
		match files::download_text(url.as_ref()) {
			Ok(text) => return Ok(text),
			Err(e) => {
				if attempt >= ATTEMPTS {
					return Err(e);
				}
				println!("{}", e);
			}
		}
		attempt += 1;
	}
}

fn try_get_file(url: Url, file: &str, verifier: &str) -> Result<()> {
	let mut attempt = 1;
	loop {
		println!(
			"Getting file attempt {} from '{}' to '{}'...",
			attempt, url, file
		);
		match files::download_file(url.as_ref(), &file) {
			Ok(()) => match files::get_verifier(&file) {
				Ok(verify) => {
					if verify != verifier {
						let e = utils::get_err(
							"Error: The download file doesn't check with the verifier.",
						);
						if attempt >= ATTEMPTS {
							return Err(e);
						}
						eprintln!("{}", e);
					} else {
						return Ok(());
					}
				}
				Err(e) => {
					if attempt >= ATTEMPTS {
						return Err(e);
					}
					eprintln!("{}", e);
				}
			},
			Err(e) => {
				if attempt >= ATTEMPTS {
					return Err(e);
				}
				eprintln!("{}", e);
			}
		}
		attempt += 1;
	}
}
