use std::env;
use dirs;

pub fn editor_selection(preferred: &str, default: &str) -> Result<String, String> {
    if let Ok(editor) = env::var(preferred) {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }; 
    
    if let Ok(editor) = env::var(default) {
        return Ok(editor);
    };

    Err(String::from("faild to read environment variables"))
}

pub fn home_path() -> Result<String, String> {
    let config: Option<std::path::PathBuf> = dirs::home_dir();
    match config {
        Some(config_path) => {
            Ok(config_path.to_string_lossy().to_string())
        },
        None => {
            Err(String::from("could not find user home directory"))
        },
    }
}

pub fn config_path() -> Result<String, String> {
    let config: Option<std::path::PathBuf> = dirs::config_dir();
    match config {
        Some(config_path) => {
            Ok(config_path.to_string_lossy().to_string())
        },
        None => {
            Err(String::from("could not find user config directory"))
        },
    }
}
