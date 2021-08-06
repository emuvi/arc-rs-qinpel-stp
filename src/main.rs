use std::env;
use std::fs::File;
use std::io;
extern crate reqwest;

fn get_os() -> String {
    let os = env::consts::OS;
    if os.len() < 3 {
        panic!("Error: Operation system is not supported.");
    }
    let result = &os[..3];
    match result {
        "lin" | "mac" | "win" => String::from(result),
        _ => panic!("Error: Operation system is not supported."),
    }
}

fn get_arch() -> String {
    match std::mem::size_of::<&char>() {
        4 => String::from("32"),
        8 => String::from("64"),
        _ => panic!("Error: System architecture is not supported."),
    }
}

fn setup(path: String) {
    let mut client = reqwest::Client::new();
    let mut request = client.get(path).send();
    // let mut out = File::create("rustup-init.sh").expect("failed to create file");
    // io::copy(&mut request, &mut out).expect("failed to copy content");
}

fn setupApp(name: &String) {}

fn setupCmd(os: &String, arch: &String, name: &String) {}

fn main() {
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
            println!("Error: Can not setup. Mal formed argument: {}.", argument);
            continue;
        }
        let name = String::from(&argument[5..]);
        if argument.starts_with("apps/") {
            println!("Setting up application: {}", name);
            setupApp(&name);
        } else if argument.starts_with("cmds/") {
            println!("Setting up command: {}", name);
            setupCmd(&os, &arch, &name);
        } else {
            println!("Error: Can not setup. Mal formed argument: {}.", argument);
        }
    }
}
