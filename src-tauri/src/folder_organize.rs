#[derive(serde::Serialize)]
pub struct FilesList {
    name: String,
    path: String,
    size: u64,
    is_dir: bool,
    is_file: bool,
}

#[tauri::command]
pub fn get_list_of_files_in_folder(folder_path: &str) -> Vec<FilesList> {
    let paths = std::fs::read_dir(folder_path).unwrap();
    let mut file_list: Vec<FilesList> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        if let Some(path_str) = file_path.to_str() {
            let file_size = file_path.metadata().unwrap().len();

            file_list.push(FilesList {
                name: path_str.replace(folder_path, "").to_string(),
                path: path_str.to_string(),
                size: file_size,
                is_dir: file_path.is_dir(),
                is_file: file_path.is_file(),
            });
        }
        // if file_path.is_file() {}
    }

    file_list
}
