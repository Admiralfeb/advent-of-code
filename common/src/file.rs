use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn get_common_data_path(year: i32) -> String {
    let current_path = env::current_dir().unwrap();
    let response = current_path
        .join("lib_years")
        .join(year.to_string())
        .join("data");
    println!("path: {:?}", response.to_str().unwrap());
    format!("lib_years/{}/data/", year)
}

pub fn get_data_path(year: i32, file_name: &str) -> PathBuf {
    let current_path = env::current_dir().unwrap();
    current_path
        .join("lib_years")
        .join(year.to_string())
        .join("data")
        .join(file_name)
}
