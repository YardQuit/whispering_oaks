// use std::path::{Path, PathBuf};
// use std::process::Command;
// use std::fs;
// use lib_fs::verify;
// use lib_misc::err::{self, WoErr};
// use lib_procs::build::CommandBuilder;

// pub fn file_decryption(output_file: &str, input_file: &str) -> String {
//     let mut decrypt_attempts = 0;

//     let mut gnupg_command = CommandBuilder::new("gpg")
//         .gnupg_set_pinentry()
//         .gnupg_set_output_file(output_file)
//         .gnupg_set_decrypt_file()
//         .generic_set_input_file(input_file)
//         .build();

//     loop {
//         println!("\ndebug: gnupg_command executed {:?}", gnupg_command);
//         let status = gnupg_command.output();
//         match status {
//             Ok(status) => {
//                 if status.status.success() {
//                     break;
//                 } else {
//                     decrypt_attempts += 1;
//                     if decrypt_attempts >= 5 {
//                         let error = WoErr::new(
//                             "", 
//                             "failed to decrypt due to wrong password", 
//                             "", 
//                             1
//                         );
//                         err::error_handling(&error);
//                         // err::error_handling("execute error", 
//                         //     "failed to decrypt due to wrong password", 
//                         //     "", 
//                         //     1);
//                     }
//                 }
//             },
//             Err(e) => {
//                 let error = WoErr::new(
//                     e,
//                     "failed to execute command",
//                     "",
//                     1
//                 );
//                 err::error_handling(&error);
//                 // err::error_handling(e, 
//                 //     "failed to execute command", 
//                 //     "", 
//                 //     1);
//             },
//         }
//     }

//     let old_output_file = PathBuf::from(&input_file);
//     let new_output_file = old_output_file
//         .with_extension("")
//         .to_string_lossy()
//         .to_string();

//     println!("\ndebug: new_output_file: {}", new_output_file);

//     new_output_file
// }

// pub fn template(file_name: &str, template_path: &str, template: &str) -> bool  {
//     let file_name = Path::new("/dev/shm/").join(file_name);
//     let template = format!("{}{}", template_path, template);

//     let status = verify::f_gpg("", &template);
//     match status {
//         Ok(value) {
//             if value == "encrypted" {
//             // if template.ends_with(".gpg") {
//                 let mut attempts = 0;
//                 loop {
//                     let command_gpg = Command::new("gpg")
//                         .arg("--pinentry-mode").arg("loopback")
//                         .arg("-o").arg(&file_name)
//                         .arg("-d").arg(&template)
//                         .output()
//                         .expect("\nerror: failed to execute command\n");

//                     if command_gpg.status.success() {
//                         return true;
//                     } else {
//                         attempts += 1;
//                         if attempts >= 5 {
//                             eprintln!("\nerror: faild to provide the correct passphrase");
//                             std::process::exit(1);
//                         }
//                     }
//                 }
//             } else {
//                 let status = fs::copy(&template, &file_name);
//                 match status {
//                     Ok(_) => true,
//                     Err(e) => {
//                         eprintln!("\nerror: template failed with code: {}\n", e);
//                     },
//                 }
//             }
//         Err(e) => {
//             //
//         },
// },
// }
/*
    UNIT-TESTS BEGINS NOW
*/
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_template_gpg() {}
    
//     #[test]
//     fn test_template_txt() {}

//     #[test]
//     fn test_template_md() {}

//     #[test]
//     #[should_panic]
//     fn test_template_missing_ext_panic() {}

// }
