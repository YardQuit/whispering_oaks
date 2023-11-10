use lib_misc::{env, gen};
use lib_fs::{make, verify, wreck};
use lib_procs::init;
use lib_cipher::{encrypt, decrypt};
use lib_args::matches;

fn main() {
    let editor = env::editor_selection();

    let file_name = gen::name();
    let dir_path = "/dev/shm/";

    let status = verify::binary_verification("gpg");
    if !status {
        panic!();
    }    

    let status = verify::binary_verification(&editor); 
    if !status {
        panic!();
    }

    let (filename, recipient, decrypt) = matches::cli_args();
    if decrypt {

        let filename = decrypt::file_decryption(&filename, &file_name);

        init::editor_initiation(&editor, &file_name);

        let status = verify::file_verification(dir_path, &file_name);
        if !status {
            panic!();
        }

        encrypt::file_encryption(&filename, &recipient, &file_name);

        let status = wreck::file(dir_path, &file_name);
        if !status {
            panic!();
        }
        
    } else {

        let status = make::file(dir_path, &file_name); 
        if !status {
            panic!();
        }

        init::editor_initiation(&editor, &file_name);

        let status = verify::file_verification(dir_path, &file_name);
        if !status {
            panic!();
        }

        encrypt::file_encryption(&filename, &recipient, &file_name);

        let status = wreck::file(dir_path, &file_name);
        if !status {
            panic!();
        }
    }
}
