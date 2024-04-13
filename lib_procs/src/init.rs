use std::io;
use crate::build::CommandBuilder;

pub fn editor_initiation(editor: &str, file_name: &str) -> Result<(), io::Error> {
    let mut editor_command = CommandBuilder::new(editor)
        .editor_set_current_dir("/dev/shm")
        .generic_set_input_file(file_name)
        .build();

    let status = editor_command.status();
    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
