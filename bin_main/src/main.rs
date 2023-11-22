// use std::time::SystemTime;
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
        std::process::exit(1);
    }

    /*
        verify binaries
    */
    let status = verify::binary(vec!["gpg", &editor, "file"]);
    if !status {
        std::process::exit(1);
    }

    /*
        get provided valiues given from command-line as arguments
    */
    let (filename, 
        recipient, 
        template, 
        decrypt, 
        template_list,
        clear) = matches::cli_args(&config_path, "config.toml");

    /*
        execute tracks depending on provided command-line argumets
        0. lists template files stored in .config/whispering_oaks/templates
        1. decrypt existing file for editing
        2. decrypt or load template file to be used with for a new file
        3. cleate a new file without pre-loaded with template data.
    */
    //
    // TEMPLATE_LIST
    //
    if template_list {
        let status = make::list(&template_path);
        if !status {
            eprintln!("\nerror: could not list directory");
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    //
    // CLEAR SHARED MEMORY
    //
    if clear {
        let status = wreck::clear(temporary_dir_path);
        if !status {
            eprintln!("\nerror: could not clear shared memory");
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    //
    // USE TEMPLATE
    //
    if decrypt {
        let status = verify::file("", &filename);
        if !status {
            eprintln!("\nerror: verify '-f <filename>'");
            std::process::exit(1);
        }

        let status = verify::f_gpg("", &filename);
        if !status {
            eprintln!("\nerror: file doesn't seem to be encrypted");
            std::process::exit(1);
        }

        let filename = decrypt::file(&filename, &temporary_file_name);

        let modtime_1 = verify::f_meta(temporary_dir_path, &temporary_file_name).unwrap();

        init::editor_initiation(&editor, &temporary_file_name);

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }

        let modtime_2 = verify::f_meta(temporary_dir_path, &temporary_file_name).unwrap();
        if modtime_2 > modtime_1 {
            encrypt::file_encryption(&filename, &recipient, &temporary_file_name);
        }

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    //
    // EDIT GPG ENC FILE
    //
    if !template.is_empty() {
        let status = decrypt::template(&temporary_file_name, &template_path, &template);
        if !status {
            std::process::exit(1);
        }

        let modtime_1 = verify::f_meta(temporary_dir_path, &temporary_file_name).unwrap();

        init::editor_initiation(&editor, &temporary_file_name);

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }

        let modtime_2 = verify::f_meta(temporary_dir_path, &temporary_file_name).unwrap();
        if modtime_2 > modtime_1 {
            encrypt::file_encryption(&filename, &recipient, &temporary_file_name);
        }

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }
        std::process::exit(0);
    } else {
    //
    // CREATE NEW ENC FILE WITHOUT TEMPLATE
    //
        let status = make::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }

        init::editor_initiation(&editor, &temporary_file_name);

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }

        let status = verify::f_size(temporary_dir_path, &temporary_file_name);
        if !status {
            let status = wreck::file(temporary_dir_path, &temporary_file_name);
            if !status {
                std::process::exit(1);
            }
            std::process::exit(0);
        }

        encrypt::file_encryption(&filename, &recipient, &temporary_file_name);

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        if !status {
            std::process::exit(1);
        }
        std::process::exit(0);
    }
}
