use std::path::Path;

/*
    function takes a path and verifies if the file is present on the file system
*/
pub fn file_verification(dir_path: &str, file_name: &str) -> bool {
    let file_path = Path::new(dir_path).join(file_name);
    file_path.is_file()
}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_fs_file_present() {
        let dir_path = "/dev/shm/";
        let file_name = "unit_test_verify_file.txt";
        let file_path = format!("{}{}", dir_path, file_name);
        let _ = File::create(&file_path);
        assert!(file_verification(dir_path, file_name));
        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_fs_file_missing() {
        let file_name = "unit_test_verify_file.txt";
        let dir_path = "/dev/shm/";
        assert!(!file_verification(dir_path, file_name));
    }
}
