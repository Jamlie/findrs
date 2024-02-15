use clap::Parser;
use handle_exec::visit_dirs;
use std::path::Path;
use std::rc::Rc;

mod findrs_macro;
mod handle_exec;

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

    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Rc::new(Args::parse());
    let current_dir = Path::new(".");
    let ignore = args.ignore.clone().unwrap_or("".to_string());

    if args.recursive {
        visit_dirs(current_dir, &args.all, &ignore, &args.type_, &args.name);
    } else {
        let args = Rc::clone(&args);
        process_find(current_dir, &args, &ignore, &args.name);
    }
}

fn process_find(current_dir: &Path, args: &Args, ignore: &str, name: &Option<String>) {
    for entry in current_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if entry.file_name() == *ignore {
                continue;
            }

            if let Some(type_) = &args.type_ {
                find_dirs!(path, &args.all, type_.as_str(), name);
                find_files!(path, &args.all, type_.as_str(), name);
            } else {
                find_dirs!(path, &args.all, &args.name);
                find_files!(path, &args.all, name);
            }
        }
    }
}
