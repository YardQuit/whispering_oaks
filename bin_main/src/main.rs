use lib_args::matches;
use lib_cipher::{decrypt, encrypt};
use lib_fs::{make, verify, wreck};
use lib_misc::{env, gen};
use lib_procs::init;

fn main() {
    let editor = env::editor_selection();

    /*
        configures the temorary file name and path
    */
    let temporary_file_name = gen::name();
    let temporary_dir_path = "/dev/shm/";

    // configure the application config path
    let config_path = format!("{}/whispering_oaks", env::config_path());

    /*
        make template directory structure if not exists
        and configures the template path
    */
    let template_path = format!("{}/templates/", config_path);
    let status = make::dir(template_path.as_str());
    if !status {
        panic!();
    }

    /*
        verify binaries
    */
    let status = verify::binary(vec!["gpg", "&editor"]);
    if !status {
        panic!();
    }

    /*
        get provided valiues given from command-line as arguments
    */
    let (filename, recipient, template, decrypt) = matches::cli_args();

    /*
        execute tracks depending on provided command-line argumets
        1. decrypt existing file for editing
        2. decrypt or load template file to be used with for a new file
        3. cleate a new file without pre-loaded with template data.
    */
    if decrypt {
        let filename = decrypt::file(&filename, &temporary_file_name);

        init::editor_initiation(&editor, &temporary_file_name);

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        encrypt::file_encryption(&filename, &recipient, &temporary_file_name);

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        std::process::exit(0);
    }

    if !template.is_empty() {
        let status = decrypt::template(&temporary_file_name, &template_path, &template);
        if !status {
            panic!();
        }

        init::editor_initiation(&editor, &temporary_file_name);

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        encrypt::file_encryption(&filename, &recipient, &temporary_file_name);

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        std::process::exit(0);
    } else {
        let status = make::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        init::editor_initiation(&editor, &temporary_file_name);

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        encrypt::file_encryption(&filename, &recipient, &temporary_file_name);

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        if !status {
            panic!();
        }

        std::process::exit(0);
    }
}
