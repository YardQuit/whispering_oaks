use std::process::{Command, ExitStatus};

pub fn editor_initiation(editor: &str, file_name: &str) {
    let mut command_editor = Command::new(editor);
    command_editor.current_dir("/dev/shm/");
    command_editor.arg(file_name);

    let status: std::io::Result<ExitStatus> = command_editor.status();
    match status {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\nerror: failed with editor initiation with code: {}\n", e);
            std::process::exit(1);
        }
    }
}
