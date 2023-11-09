pub mod editor {
    use std::process::{Command, ExitStatus};
    use std::env;

    pub fn editor_selection() -> String {
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

    pub fn editor_initiation(editor: &str, temporary_file: &str) {
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


}
