use std::process::Command;
use std::path::Path;
use std::os::linux::fs::MetadataExt;
use std::fs::{self};
use std::io::{self};
use std::time::SystemTime;
use lib_procs::build::CommandBuilder;

/*
function takes a binary name and checks if the binary is present
*/
pub fn binary(binaries: Vec<&str>) -> Result<(), io::Error> {
    for binary_name in binaries {
        let mut command = Command::new(binary_name);
        command.arg("--version");

        let status = command.output();
        match status {
            Ok(_) => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/*
function takes a path and verifies if the file is present on the file system
*/
pub fn file(dir_path: &str, file_name: &str) -> bool {
    let file_path = Path::new(dir_path).join(file_name);
    file_path.is_file()
}

/*
function takes a path and verifies if the path is present on the file system
*/
pub fn dir(dir_path: &str) -> bool {
    let dir_path = Path::new(dir_path);
    dir_path.is_dir()
}

/*
function takes a path to file and returns last modified time
*/
pub fn f_meta(dir_path: &str, file_name: &str) -> Result<SystemTime, io::Error> {
    let file_path = Path::new(dir_path).join(file_name);

    if !file_path.exists() {
        return Err(
            io::Error::new(
            io::ErrorKind::NotFound,"file not found"
            )
        );
    }

    let metadata = fs::metadata(file_path);
    match metadata {
        Ok(metadata) => Ok(metadata.modified().unwrap()),
        Err(e) => Err(io::Error::other(e)),
    }
}

/*
function takes a path and file and returns bool if file is larger than 0 or not
*/
pub fn f_size(dir_path: &str, file_name: &str) -> Result<bool, io::Error> {
    let file_path = format!("{}{}", dir_path, file_name);
    let metadata = fs::metadata(file_path);
    match metadata {
        Ok(metadata) => {
            Ok(metadata.st_size() > 0)                     // returns true or false
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::Other,e)),
    }
}

/*
function that urilizes the gnupg "--list-packets" flag to determine if a file
is gnupg encrypted or not.
*/
pub fn f_gpg(dir_path: &str, file_name: &str) -> Result<String, io::Error> {
    let file_path = format!("{}{}", dir_path, file_name);
    let mut gnupg_command = CommandBuilder::new("gpg")
        .gnupg_set_listpackets()
        .generic_set_input_file(&file_path)
        .build();
    
    let status = gnupg_command.output();
    match status {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            match output_str.contains("encrypted") {
                true => Ok(String::from("encrypted")),
                false => Ok(String::from("cleartext")),
            }
        },
        Err(e) => Err(e),
    }
}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    // #[test]
    // fn test_binary_fs_binary_present() {
    //     let status = binary(vec!["rm", "gpg"]);
    //     assert!(status);
    // }

    // #[test]
    // fn test_binary_fs_binary_missing() {
    //     let status = binary(vec!["binary_missing"]);
    //     assert!(!status);
    // }
    #[test]
    fn test_file_fs_file_present() {
        let dir_path = "/dev/shm/";
        let file_name = "unit_test_verify_file.txt";
        let file_path = format!("{}{}", dir_path, file_name);
        let _ = File::create(&file_path);
        assert!(file(dir_path, file_name));
        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_file_fs_file_missing() {
        let file_name = "unit_test_verify_file.txt";
        let dir_path = "/dev/shm/";
        assert!(!file(dir_path, file_name));
    }

    #[test]
    fn test_dir_fs_path_present() {
        let _ = std::fs::create_dir("/dev/shm/unit_test_verify_dir");
        let dir_path = "/dev/shm/unit_test_verify_dir";
        
        assert!(dir(dir_path));
        let _ = std::fs::remove_dir(dir_path);
    }

    #[test]
    fn test_dir_fs_path_missing() {
        let path_dir = "/dev/shm/nonexistent_directory";
        assert!(!dir(path_dir)); 
    }
}
