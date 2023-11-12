use lib_args::matches;
use lib_cipher::{decrypt, encrypt, template};
use lib_fs::{make, verify, wreck};
use lib_misc::{env, gen};
use lib_procs::init;

fn main() {
    let editor = env::editor_selection();

    // temorary file name and path
    let file_name = gen::name();
    let dir_path = "/dev/shm/";

    // home dir path
    let home_dir = std::env::home_dir().unwrap();
    let home_dir = home_dir.to_string_lossy();

    // make config directory structure if not exists
    let config_path = format!("{}/.config/whispering_oaks/", home_dir);
    let status = make::dir(config_path.as_str());
    if !status {
        panic!();
    }

    // make template directory structure if not exists
    let template_path = format!("{}/.config/whispering_oaks/templates/", home_dir);
    let status = make::dir(template_path.as_str());
    if !status {
        panic!();
    }

    // verify binaries
    let status = verify::binary_verification("gpg");
    if !status {
        panic!();
    }

    let status = verify::binary_verification(&editor);
    if !status {
        panic!();
    }

    // get provided valiues given from command-line as arguments
    let (filename, recipient, template, decrypt) = matches::cli_args();

    // execute tracks depending on provided command-line argumets
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

        std::process::exit(0);
    }

    if !template.is_empty() {
        println!("\ndebug: template track");

        let status = template::file_decryption(&file_name, &template_path, &template);
        if !status {
            println!("\ndebug: false");
            panic!();
        } else {
            println!("\ndebug: true")
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

        std::process::exit(0);
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

        std::process::exit(0);
    }
}
