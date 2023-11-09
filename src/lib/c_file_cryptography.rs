pub mod gpgfile {
    use std::io::{self, BufWriter, Write};
    use std::path::{Path, PathBuf};
    use std::process::Command;

    pub fn file_encryption(filename: &str, recipient: &str, temporary_file: &str) {
        let filename = format!("{}.gpg", filename);
        
        let mut command_gpg = Command::new("gpg");
        command_gpg.arg("-o").arg(&filename);
        command_gpg.arg("-e");
        command_gpg.arg("-r").arg(recipient);
        command_gpg.arg(format!("/dev/shm/{}", temporary_file));

        let status = command_gpg.status();
        match status {
            Ok(code) => {
                if !code.success() {
                    let stdout = io::stdout();
                    let mut handle = BufWriter::new(stdout.lock());
                    writeln!(handle, "\nerror: WARNING! something went wrong with the encryption").unwrap();
                    writeln!(handle, "       The unencrypted file may still be found in memory").unwrap();
                    writeln!(handle, "       as /dev/shm/{}\n", temporary_file).unwrap();
                    handle.flush().unwrap();
                    panic!();
                } else {
                    println!("success: encryption was successful");
                }
            },
            Err(e) => {
                let stdout = io::stdout();
                let mut handle = BufWriter::new(stdout.lock());
                writeln!(handle, "\nerror: WARNING! something went wrong with the encryption").unwrap();
                writeln!(handle, "       The unencrypted file may still be found in memory").unwrap();
                writeln!(handle, "       as /dev/shm/{}\n", temporary_file).unwrap();
                handle.flush().unwrap();
                panic!("\nerror: program terminated with code: {}\n", e);
            },
        }
    }

    pub fn file_decryption(filename: &str, temporary_file: &str) -> String {
        let temporary_file = Path::new("/dev/shm/").join(temporary_file);
        let filename = Path::new("").join(filename);
        let mut attempts = 0;


        loop {
            println!("passphrase attempt: {}/5", attempts + 1);
            let command_gpg = Command::new("gpg")
                .arg("--pinentry-mode").arg("loopback")
                .arg("-o").arg(&temporary_file)
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
        // let temporary_file: String = temporary_file.to_string_lossy().to_string();

        new_filename
    }
}
