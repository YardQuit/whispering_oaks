pub mod arguments {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(
        author = "Michael Jones", 
        version, 
        about = "Encrypt your documents using GPG while leveraging your preferred editor.", 
        long_about = None
    )]

    pub struct CliArgs {
        #[arg(short, 
            long, 
            help = "To encrypt for a specific user ID and determine
    which public keys to use, you can obtain the
    necessary information by running the command
    'gpg --list-public-keys'.")]
        pub recipient: String,

        #[arg(short, 
            long, 
            help = "To assign a filename to a document, simply provide
    the desired name without including the '.gpg'
    extension. This extension will be added
    automatically. If you want to save the file in a
    particular directory, you can include the
    desired path as part of the filename.")]
        pub filename: String,
    }
}
