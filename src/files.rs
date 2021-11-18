use futures_util::StreamExt;
use reqwest;
use sha2;

use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use super::Result;

pub fn download_text(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    Ok(body)
}

pub async fn download_file(url: &str, file: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let mut file = File::create(file)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write(&chunk)?;
        downloaded = downloaded + (chunk.len() as u64);
        println!("Downloaded {} bytes from {}", downloaded, url);
    }
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
