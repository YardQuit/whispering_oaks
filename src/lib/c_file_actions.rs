pub mod fileact {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::path::Path;
    use std::process::Command;
    use std::fs::File;

    pub fn file_namegen() -> String {
        let generate: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        generate
    }

    pub fn file_creation(temporary_file: &str) {
        // code here        
        let path = Path::new("/dev/shm/").join(temporary_file);        
        let status = File::create(path);
        match status {
            Ok(_) => {},
            Err(e) => {
                panic!("\nerror: unable to create a termprary file in memory with code: {}", e);
            }
        }
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
}
