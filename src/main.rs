use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    ignore: Option<String>,
}

fn main() {
    let args = Args::parse();
    let current_dir = Path::new(".");
    let ignore = args.ignore.unwrap_or("".to_string());
    if args.recursive {
        visit_dirs(current_dir, &args.all, &ignore);
    } else {
        for entry in current_dir.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if entry.file_name() == *ignore {
                    continue;
                }
                if path.is_dir() {
                    let dir = path.display().to_string();
                    get_dirs(&dir, &args.all);
                } else if path.is_file() {
                    let file = path.display().to_string();
                    get_files(&file, &args.all);
                }
            }
        }
    }
}

fn visit_dirs(dir: &Path, all: &bool, ignore: &String) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if entry.file_name() == **ignore {
                        continue;
                    }
                    if !*all {
                        if is_hidden(&path.display().to_string()) {
                            continue;
                        }
                    }
                    let dir = path.display().to_string();
                    get_dirs(&dir, all);
                    visit_dirs(&path, all, ignore);
                } else if path.is_file() {
                    if entry.file_name() == **ignore {
                        continue;
                    }
                    if !*all {
                        if is_hidden(&path.display().to_string()) {
                            continue;
                        }
                    }
                    let file = path.display().to_string();
                    get_files(&file, all);
                }
            }
        }
    }
}

fn is_hidden(path: &String) -> bool {
    let name = path.split("/").last().unwrap();
    if name.starts_with(".") {
        return true;
    }
    false
}

fn get_dirs(path: &String, all: &bool) {
    let slash = "/".blue();
    if path.starts_with("./") {
        let path = path.strip_prefix("./").unwrap();
        if is_hidden(&path.to_string()) {
            if *all {
                println!("{}{}", path.blue(), slash);
            }

            return;
        }

        println!("{}{}", path.blue(), slash);
        return;
    }
    println!("{}{}", path.blue(), slash);
}

fn get_files(path: &String, all: &bool) {
    if path.starts_with("./") {
        let path = path.strip_prefix("./").unwrap();
        if path.starts_with(".") {
            if *all {
                println!("{}", path.green());
            }
        } else {
            println!("{}", path.green());
        }
    } else {
        println!("{}", path.green());
    }
}
