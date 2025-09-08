#[derive(serde::Serialize)]
pub struct FilesList {
    name: String,
    path: String,
}

#[tauri::command]
pub fn get_list_of_files_in_folder(folder_path: &str) -> Vec<FilesList> {
    let paths = std::fs::read_dir(folder_path).unwrap();
    let mut file_list: Vec<FilesList> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.is_file() {
            if let Some(path_str) = file_path.to_str() {
                file_list.push(FilesList {
                    name: path_str.replace(folder_path, "").to_string(),
                    path: path_str.to_string(),
                });
            }
        }
    }

    file_list
}   