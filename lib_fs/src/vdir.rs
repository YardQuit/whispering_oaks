use std::path::Path;

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

    #[test]
    fn test_fs_path_present() {
        let _ = std::fs::create_dir("/dev/shm/unit_test_verify_dir");
        let dir_path = "/dev/shm/unit_test_verify_dir";
        
        assert!(dir_verification(dir_path));
        let _ = std::fs::remove_dir(dir_path);
    }

    #[test]
    fn test_fs_path_missing() {
        let path_dir = "/dev/shm/nonexistent_directory";
        assert!(!dir_verification(path_dir)); 
    }
    
}
