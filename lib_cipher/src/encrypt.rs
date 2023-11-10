use std::io::{self, BufWriter, Write};
use std::process::Command;

pub fn file_encryption(filename: &str, recipient: &str, file_name: &str) {
    let filename = format!("{}.gpg", filename);
    
    let mut command_gpg = Command::new("gpg");
    command_gpg.arg("-o").arg(&filename);
    command_gpg.arg("-e");
    command_gpg.arg("-r").arg(recipient);
    command_gpg.arg(format!("/dev/shm/{}", file_name));

    let status = command_gpg.status();
    match status {
        Ok(code) => {
            if !code.success() {
                let stdout = io::stdout();
                let mut handle = BufWriter::new(stdout.lock());
                writeln!(handle, "\nerror: WARNING! something went wrong with the encryption").unwrap();
                writeln!(handle, "       The unencrypted file may still be found in memory").unwrap();
                writeln!(handle, "       as /dev/shm/{}\n", file_name).unwrap();
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
            writeln!(handle, "       as /dev/shm/{}\n", file_name).unwrap();
            handle.flush().unwrap();
            panic!("\nerror: program terminated with code: {}\n", e);
        },
    }
}
