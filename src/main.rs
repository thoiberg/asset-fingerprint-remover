use regex::Regex;
use std::path::Path;
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() < 2 {
        eprint!("Must specify a directory to check\n")
    }

    let relative_directory_path = Path::new(&args[1]);

    println!("{:?}", relative_directory_path);
    println!("{:?}", relative_directory_path.exists());
    println!("{:?}", relative_directory_path.is_dir());

    if !relative_directory_path.exists() {
        eprint!("{} does not exist\n", relative_directory_path.display());
    }
    // accept param which is the directory
    for entry in WalkDir::new(relative_directory_path).into_iter() {
        match entry {
            Ok(file_path) => remove_asset_fingerprint(file_path),
            _ => eprintln!("Unable to find file, skipping"),
        }
    }
    // recursively list all files
    // rename each file by removing the asset fingerprint from the name
}

fn remove_asset_fingerprint(file_path: walkdir::DirEntry) {
    let re = Regex::new(r"-[a-z0-9]{64}").unwrap();

    if file_path.path().is_file() {
        println!("{:?}", file_path.path());
        let new_path = re
            .replace(file_path.path().to_str().unwrap(), "")
            .into_owned();

        println!("{}", new_path);
        match fs::rename(file_path.path(), Path::new(&new_path)) {
            Ok(_) => println!("Renamed {} to {}", file_path.path().display(), new_path),
            Err(err) => eprint!("Oh no: {}", err),
        }
    }
}
