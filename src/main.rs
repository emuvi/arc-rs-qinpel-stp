extern crate url;

mod boss;
mod data;
mod files;
mod utils;

const URL_MAIN: &str = "http://www.pointel.com.br/qinpel";
const SEPARATOR: char = std::path::MAIN_SEPARATOR;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn setup(url_root: String, file_root: String) {
    let url_path = url::Url::parse(&url_root).expect("error: could not parse the url root");
    let current_dir = std::env::current_dir().expect("error: could not retrieve the current dir");
    let destiny_path = current_dir.join(&file_root);
    let temp_path = current_dir.join("tmp");
    println!("setup url path: {}", url_path);
    println!("setup destiny path: {}", destiny_path.display());
    println!("setup temp path: {}", temp_path.display());
    boss::Boss::new(url_path, destiny_path, temp_path).run();
}

fn setup_app(name: &str) {
    let url_root = format!("{}/{}/{}/", URL_MAIN, "apps", name);
    let file_root = format!("{}{}{}{}{}", "run", SEPARATOR, "apps", SEPARATOR, name);
    setup(url_root, file_root);
}

fn setup_cmd(os: &str, arch: &str, name: &str) {
    let url_root = format!("{}/{}/{}/{}/{}/", URL_MAIN, "cmds", os, arch, name);
    let file_root = format!(
        "{}{}{}{}{}{}{}{}{}",
        "run", SEPARATOR, "cmds", SEPARATOR, os, SEPARATOR, arch, SEPARATOR, name
    );
    setup(url_root, file_root);
}

fn main() {
    println!("Qinpel setup starting...");
    let os = utils::get_os();
    let arch = utils::get_arch();
    println!("identified operation system: {}", os);
    println!("identified system architecture: {}", arch);
    for (index, argument) in std::env::args().enumerate() {
        if index == 0 {
            continue;
        }
        if argument.len() < 6 {
            println!(
                "error: can not setup this very small argument: {}",
                argument
            );
            continue;
        }
        let name = &argument[5..];
        if argument.starts_with("apps/") {
            println!("setting up application: {}", name);
            setup_app(name);
        } else if argument.starts_with("cmds/") {
            println!("setting up command: {}", name);
            setup_cmd(os, arch, name);
        } else {
            println!(
                "error: can not setup this mal formed argument: {}",
                argument
            );
        }
    }
}
