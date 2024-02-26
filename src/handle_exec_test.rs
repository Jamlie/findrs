#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{handle_exec::*, Args};

    #[test]
    fn test_process_find() {
        let path = Path::new("./tmp");
        let args = Args {
            all: true,
            recursive: false,
            ignore: None,
            type_: None,
            name: None,
        };
        let ignore = "";
        let name: &Option<String> = &None;

        let dirs_and_files = process_find(path, &args, ignore, name);

        let expected = vec![
            "./tmp/.hidden.txt",
            "./tmp/3.txt",
            "./tmp/1.txt",
            "./tmp/2.txt",
        ];

        assert_eq!(dirs_and_files, expected);
    }
}
