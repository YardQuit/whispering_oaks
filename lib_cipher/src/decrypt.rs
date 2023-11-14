use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

pub fn file(filename: &str, file_name: &str) -> String {
    let file_name = Path::new("/dev/shm/").join(file_name);
    let filename = Path::new("").join(filename);
    let mut attempts = 0;
   
    loop {
        println!("passphrase attempt: {}/5", attempts + 1);
        let command_gpg = Command::new("gpg")
            .arg("--pinentry-mode").arg("loopback")
            .arg("-o").arg(&file_name)
            .arg("-d").arg(&filename)
            .output()
            .expect("\nerror: failed to execute command");

        if command_gpg.status.success() {
            println!("success: password accepted");
            break;
        } else {
            attempts += 1;
            if attempts >= 5 {
                eprintln!("\nerror: faild to provide the correct passphrase");
                std::process::exit(1);
            }
        }
    }
    let path_fs_filename = PathBuf::from(&filename);
    let new_filename = path_fs_filename.with_extension("").to_string_lossy().to_string();

    new_filename
}

pub fn template(file_name: &str, template_path: &str, template: &str) -> bool  {
    let file_name = Path::new("/dev/shm/").join(file_name);
    let template = format!("{}{}", template_path, template);

    if template.ends_with(".gpg") {
        let mut attempts = 0;
        loop {
            println!("passphrase attempt: {}/5", attempts + 1);
            let command_gpg = Command::new("gpg")
                .arg("--pinentry-mode").arg("loopback")
                .arg("-o").arg(&file_name)
                .arg("-d").arg(&template)
                .output()
                .expect("\nerror: failed to execute command\n");

            if command_gpg.status.success() {
                println!("success: password accepted");
                return true;
            } else {
                attempts += 1;
                if attempts >= 5 {
                    eprintln!("\nerror: faild to provide the correct passphrase");
                    std::process::exit(1);
                }
            }
        }
    } else {
        let status = fs::copy(&template, &file_name);
        match status {
            Ok(_) => true,
            Err(e) => {
                eprintln!("\nerror: template failed with code: {}\n", e);
                false
            },
        }
    }
}

/*
    UNIT-TESTS BEGINS NOW
*/
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_template_gpg() {}
    
//     #[test]
//     fn test_template_txt() {}

//     #[test]
//     fn test_template_md() {}

//     #[test]
//     #[should_panic]
//     fn test_template_missing_ext_panic() {}

// }
