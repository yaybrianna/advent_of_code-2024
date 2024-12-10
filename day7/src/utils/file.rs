use std::fs;

pub fn load_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}
