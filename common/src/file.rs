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

    // Find workspace root by looking for the root Cargo.toml
    let mut workspace_root = current_path.clone();
    while !workspace_root.join("Cargo.toml").exists()
        || workspace_root.join("lib_years").exists() == false
    {
        if let Some(parent) = workspace_root.parent() {
            workspace_root = parent.to_path_buf();
        } else {
            // Fallback to current directory if workspace root not found
            workspace_root = current_path;
            break;
        }
    }

    workspace_root
        .join("lib_years")
        .join(year.to_string())
        .join("data")
        .join(file_name)
}
