// use lib_misc::err;
// use lib_procs::build::GnuPgCommandBuilder;

// pub fn file_encryption(output_file: &str, recipient: &str, input_file: &str) {
//     let output_file = format!("{}.gpg", output_file);

//     let mut gnupg_command = GnuPgCommandBuilder::new()
//         .set_output_file(&output_file)
//         .set_encrypt()
//         .set_recipient(recipient)
//         .set_input_file(input_file)
//         .build();

//     println!("\ndebug: gnupg_command executed: {:?}", gnupg_command);

//     let status = gnupg_command.status();
//     match status {
//         Ok(code) => {
//             if !code.success() {
//                 err::error_handling(code,
//                     "The unencrypted file may still be recoverable from shared memory with name:",
//                     input_file,
//                     1);
//             }
//         },
//         Err(e) => err::error_handling(e,
//             "The unencrypted file may still be recoverable from shared memory with name:",
//             input_file,
//             1),
//     }
// }
