use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::rc::Rc;

macro_rules! find_dirs {
    ($path:expr, $all:expr) => {
        if $path.is_dir() {
            let dir = $path.display().to_string();
            get_dirs(&dir, &$all);
        }
    };

    ($path:expr, $all:expr, $type_:expr) => {
        match $type_ {
            "d" => {
                find_dirs!($path, $all);
            }
            _ => {}
        }
    };

    ($path:expr, $all:expr, $ignore:expr, $type_:expr) => {
        if $path.is_dir() {
            find_dirs!($path, $all);
            visit_dirs(&$path, $all, $ignore, $type_);
        }
    };
}

macro_rules! find_files {
    ($path:expr, $all:expr) => {
        if $path.is_file() {
            let file = $path.display().to_string();
            get_files(&file, &$all);
        }
    };

    ($path:expr, $all:expr, $type_:expr) => {
        match $type_ {
            "f" => find_files!($path, $all),
            _ => {}
        }
    };

    ($path:expr, $all:expr, $type_:expr, $has_type:expr) => {
        if $has_type && is_type_file($type_) {
            find_files!($path, $all, $type_);
        }
    };
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    ignore: Option<String>,

    #[arg(short, long)]
    type_: Option<String>,
}

fn main() {
    let args = Rc::new(Args::parse());
    let current_dir = Path::new(".");
    let ignore = args.ignore.clone().unwrap_or("".to_string());

    if args.recursive {
        visit_dirs(current_dir, &args.all, &ignore, &args.type_);
    } else {
        let args = Rc::clone(&args);
        process_find(current_dir, &args, &ignore);
    }
}

fn process_find(current_dir: &Path, args: &Args, ignore: &str) {
    for entry in current_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if entry.file_name() == *ignore {
                continue;
            }

            if let Some(type_) = &args.type_ {
                find_dirs!(path, &args.all, type_.as_str());
                find_files!(path, &args.all, type_.as_str());
            } else {
                find_dirs!(path, &args.all);
                find_files!(path, &args.all);
            }
        }
    }
}

fn is_type_file(type_: &str) -> bool {
    return type_ == "f";
}

fn visit_dirs(dir: &Path, all: &bool, ignore: &String, type_: &Option<String>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|_| panic!("read_dir call failed"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|_| panic!("entry call failed"));
        if entry.file_name() == **ignore {
            continue;
        }

        let path = entry.path();

        find_dirs!(path, all, ignore, type_);
        match type_ {
            Some(type_) => find_files!(path, all, type_.as_str(), false),
            None => find_files!(path, all),
        }
    }
}

fn get_dirs(path: &str, all: &bool) {
    let slash = "/".blue();
    let path = path.strip_prefix("./").unwrap();

    match path.starts_with(".") {
        true => {
            if *all {
                println!("{}{}", path.blue(), slash);
            }
        }
        _ => {
            println!("{}{}", path.blue(), slash);
        }
    }
}

fn get_files(path: &str, all: &bool) {
    let path = path.strip_prefix("./").unwrap();

    match path.starts_with(".") {
        true => {
            if *all {
                println!("{}", path.green());
            }
        }
        _ => {
            println!("{}", path.green());
        }
    }
}
