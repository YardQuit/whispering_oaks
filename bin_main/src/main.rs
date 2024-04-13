use std::time::SystemTime;
use lib_args::matches;
use lib_cipher::gnupg;
use lib_fs::{make, verify, wreck};
use lib_misc::{env, err::{self, WoErr}, gen};
use lib_procs::init;

static P_ENV_KEY: &str = "WHISPERING_OAKS";
static D_ENV_KEY: &str = "EDITOR";

fn main() {
    /*
    BIND EDITOR TO "editor" VARIABLE
    
    calls the editor_selection function which returns the preferred editor
    stored in environment variable WHISPERING_OAKS. If such variable is not
    set or is being empty, it looks for environment variable EDITOR. If such
    variable is not set, it calls for program exit. If EDITOR value is not set
    it returns an empty String. If EDITOR value is empty, it calls for program
    exit, else editor variable is set to return Ok(value).
    */
    let mut editor = String::new();
    let status = env::editor_selection(P_ENV_KEY, D_ENV_KEY);
    match status {
        Ok(value) => {
            if !value.is_empty() {
                editor = value;
            } else {
                err::error_handling(&WoErr::new(
                    "",
                    "no editor is set as preferred nor default as environment variable",
                    "",
                    1,
                ));
            }
        }
        Err(e) => {
            err::error_handling(
                &WoErr::new(
                    e, 
                    "faild to define editor to use", 
                    "", 
                    1
                )
            );
        }
    }

    /*
    BIND TEMP FILE NAME TO "temprorary_file_name" VARIABLE

    calls the name() function which generates a temporary filename beginning
    with prefix "wo_" followed with 10 random alphanumerics. Function will
    return the genrated name if genrerating it was successful, else it returns
    an Err. If recives an Err, it calls for program exit. else the variable
    temporary_file_name is set to return Ok(value).
    */
    let mut temporary_file_name = String::new();
    let status = gen::name(); 
    match status {
        Ok(value) => temporary_file_name = value,
        Err(e) => {
            err::error_handling(
                &WoErr::new(
                    "", 
                    e, 
                    "", 
                    1
                )
            );
        },
    };

    // BIND SHARED MEMORY TO "temporary_dir_path" VARIABLE
    let temporary_dir_path = "/dev/shm/";

    /*
    BIND APP CONFIG FOLDER TO "config_path" VARIABLE

    calls the config_path function which returns the ".config" path which is 
    set to the "config_path" variable with an Ok(value). If failed to locate
    the ".config" directory, it calls for program exit with an Err.
    */
    let mut config_path = String::new();
    let status = env::config_path();
    match status {
        Ok(value) => config_path = format!("{}/whispering_oaks/", value),
        Err(e) => {
            err::error_handling(
                &WoErr::new(
                    e,
                    "",
                    &config_path,
                    1
                )
            );
        },
    }

    // LOOK LOOK LOOK
    // make template directory structure if not exists
    // and configures the template path
    let template_path = format!("{}/templates/", config_path);
    let status = make::dir(template_path.as_str());
    match status {
        Ok(_) => {},
        Err(e) => {
            err::error_handling(
                &WoErr::new(
                    e,
                    "",
                    &template_path,
                    1
                )
            );
        },
    }

    /*
    BINDS TO "nothing" ONLY EXECUTES FUNCTION

    calls function binary which verifies that the vector of binary names
    are present in the system of which the program depends on in order to
    function. the function returns () for Ok, or Err in a failure.
    */
    if let Err(e) = verify::binary(vec!["gpg", &editor, "file"]) {
        err::error_handling(
            &WoErr::new(
                e, 
                "failed to verify binary", 
                "", 
                1)
        );
    }

    /*
    BIND OUTPUT PATH AND FILENAME TO "filename" VARIABLE
    BIND PUBLIC KEY TO "recipient" VARIABLE
    BIND TEMPLATE NAME TO "template" VARIABLE
    BIND BOOL TO "decrypt" VARIABLE
    BIND BOOL TO "template_list" VARIABLE
    BIND BOOL TO "clear" VARIABLE

    calls function cli_args which utlilzes the clap cargo for processing 
    command-line arguments and returns a tuple with arguments and flags.
    The function takes the "config_path" together with "config.toml" which is
    used to read the default recipient if such file or setting exists. The 
    default value is returned and set to variable "recipient" if such argument
    hasn't explicitly being set at runtime.
    */
    let mut set_arg = matches::cli_args(&config_path, "config.toml");

    // LIST_KEYS
    if set_arg.keylist {
        let status = make::keylist();
        match status {
            Ok(_) => {std::process::exit(0)},
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        "",
                        1
                    )
                );
            },
        }
    }

    // TEMPLATE_LIST
    if set_arg.template_list {
        let status = make::list(&template_path);
        if !status {
            err::error_handling(
                &WoErr::new(
                    "",
                    "could not list directory",
                    &template_path,
                    1
                )
            );
        }
        std::process::exit(0);
    }

    // CLEAR SHARED MEMORY
    if set_arg.clear {
        let status = wreck::clear(temporary_dir_path);
        match status {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "could not clear shared memory",
                        "",
                        1
                    )
                );
            },
        }
    }

    /*
    SECTION "decrypt" BEGINS HERE
    section executed if "decrypt" flag is set.
    ---------------------------------------------------------------------------
    */
    if set_arg.decrypt {
        /*
        BINDS TO "noting" ONLY EXECUTES FUNCTION

        calls function "file" which tests the file path and the existens of the
        passed file. The function returns a boolian if verification is success-
        ful or not.
        */
        let status = verify::file("", &set_arg.filename);
        if !status {
            err::error_handling(
                &WoErr::new(
                    "",
                    "faild to locate file",
                    &set_arg.filename,
                    1
                )
            );
        }

        /*
        BIND TO "nothing" ONLY EXECUTES FUNCTION

        calls function "f_gpg" which verifies if the file is GnuPG encrypted or
        is a clear text file. function returns a Result with () for Ok, or Err
        */
        let status = verify::f_gpg("", &set_arg.filename);
        match status {
            Ok(value) => {
                if value == "cleartext" {
                    err::error_handling(
                        &WoErr::new(
                            "",
                            "failed to decrypt file. file is not encrypted with GnuPG",
                            &set_arg.filename,
                            1
                        )
                    );
                }
            },
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        &set_arg.filename,
                        1
                    )
                );
            },
        }

        let status = gnupg::file_decrypt(
            format!("/dev/shm/{}", &temporary_file_name).as_str(),
            &set_arg.filename,
        );
        match status {
            Ok(value) => set_arg.filename = value,
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        "",
                        1
                    )
                );
            },
        }

        let mut modtime_1: SystemTime = SystemTime::now();
        let status = verify::f_meta(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(value) => modtime_1 = value,
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        &temporary_file_name,
                        1
                    )
                );
            },
        }

        if let Err(e) = init::editor_initiation(&editor, &temporary_file_name) {
            err::error_handling(
                &WoErr::new(
                    e, 
                    "", 
                    "", 
                    1
                )
            );
        }

        let modtime_2 = verify::f_meta(temporary_dir_path, &temporary_file_name).unwrap();
        if modtime_2 > modtime_1 {
            let status = gnupg::file_encrypt(
                &set_arg.filename,
                &set_arg.recipient,
                format!("{}{}", temporary_dir_path, &temporary_file_name).as_str(),
            );
            match status {
                Ok(_) => {},
                Err(e) => {
                    err::error_handling(
                        &WoErr::new(
                            e,
                            "file may be recoverable from shared memory",
                            &temporary_file_name,
                            1
                        )
                    );
                },
            }
        }

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "could not clear shared memory",
                        "",
                        1
                    )
                );
            },
        }
    }
    /*
    SECTION "decrypt" ENDS HERE
    ---------------------------------------------------------------------------
    */

    /*
    SECTION "encrypt with template" BEGINS HERE
    section executed if "encrypt" with template is set.
    ---------------------------------------------------------------------------
    */
    if !set_arg.template.is_empty() {
        use std::fs;
        let status = verify::f_gpg(&template_path, &set_arg.template);
        match status {
            Ok(value) => {
                /*
                if the f_gpg returns value "cleartext", the template will be
                copied to shared memory with a random "wo_" name.
                */
                if value == "cleartext" {
                    let status = fs::copy(
                        format!("{}{}", template_path, set_arg.template), 
                        format!("{}{}", temporary_dir_path, temporary_file_name)
                    );
                    match status {
                        Ok(_) => {},
                        Err(e) => {
                            err::error_handling(
                                &WoErr::new(
                                    e, 
                                    "", 
                                    "", 1
                                )
                            );
                        },
                    }
                } 

                /*
                if the f_gpg returns value "encrytped", the template will be  
                decrypted to shared memory with a random "wo_" name.
                */
                if value == "encrypted" {
                    let status = gnupg::file_decrypt(
                        format!("{}{}", temporary_dir_path, temporary_file_name).as_str(), 
                        format!("{}{}", template_path, set_arg.template).as_str());
                    match status {
                        Ok(_) => {},
                        Err(e) => {
                            err::error_handling(
                                &WoErr::new(
                                    e,
                                    "",
                                    "",
                                    1
                                )
                            );
                        },
                    }
                }
            },
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e, 
                        "", 
                        "", 1
                    )
                );
            }
        } 

        let mut modtime_1: SystemTime = SystemTime::now();
        let status = verify::f_meta(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(value) => modtime_1 = value,
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        &temporary_file_name,
                        1
                    )
                );
            },
        }

        if let Err(e) = init::editor_initiation(&editor, &temporary_file_name) {
            err::error_handling(
                &WoErr::new(
                    e, 
                    "", 
                    "", 
                    1
                )
            );
        }

        let modtime_2 = verify::f_meta(temporary_dir_path, &temporary_file_name).unwrap();
        if modtime_2 > modtime_1 {
            let status = gnupg::file_encrypt(
                &set_arg.filename,
                &set_arg.recipient,
                format!("{}{}", temporary_dir_path, &temporary_file_name).as_str(),
            );
            match status {
                Ok(_) => {},
                Err(e) => {
                    err::error_handling(
                        &WoErr::new(
                            e,
                            "file may be recoverable from shared memory",
                            &temporary_file_name,
                            1
                        )
                    );
                },
            }
        }
        
        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "could not clear shared memory",
                        "",
                        1
                    )
                );
            },
        }
    }
    /*
    SECTION "encrypt with template" ENDS HERE
    ---------------------------------------------------------------------------
    */

    /*
    SECTION "encrypt" BEGINS HERE
    section executed if "encrypt" without template is set.
    ---------------------------------------------------------------------------
    */
    if set_arg.template.is_empty() {
        let status = make::file(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(_) => {},
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        &temporary_file_name,
                        1
                    )
                );
            },
        }

        if let Err(e) = init::editor_initiation(&editor, &temporary_file_name) {
            err::error_handling(
                &WoErr::new(
                    e, 
                    "", 
                    "", 
                    1
                )
            );
        }

        let status = verify::file(temporary_dir_path, &temporary_file_name);
        if !status {
            err::error_handling(
                &WoErr::new(
                    "",
                    "is not a present file",
                    &temporary_file_name,
                    1
                )
            );
        }

        let status = verify::f_size(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(value) => {
                if !value {
                    return;
                } else {
                    let status = gnupg::file_encrypt(
                        &set_arg.filename,
                        &set_arg.recipient,
                        format!("{}{}", temporary_dir_path, &temporary_file_name).as_str(),
                    );
                    match status {
                        Ok(_) => {},
                        Err(e) => {
                            err::error_handling(
                                &WoErr::new(
                                    e,
                                    "file may be recoverable from shared memory",
                                    &temporary_file_name,
                                    1
                                )
                            );
                        },
                    }
                }
            },
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "",
                        &temporary_file_name,
                        1
                    )
                );
            },
        }

        let status = wreck::file(temporary_dir_path, &temporary_file_name);
        match status {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                err::error_handling(
                    &WoErr::new(
                        e,
                        "could not clear shared memory",
                        "",
                        1
                    )
                );
            },
        }
    }
    /*
    SECTION "encrypt" ENDS HERE
    ---------------------------------------------------------------------------
    */
}
