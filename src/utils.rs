use simple_error::SimpleError;

use std::env;
use std::path::PathBuf;

pub fn get_os() -> &'static str {
    let os = env::consts::OS;
    if os.len() < 3 {
        panic!("Error: Operation system is not supported.");
    }
    return &os[..3];
}

pub fn get_arch() -> &'static str {
    let arch = env::consts::ARCH;
    let size = arch.chars().count();
    let ends = &arch[size - 2..];
    match ends {
        "64" => "64",
        _ => "32",
    }
}

pub fn check_os_arch(os: &str, arch: &str) {
    if os == "mac" && arch != "64" {
        panic!("Error: The Mac OS is only supported on 64 ARCH.")
    }
    match os {
        "lin" | "win" | "mac" => (),
        _ => panic!("Error: Operation system is not supported."),
    }
    match arch {
        "32" | "64" => (),
        _ => panic!("Error: Architecture is not supported."),
    }
}

pub fn get_exec_extension() -> &'static str {
    env::consts::EXE_EXTENSION
}

pub fn get_err(description: &str) -> Box<dyn std::error::Error> {
    Box::new(SimpleError::new(description))
}

#[cfg(windows)]
pub fn is_executable(path: &PathBuf) -> bool {
    if path.is_file() {
        let extension = path.extension().and_then(std::ffi::OsStr::to_str);
        if let Some(extension) = extension {
            let extension = String::from(extension).to_lowercase();
            match &extension[..] {
                "exe" | "bat" | "com" => return true,
                _ => (),
            }
        }
    }
    false
}

#[cfg(unix)]
pub fn is_executable(path: &PathBuf) -> bool {
    if (path.is_file()) {
        use std::os::unix::fs::MetadataExt;
        let meta = std::fs::metadata(path);
        let mode = meta.mode();
        return mode & 0o111;
    }
    false
}
#[cfg(windows)]
pub fn set_executable(_path: &PathBuf) {}

#[cfg(unix)]
pub fn set_executable(path: &PathBuf) {
    std::fs::set_permissions(destiny, std::os::unix::fs::PermissionsExt::from_mode(0o753));
}
