pub mod tempfile {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::io::{self, BufWriter, Write};
    use std::process::Command;

    pub fn create_filename() -> String {
        let generate: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        generate
    }

    pub fn file_verification(temporary_file: &str) {
        let path = format!("/dev/shm/{}", temporary_file);

        let verify = std::fs::OpenOptions::new().read(true).open(path);
        match verify {
            Ok(_) => {}
            Err(e) => panic!("\nerror: failed with file verification with code: {}\n", e),
        }
    }

    pub fn file_removal(temporary_file: &str) {
        let mut command_rm = Command::new("rm");
        command_rm.arg(format!("/dev/shm/{}", temporary_file));

        let status = command_rm.status();
        match status {
            Ok(code) => {
                if code.success() {
                    println!("success: memory was successfully cleaned");
                } else {
                    println!("\nerror: failed to clean memory with code: {}\n", code);
                }
            },
            Err(e) => panic!("\nerror: failed to remove /dev/shm/{} with code: {}\n", temporary_file, e),
        }
    }

    pub fn file_encryption(filename: &str, recipient: &str, temporary_file: &str) {
        let output_file = format!("{}.gpg", filename);

        let mut command_gpg = Command::new("gpg");
        command_gpg.arg("-o").arg(&output_file);
        command_gpg.arg("-e");
        command_gpg.arg("-r").arg(recipient);
        command_gpg.arg(format!("/dev/shm/{}", temporary_file));

        let status = command_gpg.status();
        match status {
            Ok(code) => {
                if !code.success() {
                    let stdout = io::stdout();
                    let mut handle = BufWriter::new(stdout.lock());
                    writeln!(handle, "\nerror: WARNING! something went wrong with the ecryption").unwrap();
                    writeln!(handle, "       The unencrypted file may still be found in memeory").unwrap();
                    writeln!(handle, "       as /dev/shm/{}\n", temporary_file).unwrap();
                    handle.flush().unwrap();
                    panic!();
                } else {
                    println!("success: encryption was successful");
                }
            }
            Err(e) => {
                let stdout = io::stdout();
                let mut handle = BufWriter::new(stdout.lock());
                writeln!(handle, "\nerror: WARNING! something went wrong with the ecryption").unwrap();
                writeln!(handle, "       The unencrypted file may still be found in memeory").unwrap();
                writeln!(handle, "       as /dev/shm/{}\n", temporary_file).unwrap();
                handle.flush().unwrap();
                panic!("\nerror: program terminated with code: {}\n", e);
            }
        }
    }
}
