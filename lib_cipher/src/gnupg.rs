use lib_procs::build::CommandBuilder;
use std::io;
use std::path::PathBuf;

pub fn file_encrypt(output_file: &str, recipient: &str, input_file: &str) -> Result<(), io::Error> {
    let output_file = format!("{}.gpg", output_file);

    let mut gnupg_command = CommandBuilder::new("gpg")
        .gnupg_set_output_file(&output_file)
        .gnupg_set_encrypt()
        .gnupg_set_recipient(recipient)
        .generic_set_input_file(input_file)
        .build();

    let status = gnupg_command.status();
    match status {
        Ok(value) => {
            if value.success() {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "GnuPG encryption failed",
                ))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn file_decrypt(output_file: &str, input_file: &str) -> Result<String, io::Error> {
    let mut decrypt_attempts = 0;

    let mut gnupg_command = CommandBuilder::new("gpg")
        .gnupg_set_pinentry()
        .gnupg_set_output_file(output_file)
        .gnupg_set_decrypt_file()
        .generic_set_input_file(input_file)
        .build();

    loop {
        let status = gnupg_command.output();
        match status {
            Ok(status) => {
                if status.status.success() {
                    break;
                } else {
                    decrypt_attempts += 1;
                    if decrypt_attempts >= 3 {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "GnuPG decryption failed",
                        ));
                    }
                }
            },
            Err(e) => return Err(e),
        }
    }

    let old_output_file = PathBuf::from(&input_file);
    let new_output_file = old_output_file
        .with_extension("")
        .to_string_lossy()
        .to_string();

    Ok(new_output_file)
}
