pub mod sysutils {
    use std::process::Command;

    pub fn system_dependencies(editor: &str) {
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
}
    

