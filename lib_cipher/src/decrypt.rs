use std::path::{Path, PathBuf};
use std::process::Command;

pub fn file_decryption(filename: &str, file_name: &str) -> String {
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
                panic!("\nerror: failed to provide the correct passphrase");
            }
        }
    }
    let path_fs_filename = PathBuf::from(&filename);
    let new_filename = path_fs_filename.with_extension("").to_string_lossy().to_string();

    new_filename
}
