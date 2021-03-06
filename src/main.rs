use std::process::Command;
use std::time::Duration;

mod clip;
mod data;
mod files;
mod index;
mod install;
mod utils;

const URL_MAIN: &str = "http://www.pointel.com.br/qinpel";
const SEPARATOR: char = std::path::MAIN_SEPARATOR;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
	let args = clip::parse();
	println!("QinpelStp starting...");
	let os = if args.is_present("os") {
		args.value_of("os")
			.expect("Could not parse the OS argument.")
	} else {
		utils::get_os()
	};
	let arch = if args.is_present("arch") {
		args.value_of("arch")
			.expect("Could not parse the ARCH argument.")
	} else {
		utils::get_arch()
	};
	println!("Identified operation system: {}", os);
	println!("Identified system architecture: {}", arch);
	utils::check_os_arch(os, arch);
	let wait_str = args.value_of("wait").expect("You must pass a wait time.");
	let wait = wait_str.parse().expect("You must pass a valid wait time.");
	if wait > 0 {
		println!("Waiting {} milliseconds to execute.", wait);
		std::thread::sleep(Duration::from_millis(wait));
	}
	if let Some(argument) = args.value_of("index") {
		index::run(argument);
	}
	if let Some(argument) = args.value_of("install") {
		if argument.len() < 6 {
			println!(
				"Error: Can not install this very small argument: {}",
				argument
			);
		}
		let name = &argument[4..];
		if argument.starts_with("app/") {
			println!("Installing application: {}", name);
			install_app(name).await;
		} else if argument.starts_with("cmd/") {
			println!("Installing command: {}", name);
			install_cmd(os, arch, name).await;
		} else {
			println!(
				"Error: Can not install this mal formed argument: {}",
				argument
			);
		}
	}
	if let Some(argument) = args.value_of("run") {
		run_cmd(argument);
	}
	if let Some(argument) = args.value_of("install-run") {
		install_run(os, arch, argument).await;
	}
	Ok(())
}

async fn install_app(name: &str) {
	let url_root = format!("{}/{}/{}/", URL_MAIN, "app", name);
	let dir_root = format!("{}{}{}{}{}", "run", SEPARATOR, "app", SEPARATOR, name);
	install(url_root, dir_root).await;
}

async fn install_cmd(os: &str, arch: &str, name: &str) {
	let url_root = format!("{}/{}/{}/{}/{}/", URL_MAIN, "cmd", os, arch, name);
	let dir_root = format!("{}{}{}{}{}", "run", SEPARATOR, "cmd", SEPARATOR, name);
	install(url_root, dir_root).await;
}

async fn install(url_root: String, dir_root: String) {
	let url_path = url::Url::parse(&url_root).expect("Error: Could not parse the url root");
	let current_dir = std::env::current_dir().expect("Error: Could not retrieve the current dir");
	let destiny_path = current_dir.join(&dir_root);
	let temp_path = current_dir.join("tmp");
	println!("Install url path: {}", url_path);
	println!("Install destiny path: {}", destiny_path.display());
	println!("Install temp path: {}", temp_path.display());
	install::Boss::new(url_path, destiny_path, temp_path).run().await;
}

fn run_cmd(name: &str) {
	let exec_extension = utils::get_exec_extension();
	let full_name = format!(
		"{}{}",
		name,
		if !name.ends_with(exec_extension) {
			exec_extension
		} else {
			""
		}
	);
	println!("Running command: {}", full_name);
	let current_dir = std::env::current_dir().expect("Error: Could not retrieve the current dir");
	let dir_root = format!("{}{}{}{}{}", "run", SEPARATOR, "cmd", SEPARATOR, name);
	let full_dir = current_dir.join(&dir_root);
	let full_path = full_dir.join(&full_name);
	let full_call = format!("{}", full_path.display());
	if full_path.exists() {
		println!("Calling full path: {}", full_call);
		Command::new(full_call)
			.current_dir(full_dir)
			.spawn()
			.unwrap();
	} else {
		println!("Calling short path: {}", full_name);
		Command::new(full_name).spawn().unwrap();
	}
}

async fn install_run(os: &str, arch: &str, name: &str) {
	install_cmd(os, arch, name).await;
	run_cmd(name);
}
