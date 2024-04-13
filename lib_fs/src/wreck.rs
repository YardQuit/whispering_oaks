use std::path::Path;
use std::fs;
use std::io;

/*
    function takes a path and destroys a file on the file system
*/
pub fn file(dir_path: &str, file_name: &str) -> Result<(), io::Error> {
    let file_path = Path::new(dir_path).join(file_name);    

    let status = fs::remove_file(file_path);
    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/*
    function that destroys files with prefix "wo_" in shared memory
*/
pub fn clear(dir_path: &str) -> Result<(), io::Error> {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_str = entry.file_name();
                    let entry_str_lossy = entry_str.to_string_lossy().to_string();

                    if entry_str_lossy.starts_with("wo_") {
                        let file = entry.path();
                        match fs::remove_file(&file) {
                            Ok(_) => {},
                            Err(e) => eprintln!("\nerror: could not delete entry {}, with code: {}", file.display(), e),
                        }
                    }
                } else {
                    eprintln!("\nerror: could not read shared memory entry");
                }
            }
            Ok(())
        }
        Err(e) =>  Err(e),
    }

}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::make;

    // #[test]
    // fn test_fs_wreck_file_success() {
    //     let dir_path = "/dev/shm/";
    //     let file_name = "unit_test_wreck_file.txt";
    //     make::file(dir_path, file_name);

    //     assert!(file(dir_path, file_name));
    //     let file_path = Path::new(dir_path).join(file_name);
    //     let _ = std::fs::remove_file(file_path);
    // }
    
    // #[test]
    // fn test_fs_wreck_file_failure() {
    //     let dir_path = "/dev/shm/";
    //     let file_name = "unit_test_wreck_file.txt";
    //     make::file(dir_path, file_name);

    //     assert!(!file(dir_path, "something_else"));
    //     let file_path = Path::new(dir_path).join(file_name);
    //     let _ = std::fs::remove_file(file_path);
    // }
}
