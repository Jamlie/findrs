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