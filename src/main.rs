use clap::Parser;
use handle_exec::{process_find, visit_dirs};
use std::path::Path;
use std::rc::Rc;

mod findrs_macro;
mod handle_exec;
mod handle_exec_test;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Show hidden files
    #[arg(short, long)]
    all: bool,

    /// Search recursively
    #[arg(short, long)]
    recursive: bool,

    /// Ignore a specific file or directory
    #[arg(short, long)]
    ignore: Option<String>,

    /// Type of file or directory (f or d)
    #[arg(short, long)]
    type_: Option<String>,

    /// Search by name
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
