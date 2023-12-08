use std::process::Command;

pub struct CommandBuilder {
    command: Command,
}

impl CommandBuilder {
    pub fn new(program: &str) -> Self {
        Self {
            command: Command::new(program),
        }
    }

    pub fn build(self) -> Command {
        self.command
    }

/*
    GENERIC ARGUMENTS AND FLAGS
*/
    pub fn generic_set_input_file(mut self, filename: &str) -> Self {
        self.command.arg(filename);
        self
    }

/*
    GNUPG SPECIFIC ARGUMENTS AND FLAGS
*/
    pub fn gnupg_set_recipient(mut self, recipient: &str) -> Self {
        if recipient != "_omit_recipient_" {
            self.command.arg("-r").arg(recipient);
        }
        self
    }

    pub fn gnupg_set_output_file(mut self, filename: &str) -> Self {
        self.command.arg("-o").arg(filename);
        self
    }

    pub fn gnupg_set_encrypt(mut self) -> Self {
        self.command.arg("-e");
        self
    }

    pub fn gnupg_set_decrypt_file(mut self) -> Self {
        self.command.arg("-d");
        self
    }

    pub fn gnupg_set_pinentry(mut self) -> Self {
        self.command.arg("--pinentry-mode");
        self.command.arg("loopback");
        self
    }

    pub fn gnupg_set_no_key(mut self) -> Self {
        self.command.arg("-c");
        self
    }

    pub fn gnupg_set_listpackets(mut self) -> Self {
        self.command.arg("--list-packets");
        self
    }

    pub fn gnupg_set_listkeys(mut self) -> Self {
        self.command.arg("--list-key");
        self
    }

/*
    EDITOR ARGUMENTS AND FLAGS
*/
    pub fn editor_set_current_dir(mut self, dir: &str) -> Self {
        self.command.current_dir(dir);
        self
    } 
}

