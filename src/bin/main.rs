use libs::c_prog_dependencies::sysutils;
use libs::c_cli_arguments::arguments;
use libs::c_file_actions::fileact;
use libs::c_file_cryptography::gpgfile;
use libs::c_editor_selection::editor;

fn main() {
    let editor = editor::editor_selection();
    let temporary_file = fileact::file_namegen();

    sysutils::system_dependencies(&editor);

    let (filename,recipient, decrypt) = arguments::cli_args();
    if !decrypt {
        fileact::file_creation(&temporary_file);
        editor::editor_initiation(&editor, &temporary_file);
        fileact::file_verification(&temporary_file);
        gpgfile::file_encryption(&filename, &recipient, &temporary_file);
        fileact::file_removal(&temporary_file);
    } else {
        let new_filename = gpgfile::file_decryption(&filename, &temporary_file);
        editor::editor_initiation(&editor, &temporary_file);
        fileact::file_verification(&temporary_file);
        gpgfile::file_encryption(&new_filename, &recipient, &temporary_file);
        fileact::file_removal(&temporary_file);
    }
}
