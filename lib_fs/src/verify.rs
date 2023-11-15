use std::process::Command;
use std::path::Path;
use std::os::linux::fs::MetadataExt;
use std::fs;
use std::fs::{File};
use std::io::{self, Read};

/*
    function takes a path and verifies if the file is present on the file system
*/
pub fn binary(binaries: Vec<&str>) -> bool {

    for binary_name in binaries {
        let mut command = Command::new(binary_name);
        command.arg("--version");

        let status = command.output();
        match status {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("\nerror: failed to verify binarywith code: {}", e);
                return false;
            },
        }
    }
    true
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

pub fn f_size(dir_path: &str, file_name: &str) -> bool {
    let file_path = format!("{}{}", dir_path, file_name);
    let metadata = fs::metadata(file_path);
    match metadata {
        Ok(metadata) => {
            metadata.st_size() > 0                     // returns true or false
        },
        Err(e) => {
            eprintln!("\nerror: could not read metadata with code: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn f_gpg(dir_path: &str, file_name: &str) -> bool {
    let file_path = format!("{}{}", dir_path, file_name);
    match read_file_header(&file_path) {
        Ok(header) => {
            header[0] == 133 && header[1] == 2 && header[2] == 12 && header[3] == 3
        },
        Err(e) => {
            eprintln!("\nerror: failed to read file header with code: {}", e);
            false
        },  
    }
}

fn read_file_header(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    const HEADER_SIZE: usize = 4;
    let mut header = vec![0u8; HEADER_SIZE];

    file.read_exact(&mut header)?;
    Ok(header)
}

// pub fn f_gpg(dir_path: &str, file_name: &str) -> bool {
//     let file_path = format!("{}{}", dir_path, file_name);
//     let output = Command::new("file")
//         .arg(file_path)
//         .output();

//     let file_type = String::from_utf8_lossy(&output.unwrap().stdout).trim().to_string();
//         file_type.contains("encrypt")
// }

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_binary_fs_binary_present() {
        let status = binary(vec!["rm", "gpg"]);
        assert!(status);
    }

    #[test]
    fn test_binary_fs_binary_missing() {
        let status = binary(vec!["binary_missing"]);
        assert!(!status);
    }
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
