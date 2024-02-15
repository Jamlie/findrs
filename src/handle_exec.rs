//! This module contains the functions that are used to handle the execution of the program to find
//! files and directories.

use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::{find_dirs, find_files};

/// Checks if the type is a file (f)
fn is_type_file(type_: &str) -> bool {
    return type_ == "f";
}

// Visits the directories recursively to find all files and directories
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

/// Gets the directories either hidden or not, based on what is passed.
pub fn get_dirs(path: &str, all: &bool, name: &Option<String>) {
    let path = path.strip_prefix("./").unwrap();

    fn print_path(path: &str, name: &Option<String>) {
        let slash = "/".blue();
        if let Some(name) = name {
            let dir_name = path.split("/").last().unwrap_or(path);
            if dir_name.contains(name) {
                println!("{}{}", path.blue(), slash);
            }
        } else {
            println!("{}{}", path.blue(), slash);
        }
    }

    match path.starts_with(".") {
        true => {
            if *all {
                print_path(path, name);
            }
        }
        _ => {
            print_path(path, name);
        }
    }
}

/// Gets the files either hidden or not, based on what is passed.
pub fn get_files(path: &str, all: &bool, name: &Option<String>) {
    let path = path.strip_prefix("./").unwrap();

    fn print_path(path: &str, name: &Option<String>) {
        if let Some(name) = name {
            let file_name = path.split("/").last().unwrap_or(path);
            if file_name.contains(name) {
                println!("{}", path.green());
            }
        } else {
            println!("{}", path.green());
        }
    }

    match path.starts_with(".") {
        true => {
            if *all {
                print_path(path, name);
            }
        }
        _ => {
            print_path(path, name);
        }
    }
}
