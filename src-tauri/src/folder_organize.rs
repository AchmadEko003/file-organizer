use crate::helpers::{is_document_file, is_image_file, is_video_file};
use walkdir::WalkDir;

#[derive(serde::Serialize)]
pub struct FilesList {
    name: String,
    path: String,
    size: u64,
    is_dir: bool,
    is_file: bool,
}

#[tauri::command]
pub fn get_list_of_files_in_folder(folder_path: &str) -> Result<Vec<FilesList>, String> {
    let paths = std::fs::read_dir(folder_path).map_err(|e| e.to_string())?;
    let mut file_list: Vec<FilesList> = Vec::new();

    for path in paths {
        let path = match path {
            Ok(p) => p,
            Err(_) => continue, // Skip entries we can't read
        };
        let file_path = path.path();
        if let Some(path_str) = file_path.to_str() {
            let file_size = match file_path.metadata() {
                Ok(metadata) => metadata.len(),
                Err(_) => 0, // Default size if we can't read metadata
            };

            file_list.push(FilesList {
                name: path_str.replace(folder_path, "").to_string(),
                path: path_str.to_string(),
                size: file_size,
                is_dir: file_path.is_dir(),
                is_file: file_path.is_file(),
            });
        }
    }

    Ok(file_list)
}

#[tauri::command]
pub fn organize_folder(folder_path: &str) -> Result<(), String> {
    let entries = std::fs::read_dir(folder_path).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        println!("Processing file: {}", path.display());
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                // let ext_folder = Path::new(folder_path).join(ext.to_lowercase());
                let mut ext_folder = std::path::Path::new(folder_path).to_path_buf();

                let ext_info: String = ext.to_lowercase();

                if is_image_file(&ext_info) {
                    ext_folder = ext_folder.join(format!("images/{}", ext_info));
                } else if is_document_file(&ext_info) {
                    ext_folder = ext_folder.join(format!("documents/{}", ext_info));
                } else if is_video_file(&ext_info) {
                    ext_folder = ext_folder.join(format!("videos/{}", ext_info));
                } else {
                    ext_folder = ext_folder.join(format!("others/{}", ext_info));
                }

                std::fs::create_dir_all(&ext_folder).map_err(|e| e.to_string())?;
                let file_name = path
                    .file_name()
                    .ok_or_else(|| "failed to read file name".to_string())?;
                let new_path = ext_folder.join(file_name);
                std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub fn search_file(folder_path: &str, search_term: &str) -> Result<Vec<FilesList>, String> {
    let mut result: Vec<FilesList> = Vec::new();

    for entry in WalkDir::new(folder_path).into_iter() {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if result.len() >= 20 {
            break;
        }

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            // Case-insensitive search
            if filename
                .to_lowercase()
                .contains(&search_term.to_lowercase())
            {
                let name = filename.to_string();
                let file_size = match entry.metadata() {
                    Ok(metadata) => metadata.len(),
                    Err(_) => 0,
                };

                result.push(FilesList {
                    name,
                    path: path.to_string_lossy().into_owned(),
                    size: file_size,
                    is_dir: false,
                    is_file: true,
                });
            }

            println!(
                "Checked file: {} (lowercase: {})",
                filename,
                filename.to_lowercase()
            );
        }
    }

    Ok(result)
}
