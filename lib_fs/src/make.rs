use std::path::Path;
use std::fs::{File, create_dir_all, read_dir};
use std::io::{self, BufWriter, Write};
use lib_procs::build::CommandBuilder;

/*
    function takes a path and creates a file on the file system
*/
pub fn file(dir_path: &str, file_name: &str) -> Result<(), io::Error> {
    let invalid = ['|', '&', '$', '%', '<', '>', '\\', '*', ':'];

    let status = dir(dir_path);
    match status {
        Ok(_) => {
            if file_name.is_empty() || file_name.contains(invalid) {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "failed to create file due to invalid characters"
                    )
                )
            }
            let file_path = Path::new(dir_path).join(file_name);    

            let status = File::create(file_path);
            match status {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

/*
    function takes a path and creates a file system directory structure
*/
pub fn dir(dir_path: &str) -> Result<(), io::Error> {
    let invalid = ['|', '&', '$', '%', '<', '>', '\\', '*', ':'];
    if dir_path.is_empty() || dir_path.contains(invalid) {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "failed to create directory path due to invalid characters"
            )
        );
    }
    let dir_path = Path::new(dir_path);

    let status = create_dir_all(dir_path);
    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/*
    function takes a path, forwards request to list_directory and returns
    a boolian if successful or not
*/
pub fn list(dir_path: &str) -> bool {
    println!("Avaliable templates:");
    println!("--------------------------------------------------");
    let status = list_directory(dir_path, dir_path);
    status.is_ok()
}

/*
    function takes a dir_path and list directory files
    returns Ok() or io_Err to caller
*/
fn list_directory(dir_path: &str, base_path: &str) -> Result<(), io::Error> {
    let entries = read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(base_path)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string_lossy().to_string());
            println!("{}", relative_path);
        } else if path.is_dir() {
            list_directory(&path.to_string_lossy(), base_path)?;
        }
    }
    Ok(())
}

/*
    function that utilizes the gnupg "--list-key" flag to display public keys
    available to use in conjuction with "-r" or "-R" recipient(s)
*/
pub fn keylist() -> Result<(), io::Error> {
    let mut command = CommandBuilder::new("gpg")
        .gnupg_set_listkeys()
        .build();

    let status = command.output();
    match status {
        Ok(value) => {
            let stdout = io::stdout();
            let mut handle = BufWriter::new(stdout.lock());
            writeln!(handle, "{:#?}", value).unwrap();
            handle.flush().unwrap();
            Ok(())
        },
        Err(e) => Err(e),
    }
}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_fs_make_file_success() {
    //     let file_path = "/dev/shm/";
    //     let file_name = "unit_test_make_file.txt";
    //     assert!(file(file_path, file_name));
    //     let full_path = Path::new(file_path).join(file_name);
    //     let _ = std::fs::remove_file(full_path);
    // }
    
    // #[test]
    // fn test_fs_make_file_failure() {
    //     let file_path = "/dev/shm/random_path_that_does_not_exist/";
    //     let file_name = "unit_test_make_file.txt";
    //     assert!(!file(file_path, file_name));
    // }

    // #[test]
    // #[should_panic]
    // fn test_fs_make_file_panic() {
    //     let file_path = "/dev/shm/random_path_that_does_not_exist/";
    //     let file_name = "uni|t_test_make_file.txt";
    //     assert!(!file(file_path, file_name));
    // }

    // #[test]
    // fn test_fs_make_dir_success() {
    //     let dir_path = "/dev/shm/unit_test_make_dir";
    //     assert!(dir(dir_path));
    //     let _  = std::fs::remove_dir_all(dir_path);
    // }

    // #[test]
    // #[should_panic]
    // fn test_fs_make_dir_panic() {
    //     let dir_path = "/dev/shm/unit_test_make_dir|&test_dir";
    //     assert!(!dir(dir_path));
    // }
}
