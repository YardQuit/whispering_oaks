use std::path::Path;
use std::process::Command;
use std::fs;

pub fn file_decryption(file_name: &str, template_path: &str, template: &str) -> bool  {
    let file_name = Path::new("/dev/shm/").join(file_name);
    let template = Path::new(template_path).join(template);
    let template_ext = template
        .extension()
        .expect("\nerror: template must have a file extention\n")
        .to_string_lossy()
        .to_string();

    if template_ext == "gpg" {
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
                    panic!("\nerror: failed to provide the correct passphrase\n");
                }
            }
        }
    } else {
        println!("\ndebug: file does not end with .gpg");
        let status = fs::copy(&template, &file_name);
        match status {
            Ok(_) => true,
            Err(e) => {
                eprintln!("\nerror: faild to copy template with code: {}\n", e);
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
