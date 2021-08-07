use std::env;
use std::fs::File;
use std::io;
extern crate reqwest;

const URL_ROOT: &str = "http://www.pointel.com.br/qinpel";

fn get_os<'a>() -> &'a str {
    let os = env::consts::OS;
    if os.len() < 3 {
        panic!("Error: Operation system is not supported.");
    }
    let result = &os[..3];
    match result {
        "lin" | "mac" | "win" => result,
        _ => panic!("Error: Operation system is not supported."),
    }
}

fn get_arch<'a>() -> &'a str {
    match std::mem::size_of::<&char>() {
        4 => "32",
        8 => "64",
        _ => panic!("Error: System architecture is not supported."),
    }
}

use std::io::Cursor;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn download(origin: String, destiny: String) -> Result<()> {
    let response = reqwest::get(origin).await?;
    let mut file = std::fs::File::create(destiny)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

fn setup(path: String) {
    // TODO
}

fn setup_app(name: &str) {
    setup(format!("{}/{}/{}", URL_ROOT, "apps", name));
}

fn setup_cmd(os: &str, arch: &str, name: &str) {
    setup(format!("{}/{}/{}/{}/{}", URL_ROOT, "cmds", os, arch, name));
}

#[tokio::main]
async fn main() {
    println!("Qinpel Setup starting...");
    let os = get_os();
    let arch = get_arch();
    println!("Identified operation system: {}", os);
    println!("Identified system architecture: {}", arch);
    for (index, argument) in env::args().enumerate() {
        if index == 0 {
            continue;
        }
        if argument.len() < 6 {
            println!(
                "Error: Can not setup this mal formed argument: {}.",
                argument
            );
            continue;
        }
        let name = &argument[5..];
        if argument.starts_with("apps/") {
            println!("Setting up application: {}.", name);
            setup_app(name);
        } else if argument.starts_with("cmds/") {
            println!("Setting up command: {}.", name);
            setup_cmd(os, arch, name);
        } else if argument.starts_with("tes") {
            download(String::from("http://www.uol.com.br/"), String::from("./uol")).await;
        } else {
            println!(
                "Error: Can not setup this mal formed argument: {}.",
                argument
            );
        }
    }
}
