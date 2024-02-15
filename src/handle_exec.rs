use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::{find_dirs, find_files};

fn is_type_file(type_: &str) -> bool {
    return type_ == "f";
}

pub fn visit_dirs(
    dir: &Path,
    all: &bool,
    ignore: &String,
    type_: &Option<String>,
    name: &Option<String>,
) {
    let entries = fs::read_dir(dir).unwrap_or_else(|_| panic!("read_dir call failed"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|_| panic!("entry call failed"));
        if entry.file_name() == **ignore {
            continue;
        }

        let path = entry.path();

        find_dirs!(path, all, ignore, type_, name);
        match type_ {
            Some(type_) => find_files!(path, all, type_.as_str(), false, name),
            None => find_files!(path, all, name),
        }
    }
}

pub fn get_dirs(path: &str, all: &bool, name: &Option<String>) {
    let slash = "/".blue();
    let path = path.strip_prefix("./").unwrap();

    match path.starts_with(".") {
        true => {
            if *all {
                if let Some(name) = name {
                    if path.contains(name) {
                        println!("{}{}", path.blue(), slash);
                    }
                } else {
                    println!("{}{}", path.blue(), slash);
                }
            }
        }
        _ => {
            if let Some(name) = name {
                if path.contains(name) {
                    println!("{}{}", path.blue(), slash);
                }
            } else {
                println!("{}{}", path.blue(), slash);
            }
        }
    }
}

pub fn get_files(path: &str, all: &bool, name: &Option<String>) {
    let path = path.strip_prefix("./").unwrap();

    match path.starts_with(".") {
        true => {
            if *all {
                if let Some(name) = name {
                    if path.contains(name) {
                        println!("{}", path.green());
                    }
                } else {
                    println!("{}", path.green());
                }
            }
        }
        _ => {
            if let Some(name) = name {
                if path.contains(name) {
                    println!("{}", path.green());
                }
            } else {
                println!("{}", path.green());
            }
        }
    }
}
