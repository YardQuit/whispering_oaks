use std::path::Path;
use std::fs::File;

/*
    function takes a path and creates a file on the file system
*/
pub fn file(dir_path: &str, file_name: &str) -> bool {
    let file_path = Path::new(dir_path).join(file_name);    

    let status = File::create(file_path);
    match status {
        Ok(_) => true,
        Err(e) => {
            println!("\nerror: failed to create file {}, with code: {}", file_name, e);
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

    #[test]
    fn test_fs_make_file_success() {
        let file_path = "/dev/shm/";
        let file_name = "unit_test_make_file.txt";
        assert!(file(file_path, file_name));
        let full_path = Path::new(file_path).join(file_name);
        let _ = std::fs::remove_file(full_path);
    }
    
    #[test]
    fn test_fs_make_file_failure() {
        let file_path = "/dev/shm/random_path_that_does_not_exist/";
        let file_name = "unit_test_make_file.txt";
        assert!(!file(file_path, file_name));
    }
}
