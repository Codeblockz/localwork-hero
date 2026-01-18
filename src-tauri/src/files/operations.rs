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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_store_with_path(path: &str) -> PermissionStore {
        let mut store = PermissionStore::new();
        store.add(path.to_string());
        store
    }

    #[test]
    fn test_list_directory_allowed() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        // Create test files
        fs::write(dir.path().join("file1.txt"), "content1").unwrap();
        fs::write(dir.path().join("file2.txt"), "content2").unwrap();

        let result = list_directory(&store, dir.path().to_str().unwrap());
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_list_directory_denied() {
        let store = PermissionStore::new();
        let result = list_directory(&store, "/tmp/some-path");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Access denied"));
    }

    #[test]
    fn test_read_file_allowed() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello world").unwrap();

        let result = read_file(&store, file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_read_file_denied() {
        let store = PermissionStore::new();
        let result = read_file(&store, "/etc/passwd");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Access denied"));
    }

    #[test]
    fn test_write_file_allowed() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let file_path = dir.path().join("write-test.txt");
        fs::write(&file_path, "original").unwrap();

        let result = write_file(&store, file_path.to_str().unwrap(), "updated content");
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string(&file_path).unwrap(), "updated content");
    }

    #[test]
    fn test_write_file_denied() {
        let store = PermissionStore::new();
        let result = write_file(&store, "/tmp/unauthorized.txt", "content");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Access denied"));
    }

    #[test]
    fn test_create_file_allowed() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let file_path = dir.path().join("new-file.txt");

        let result = create_file(&store, file_path.to_str().unwrap(), "new content");
        assert!(result.is_ok());
        assert!(file_path.exists());
        assert_eq!(fs::read_to_string(&file_path).unwrap(), "new content");
    }

    #[test]
    fn test_create_file_already_exists() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let file_path = dir.path().join("existing.txt");
        fs::write(&file_path, "existing").unwrap();

        let result = create_file(&store, file_path.to_str().unwrap(), "new content");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_delete_file_allowed() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let file_path = dir.path().join("to-delete.txt");
        fs::write(&file_path, "delete me").unwrap();

        let result = delete_file(&store, file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_file_denied() {
        let store = PermissionStore::new();
        let result = delete_file(&store, "/tmp/unauthorized.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Access denied"));
    }

    #[test]
    fn test_move_file_allowed() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let src_path = dir.path().join("source.txt");
        let dest_path = dir.path().join("dest.txt");
        fs::write(&src_path, "move me").unwrap();

        let result = move_file(&store, src_path.to_str().unwrap(), dest_path.to_str().unwrap());
        assert!(result.is_ok());
        assert!(!src_path.exists());
        assert!(dest_path.exists());
        assert_eq!(fs::read_to_string(&dest_path).unwrap(), "move me");
    }

    #[test]
    fn test_move_file_denied_source() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let dest_path = dir.path().join("dest.txt");
        let result = move_file(&store, "/tmp/unauthorized.txt", dest_path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Access denied"));
    }

    #[test]
    fn test_move_file_denied_dest() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        let src_path = dir.path().join("source.txt");
        fs::write(&src_path, "content").unwrap();

        let result = move_file(&store, src_path.to_str().unwrap(), "/tmp/unauthorized.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Access denied"));
    }

    #[test]
    fn test_subfolder_access() {
        let dir = tempdir().unwrap();
        let store = setup_store_with_path(dir.path().to_str().unwrap());

        // Create subfolder
        let subfolder = dir.path().join("subfolder");
        fs::create_dir(&subfolder).unwrap();

        let file_path = subfolder.join("nested.txt");
        fs::write(&file_path, "nested content").unwrap();

        // Should be able to read files in subfolders
        let result = read_file(&store, file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "nested content");
    }
}
