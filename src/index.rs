use super::data;
use super::files;
use std::path::Path;
use std::path::PathBuf;

pub fn run(origin: &str) {
    println!("Indexing: '{}'...", origin);
    let mut index = data::IndexStp { files: Vec::new() };
    let origin_path = Path::new(origin);
    let mut destiny = Path::new("index-stp.json").to_owned();
    if origin_path.is_dir() {
        destiny = origin_path.join("index-stp.json");
        index_dir(origin, origin_path.to_owned(), &mut index);
    } else {
        index_file(origin, origin_path.to_owned(), &mut index);
    }
    index
        .save(destiny)
        .expect("Could not save the index file.'");
}

fn index_dir(origin: &str, path: PathBuf, index: &mut data::IndexStp) {
    let entries: Vec<_> = path
        .read_dir()
        .expect(&format!(
            "Error: Could not read index directory: '{}'.",
            path.display()
        ))
        .collect();
    for entry in &entries {
        if let Ok(entry) = entry.as_ref() {
            let inside = entry.path();
            if !inside.is_dir() {
                index_file(origin, inside, index);
            }
        }
    }
    for entry in &entries {
        if let Ok(entry) = entry.as_ref() {
            let inside = entry.path();
            if inside.is_dir() {
                index_dir(origin, inside, index);
            }
        }
    }
}

fn index_file(origin: &str, path: PathBuf, index: &mut data::IndexStp) {
    let file = format!("{}", path.display());
    let file_path = String::from(&file[origin.len() + 1..]);
    let file_verifier = files::get_verifier(&file).expect(&format!(
        "Error: Could not get the verifier from: '{}'.",
        path.display()
    ));
    let file = data::FileStp {
        path: file_path,
        verifier: file_verifier,
    };
    index.files.push(file);
}
