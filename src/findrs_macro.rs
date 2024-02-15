//! # findrs_macro
//! This module contains the macros used to find files and directories based on patterns in the
//! code.

/// # find_dirs
/// Find directories based on the pattern provided
/// It takes one of the following implementations:
/// ````
/// find_dirs!(path, all, name)
/// find_dirs!(path, all, type_, name)
/// find_dirs!(path, all, ignore, type_, name)
/// ````
#[macro_export]
macro_rules! find_dirs {
    ($path:expr, $all:expr, $name:expr) => {
        if $path.is_dir() {
            let dir = $path.display().to_string();
            $crate::handle_exec::get_dirs(&dir, &$all, $name);
        }
    };

    ($path:expr, $all:expr, $type_:expr, $name:expr) => {
        match $type_ {
            "d" => {
                find_dirs!($path, $all, $name);
            }
            _ => {}
        }
    };

    ($path:expr, $all:expr, $ignore:expr, $type_:expr, $name:expr) => {
        if $path.is_dir() {
            find_dirs!($path, $all, $name);
            $crate::handle_exec::visit_dirs(&$path, $all, $ignore, $type_, $name);
        }
    };
}

/// # find_files
/// Finds files based on the pattern provided
/// It takes one of the following implementations:
/// ````
/// find_files!(path, all, name)
/// find_files!(path, all, type_, name)
/// find_files!(path, all, ignore, type_, name)
/// ````
#[macro_export]
macro_rules! find_files {
    ($path:expr, $all:expr, $name:expr) => {
        if $path.is_file() {
            let file = $path.display().to_string();
            $crate::handle_exec::get_files(&file, &$all, $name);
        }
    };

    ($path:expr, $all:expr, $type_:expr, $name:expr) => {
        match $type_ {
            "f" => find_files!($path, $all, $name),
            _ => {}
        }
    };

    ($path:expr, $all:expr, $type_:expr, $has_type:expr, $name:expr) => {
        if $has_type && is_type_file($type_) {
            find_files!($path, $all, $type_, $name);
        }
    };
}
