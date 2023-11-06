use libs::depends::sysutils;
use libs::cliargs::arguments;
use libs::tmpfile::tempfile;
use libs::editsel::editor;

fn main() {
    let (filename,recipient) = arguments::cli_args();
    let temporary_file = tempfile::create_filename();
    let editor = editor::editor_selection();

    sysutils::system_dependencies(&editor);
    editor::editor_initiation(&editor, &temporary_file);
    tempfile::file_verification(&temporary_file);
    tempfile::file_encryption(&filename, &recipient, &temporary_file);
    tempfile::file_removal(&temporary_file);
}
