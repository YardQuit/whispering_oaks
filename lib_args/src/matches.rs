// extern crate clap;
use clap::{Arg, ArgAction, Command};
use lib_fs::read;

const AUTHOR: &str = "Author: Michael A Jones (github: YardQuit)";
const ABOUT: &str = "Encrypt your documents using GnuPG while leveraging your preferred editor.";
const FHELP: &str = include_str!("t_help_filename.txt");
const RHELP: &str = include_str!("t_help_recipient.txt");
const DHELP: &str = include_str!("t_help_decrypt.txt");
const THELP: &str = include_str!("t_help_template.txt");
const LHELP: &str = include_str!("t_help_template_list.txt");
const CHELP: &str = include_str!("t_help_clear.txt");
const OHELP: &str = include_str!("t_help_omit_recipient.txt");

/*
    function that matches and handles command-line argumets
*/
pub fn cli_args(dir_path: &str, file_name: &str) -> (String, String, String, bool, bool, bool) {
    let mut filename = String::new();
    let mut recipient = String::new();
    let mut template = String::new();

    let dynamic_def_recipient = read::read_toml(dir_path, file_name);
    let static_def_recipient: &'static str = Box::leak(dynamic_def_recipient.into_boxed_str());

    let matches = Command::new("whispering oaks")
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::new("filename")
                .help(FHELP)
                .required(true)
                .num_args(1)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("recipient")
                .help(RHELP)
                .required(static_def_recipient.is_empty())
                .short('r')
                .long("recipient")
                .aliases(["receiver", "rec"])
                .num_args(1)
                .action(ArgAction::Set)
                .default_value(static_def_recipient),
        )
        .arg(
            Arg::new("template")
                .help(THELP)
                .required(false)
                .short('t')
                .long("template")
                .num_args(1)
                .action(ArgAction::Set)
                .aliases(["tmp", "temp", "templ"])
                .conflicts_with("decrypt"),
        )
        .arg(
            Arg::new("template_list")
                .help(LHELP)
                .required(false)
                .short('T')
                .long("template_list")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["filename", "recipient", "template", "decrypt", "clear"]),
        )
        .arg(
            Arg::new("omit_recipient")
                .help(OHELP)
                .required(false)
                .short('R')
                .num_args(0)
                .long("omit_recipient")
                .action(ArgAction::SetTrue)
                .conflicts_with("recipient"),
        )
        .arg(
            Arg::new("decrypt")
                .help(DHELP)
                .required(false)
                .short('d')
                .num_args(0)
                .long("decrypt")
                .aliases(["dec", "decr"])
                .action(ArgAction::SetTrue)
                .conflicts_with("template"),
        )
        .arg(
            Arg::new("clear")
                .help(CHELP)
                .required(false)
                .short('c')
                .long("clear")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .conflicts_with_all([
                    "filename",
                    "recipient",
                    "template",
                    "decrypt",
                    "template_list",
                ]),
        )
        .get_matches();

    if let Some(f) = matches.get_one::<String>("filename") {
        filename = f.to_owned();
    };

    if let Some(r) = matches.get_one::<String>("recipient") {
        recipient = r.to_owned();
    };

    if let Some(t) = matches.get_one::<String>("template") {
        template = t.to_owned();
    }

    let decrypt = matches.get_flag("decrypt");
    let template_list = matches.get_flag("template_list");
    let clear = matches.get_flag("clear");

    let omit = matches.get_flag("omit_recipient");
    if omit {
        recipient = String::from("_omit_recipient_");
    }

    (filename, recipient, template, decrypt, template_list, clear)
}

/*
    UNIT-TEST SECTION BEGINS
*/
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_match_create() {}

//     #[test]
//     fn test_match_modify() {}
// }
