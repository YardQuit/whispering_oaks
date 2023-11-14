use std::path::Path;
use std::fs;

/*
    function takes a path and destroys a file on the file system
*/
pub fn file(dir_path: &str, file_name: &str) -> bool {
    let file_path = Path::new(dir_path).join(file_name);    

    let status = fs::remove_file(file_path);
    match status {
        Ok(_) => true,
        Err(e) => {
            eprintln!("\nerror: failed to destroy file {}, with code: {}", file_name, e);
            false
        },
    }
    
}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::make;

    #[test]
    fn test_fs_wreck_file_success() {
        let dir_path = "/dev/shm/";
        let file_name = "unit_test_wreck_file.txt";
        make::file(dir_path, file_name);

        assert!(file(dir_path, file_name));
        let file_path = Path::new(dir_path).join(file_name);
        let _ = std::fs::remove_file(file_path);
    }
    
    #[test]
    fn test_fs_wreck_file_failure() {
        let dir_path = "/dev/shm/";
        let file_name = "unit_test_wreck_file.txt";
        make::file(dir_path, file_name);

        assert!(!file(dir_path, "something_else"));
        let file_path = Path::new(dir_path).join(file_name);
        let _ = std::fs::remove_file(file_path);
    }
}
