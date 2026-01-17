use std::fs;
use std::path::Path;

use super::permissions::PermissionStore;
use super::types::FileInfo;

pub fn list_directory(store: &PermissionStore, path: &str) -> Result<Vec<FileInfo>, String> {
    if !store.is_path_allowed(path) {
        return Err("Access denied: folder not in granted permissions".to_string());
    }

    let entries =
        fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let metadata = entry
            .metadata()
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        files.push(FileInfo {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_string_lossy().to_string(),
            is_directory: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata
                .modified()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                .unwrap_or(0),
        });
    }

    Ok(files)
}

pub fn read_file(store: &PermissionStore, path: &str) -> Result<String, String> {
    if !store.is_path_allowed(path) {
        return Err("Access denied: file not in granted folder".to_string());
    }
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

pub fn write_file(store: &PermissionStore, path: &str, content: &str) -> Result<(), String> {
    if !store.is_path_allowed(path) {
        return Err("Access denied: file not in granted folder".to_string());
    }
    fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))
}

pub fn create_file(store: &PermissionStore, path: &str, content: &str) -> Result<(), String> {
    if !store.is_path_allowed(path) {
        return Err("Access denied: location not in granted folder".to_string());
    }
    if Path::new(path).exists() {
        return Err("File already exists".to_string());
    }
    fs::write(path, content).map_err(|e| format!("Failed to create file: {}", e))
}

pub fn delete_file(store: &PermissionStore, path: &str) -> Result<(), String> {
    if !store.is_path_allowed(path) {
        return Err("Access denied: file not in granted folder".to_string());
    }
    fs::remove_file(path).map_err(|e| format!("Failed to delete file: {}", e))
}

pub fn move_file(store: &PermissionStore, src: &str, dest: &str) -> Result<(), String> {
    if !store.is_path_allowed(src) || !store.is_path_allowed(dest) {
        return Err("Access denied: source or destination not in granted folder".to_string());
    }
    fs::rename(src, dest).map_err(|e| format!("Failed to move file: {}", e))
}
