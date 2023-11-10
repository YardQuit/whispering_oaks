use std::process::Command;

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
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_binary_present() {
        let status = binary_verification("rm");
        assert!(status);
    }

    #[test]
    fn test_fs_binary_missing() {
        let status = binary_verification("binary_missing");
        assert!(!status);
    }
}
