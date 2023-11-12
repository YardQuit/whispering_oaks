use std::path::Path;
use std::fs::{File, create_dir_all};

/*
    function takes a path and creates a file on the file system
*/
pub fn file(dir_path: &str, file_name: &str) -> bool {
    let invalid = ['|', '&', '$', '%', '<', '>', '\\', '*', ':'];

    if dir_path.is_empty() || dir_path.contains(invalid) {
        panic!("\nerror: directory name contains invald chars");
    }

    if file_name.is_empty() || file_name.contains(invalid) {
        panic!("\nerror: file name contains invald chars");
    }
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

pub fn dir(dir_path: &str) -> bool {
    let invalid = ['|', '&', '$', '%', '<', '>', '\\', '*', ':'];
    if dir_path.is_empty() || dir_path.contains(invalid) {
        panic!("\nerror: directory name contains invald chars");
    }
    let dir_path = Path::new(dir_path);

    let status = create_dir_all(dir_path);
    match status {
        Ok(_) => true,
        Err(e) => {
            println!("\nerror: failed to create directory structure, with code: {}", e);
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

    #[test]
    #[should_panic]
    fn test_fs_make_file_panic() {
        let file_path = "/dev/shm/random_path_that_does_not_exist/";
        let file_name = "uni|t_test_make_file.txt";
        assert!(!file(file_path, file_name));
    }

    #[test]
    fn test_fs_make_dir_success() {
        let dir_path = "/dev/shm/unit_test_make_dir";
        assert!(dir(dir_path));
        let _  = std::fs::remove_dir_all(dir_path);
    }

    #[test]
    #[should_panic]
    fn test_fs_make_dir_panic() {
        let dir_path = "/dev/shm/unit_test_make_dir|&test_dir";
        assert!(!dir(dir_path));
    }
}
