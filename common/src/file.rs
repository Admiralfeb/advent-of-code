use std::{fs, io};

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn get_common_data_path(year: i32) -> String {
    format!("lib_years/{}/data/", year)
}
