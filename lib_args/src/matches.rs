// extern crate clap;
use clap::{Arg, Command, ArgAction};

const AUTHOR: &str = "Author: Michael A Jones (github: YardQuit)";
const ABOUT: &str = "Encrypt your documents using GPG while leveraging your preferred editor.";
const FHELP: &str = include_str!("t_help_filename.txt");
const RHELP: &str = include_str!("t_help_recipient.txt");
const DHELP: &str = include_str!("t_help_decrypt.txt");
const THELP: &str = include_str!("t_help_template.txt");

/* 
    function that matches and handles command-line argumets
*/
pub fn cli_args() -> (String, String, String, bool) {
    let mut filename = String::new();
    let mut recipient = String::new();
    let mut template = String::new();

    let matches = Command::new("whispering oaks")
    .author(AUTHOR)
    .about(ABOUT)
    .arg(
        Arg::new("filename")
            .help(FHELP)
            .required(true)
            .short('f')
            .long("filename")
            .num_args(1)
            .action(ArgAction::Set)
            .aliases(["file", "file-name", "name"])
    )
    .arg(
        Arg::new("recipient")
            .help(RHELP)
            .required(true)
            .short('r')
            .long("recipient")
            .num_args(1..)
            .action(ArgAction::Set)
            .aliases(["receiver", "rec"])
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
            .conflicts_with("decrypt")
        )
    .arg(
        Arg::new("decrypt")
            .help(DHELP)
            .required(false)
            .short('d')
            .num_args(0)
            .long("decrypt")
            .action(ArgAction::SetTrue)
            .aliases(["dec", "decr"])
            .conflicts_with("template")
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
   
    (filename, recipient, template, decrypt)
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
