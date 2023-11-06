pub mod arguments {
    extern crate clap;

    use clap::{Arg, Command, ArgAction};

    const AUTHOR: &str = "Author: Michael A Jones (github: YardQuit)";
    const ABOUT: &str = "Encrypt your documents using GPG while leveraging your preferred editor.";
    const FHELP: &str = include_str!("help_filename.txt");
    const RHELP: &str = include_str!("help_recipient.txt");

    pub fn cli_args() -> (String, String) {
        let mut filename = String::new();
        let mut recipient = String::new();

        let matches= Command::new("whispering oaks")
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
        )
        .arg(
            Arg::new("recipient")
                .help(RHELP)
                .required(true)
                .short('r')
                .long("recipient")
                .num_args(1..)
                .action(ArgAction::Set)
        )
        .get_matches();

        if let Some(f) = matches.get_one::<String>("filename") {
            filename = f.to_owned();
            };

        if let Some(r) = matches.get_one::<String>("recipient") {
            recipient = r.to_owned();
            };

        (filename, recipient)
    }
}
