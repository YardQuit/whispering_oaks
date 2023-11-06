use libs::depends::sysutils;
use libs::cliargs::arguments;
use libs::encrypt::encfile;
use libs::editsel::editor;

fn main() {
    let (filename,recipient) = arguments::cli_args();
    let temporary_file = encfile::create_filename();
    let editor = editor::editor_selection();

    sysutils::system_dependencies(&editor);
    editor::editor_initiation(&editor, &temporary_file);
    encfile::file_verification(&temporary_file);
    encfile::file_encryption(&filename, &recipient, &temporary_file);
    encfile::file_removal(&temporary_file);
}
