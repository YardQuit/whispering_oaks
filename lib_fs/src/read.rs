use std::fs::File;
use std::io::Read;
use toml::Value;

pub fn read_toml(dir_path: &str, file_name: &str) -> String {
    let file_path = format!("{}/{}", dir_path, file_name);

    let mut toml_content = String::new();
    let toml_file = File::open(file_path);
    match toml_file {
        Ok(mut toml_file) => {
            toml_file.read_to_string(&mut toml_content).expect("\nerror: failed to read the file");
            let parse_toml_value: Value = toml::from_str(&toml_content).expect("\nerror: failed to parse the toml value");
            let return_value = parse_toml_value["config"]["recipient"].as_str().expect("\nerror: key in config file not found");
            String::from(return_value)
        },
        Err(_) => String::from(""),
    }
}
