use std::process::Command;
use std::path::Path;

/*
    function takes a path and verifies if the file is present on the file system
*/
pub fn binary_verification(binary_name: &str) -> bool {
    let mut command = Command::new(binary_name);
    command.arg("--version");

    let status = command.output();
    match status {
        Ok(_) => true,
        Err(e) => {
            eprintln!("\nerror: failed to verify binarywith code: {}", e);
            false
        },
    }
}

/*
    function takes a path and verifies if the file is present on the file system
*/
pub fn file_verification(dir_path: &str, file_name: &str) -> bool {
    let file_path = Path::new(dir_path).join(file_name);
    file_path.is_file()
}

/*
    function takes a path and verifies if the path is present on the file system
*/
pub fn dir_verification(dir_path: &str) -> bool {
    let dir_path = Path::new(dir_path);
    dir_path.is_dir()
}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_binary_fs_binary_present() {
        let status = binary_verification("rm");
        assert!(status);
    }

    #[test]
    fn test_binary_fs_binary_missing() {
        let status = binary_verification("binary_missing");
        assert!(!status);
    }
    #[test]
    fn test_file_fs_file_present() {
        let dir_path = "/dev/shm/";
        let file_name = "unit_test_verify_file.txt";
        let file_path = format!("{}{}", dir_path, file_name);
        let _ = File::create(&file_path);
        assert!(file_verification(dir_path, file_name));
        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_file_fs_file_missing() {
        let file_name = "unit_test_verify_file.txt";
        let dir_path = "/dev/shm/";
        assert!(!file_verification(dir_path, file_name));
    }

    #[test]
    fn test_dir_fs_path_present() {
        let _ = std::fs::create_dir("/dev/shm/unit_test_verify_dir");
        let dir_path = "/dev/shm/unit_test_verify_dir";
        
        assert!(dir_verification(dir_path));
        let _ = std::fs::remove_dir(dir_path);
    }

    #[test]
    fn test_dir_fs_path_missing() {
        let path_dir = "/dev/shm/nonexistent_directory";
        assert!(!dir_verification(path_dir)); 
    }
}
