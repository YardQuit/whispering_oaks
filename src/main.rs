use clap::Parser;
use core::panic;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::io::{self, BufWriter, Write};
use std::process::{Command, ExitStatus};

#[derive(Parser, Debug)]
#[command(
    author = "Michael Jones", 
    version, 
    about = "Encrypt your documents using GPG while leveraging your preferred editor.", 
    long_about = None
)]

struct CliArgs {
    #[arg(short, 
        long, 
        help = "To encrypt for a specific user ID and determine
which public keys to use, you can obtain the
necessary information by running the command
'gpg --list-public-keys'.")]
    recipient: String,

    #[arg(short, 
        long, 
        help = "To assign a filename to a document, simply provide
the desired name without including the '.gpg'
extension. This extension will be added
automatically. If you want to save the file in a
particular directory, you can include the
desired path as part of the filename.")]
    filename: String,
}

fn main() {
    let args = CliArgs::parse();
    let temporary_file = create_filename();
    let editor = editor_selection();

    system_dependencies(&editor);
    editor_initiation(&editor, &temporary_file);
    file_verification(&temporary_file);
    file_encryption(&args, &temporary_file);
    file_removal(&temporary_file);
}

fn file_removal(temporary_file: &str) {
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

fn editor_selection() -> String {
    let mut editor = String::new();
    let mut env_editor = env::var("WHISPERING_OAKS");
    match env_editor {
        Ok(value) => editor = value,
        Err(_) => {
            env_editor = env::var("EDITOR");
            match env_editor {
                Ok(value) => editor = value,
                Err(e) => panic!("\nerror: could not bind editor with code: {}\n", e),
            }
        }
    }
    editor
}

fn editor_initiation(editor: &str, temporary_file: &str) {
    let mut command_editor = Command::new(editor);
    command_editor.current_dir("/dev/shm/");
    command_editor.arg(temporary_file);

    let status: std::io::Result<ExitStatus> = command_editor.status();
    match status {
        Ok(_) => {}
        Err(e) => {
            panic!("\nerror: failed with editor initiation with code: {}\n", e);
        }
    }
}

fn file_encryption(args: &CliArgs, temporary_file: &str) {
    let output_file = format!("{}.gpg", args.filename);

    let mut command_gpg = Command::new("gpg");
    command_gpg.arg("-o").arg(&output_file);
    command_gpg.arg("-e");
    command_gpg.arg("-r").arg(&args.recipient);
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

fn file_verification(temporary_file: &str) {
    let path = format!("/dev/shm/{}", temporary_file);

    let verify = std::fs::OpenOptions::new().read(true).open(path);
    match verify {
        Ok(_) => {}
        Err(e) => panic!("\nerror: failed with file verification with code: {}\n", e),
    }
}

fn system_dependencies(editor: &str) {
    let mut command_gpg = Command::new("gpg");
    command_gpg.arg("--version");

    let output = command_gpg.output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                panic!("\nerror: utility \"gpg\" is not installed\n");
            }
        }
        Err(_) => {
            panic!("\nerror: failed to execute the \"GPG\" command\n");
        }
    }

    let mut command_editor = Command::new(editor);
    command_editor.arg("--version");

    let output = command_editor.output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                panic!("\nerror: utility \"{}\" is not installed\n", editor);
            }
        }
        Err(_) => {
            panic!("\nerror: failed to execute the \"{}\" command\n", editor);
        }
    }

    let mut command_rm = Command::new("rm");
    command_rm.arg("--version");

    let output = command_editor.output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                panic!("\nerror: utility \"rm\" is not installed\n");
            }
        }
        Err(_) => {
            panic!("\nerror: failed to execute the \"rm\" command\n");
        }
    }
}

fn create_filename() -> String {
    let generate: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    generate
}
