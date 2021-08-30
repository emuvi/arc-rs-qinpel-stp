use std::process::Command;
use std::time::Duration;

mod clip;
mod data;
mod files;
mod install;
mod utils;

const URL_MAIN: &str = "http://www.pointel.com.br/qinpel";
const SEPARATOR: char = std::path::MAIN_SEPARATOR;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
	println!("QinpelStp starting...");
	let os = utils::get_os();
	let arch = utils::get_arch();
	println!("Identified operation system: {}", os);
	println!("Identified system architecture: {}", arch);
	let clip = clip::run();
	let wait_str = clip.value_of("wait").expect("You must pass a wait time.");
	let wait = wait_str.parse().expect("You must pass a valid wait time.");
	println!("Waiting {} milliseconds to execute.", wait);
	std::thread::sleep(Duration::from_millis(wait));
	if let Some(argument) = clip.value_of("install") {
		if argument.len() < 6 {
			println!(
				"Error: Can not install this very small argument: {}",
				argument
			);
		}
		let name = &argument[4..];
		if argument.starts_with("app/") {
			println!("Installing application: {}", name);
			install_app(name);
		} else if argument.starts_with("cmd/") {
			println!("Installing command: {}", name);
			install_cmd(os, arch, name);
		} else {
			println!(
				"Error: Can not install this mal formed argument: {}",
				argument
			);
		}
	}
	if let Some(argument) = clip.value_of("run") {
		run_cmd(os, argument);
	}
}

fn install_app(name: &str) {
	let url_root = format!("{}/{}/{}/", URL_MAIN, "apps", name);
	let dir_root = format!("{}{}{}{}{}", "run", SEPARATOR, "apps", SEPARATOR, name);
	install(url_root, dir_root);
}

fn install_cmd(os: &str, arch: &str, name: &str) {
	let url_root = format!("{}/{}/{}/{}/{}/", URL_MAIN, "cmds", os, arch, name);
	let dir_root = format!("{}{}{}{}{}", "run", SEPARATOR, "cmds", SEPARATOR, name);
	install(url_root, dir_root);
}

fn install(url_root: String, dir_root: String) {
	let url_path = url::Url::parse(&url_root).expect("Error: Could not parse the url root");
	let current_dir = std::env::current_dir().expect("Error: Could not retrieve the current dir");
	let destiny_path = current_dir.join(&dir_root);
	let temp_path = current_dir.join("tmp");
	println!("Install url path: {}", url_path);
	println!("Install destiny path: {}", destiny_path.display());
	println!("Install temp path: {}", temp_path.display());
	install::Boss::new(url_path, destiny_path, temp_path).run();
}

fn run_cmd(os: &str, name: &str) {
	let os_extension = utils::get_exec_extension(os);
	let full_name = format!(
		"{}{}",
		name,
		if !name.ends_with(os_extension) {
			os_extension
		} else {
			""
		}
	);
	println!("Running command: {}", full_name);
	let current_dir = std::env::current_dir().expect("Error: Could not retrieve the current dir");
	let dir_root = format!("{}{}{}{}{}", "run", SEPARATOR, "cmds", SEPARATOR, name);
	let full_dir = current_dir.join(&dir_root);
	let full_path = full_dir.join(&full_name);
	let full_call = format!("{}", full_path.display());
	if full_path.exists() {
		println!("Calling full path: {}", full_call);
		Command::new(full_call).current_dir(full_dir).spawn().unwrap();
	} else {
		println!("Calling short path: {}", full_name);
		Command::new(full_name).spawn().unwrap();
	}
}
