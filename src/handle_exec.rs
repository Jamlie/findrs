//! This module contains the functions that are used to handle the execution of the program to find
//! files and directories.

use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::{find_dirs, find_files, Args};

/// Visits the directories recursively to find all files and directories
pub fn visit_dirs(dir: &Path, args: &Args) {
    let entries = fs::read_dir(dir).unwrap_or_else(|_| panic!("read_dir call failed"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|_| panic!("entry call failed"));
        if entry.file_name() == *args.ignore.clone().unwrap_or("".to_string()) {
            continue;
        }

        let path = entry.path();

        find_dirs!(&path, &args);
        match &args.type_ {
            Some(type_) => find_files!(path, &args.all, type_.as_str(), &args.name),
            None => find_files!(path, &args.all, &args.name),
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

/// Processes the findrs command with the given arguments, not recursively
pub fn process_find(current_dir: &Path, args: &Args) -> Vec<String> {
    let mut results = vec![];
    for entry in current_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if entry.file_name() == *args.ignore.clone().unwrap_or("".to_string()) {
                continue;
            }

            if let Some(type_) = &args.type_ {
                find_dirs!(path, &args.all, type_.as_str(), &args.name);
                find_files!(path, &args.all, type_.as_str(), &args.name);
            } else {
                find_dirs!(path, &args.all, &args.name);
                find_files!(path, &args.all, &args.name);
            }

            results.push(path.display().to_string());
        }
    }

    results
}
