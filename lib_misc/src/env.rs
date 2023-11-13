use std::env;
use dirs;

pub fn editor_selection() -> String {
    if let Ok(editor) = env::var("WHISPERING_OAKS") {
        return editor;
    }

    if let Ok(editor) = env::var("EDITOR") {
        return editor;
    }

    panic!("\nerror: there is no editor set as either prffered or default)");    
}

pub fn home_path() -> String {
    let home: Option<std::path::PathBuf> = dirs::home_dir();
    match home {
        Some(home_path) => home_path.to_string_lossy().to_string(),
        None => panic!("\nerror: could not find user home directory"),
    }
}

pub fn config_path() -> String {
    let config: Option<std::path::PathBuf> = dirs::config_dir();
    match config {
        Some(config_path) => config_path.to_string_lossy().to_string(),
        None => panic!("\nerror: could not find user config directory"),
    }
}
/*
    UNIT-TESTS SECTION BEGINS
*/
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_env_variable_for_editor() {}
// }
