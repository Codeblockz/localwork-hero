use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri_plugin_fs::FsExt;
use uuid::Uuid;

use super::types::FolderPermission;

pub struct PermissionStore {
    folders: HashMap<String, FolderPermission>,
}

impl PermissionStore {
    pub fn new() -> Self {
        Self {
            folders: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: String) -> FolderPermission {
        let id = Uuid::new_v4().to_string();
        let granted_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let perm = FolderPermission {
            id: id.clone(),
            path,
            granted_at,
        };
        self.folders.insert(id.clone(), perm.clone());
        perm
    }

    pub fn remove(&mut self, id: &str) -> Option<FolderPermission> {
        self.folders.remove(id)
    }

    pub fn list(&self) -> Vec<FolderPermission> {
        self.folders.values().cloned().collect()
    }

    pub fn is_path_allowed(&self, path: &str) -> bool {
        self.folders.values().any(|f| path.starts_with(&f.path))
    }
}

impl Default for PermissionStore {
    fn default() -> Self {
        Self::new()
    }
}

pub fn grant_folder_to_scope(app: &AppHandle, path: &str) -> Result<(), String> {
    let scope = app.fs_scope();
    scope
        .allow_directory(path, true)
        .map_err(|e| format!("Failed to grant scope: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store_is_empty() {
        let store = PermissionStore::new();
        assert!(store.list().is_empty());
    }

    #[test]
    fn test_add_folder() {
        let mut store = PermissionStore::new();
        let perm = store.add("/home/user/docs".to_string());

        assert!(!perm.id.is_empty());
        assert_eq!(perm.path, "/home/user/docs");
        assert!(perm.granted_at > 0);
        assert_eq!(store.list().len(), 1);
    }

    #[test]
    fn test_add_multiple_folders() {
        let mut store = PermissionStore::new();
        store.add("/home/user/docs".to_string());
        store.add("/home/user/projects".to_string());
        store.add("/tmp/test".to_string());

        assert_eq!(store.list().len(), 3);
    }

    #[test]
    fn test_remove_folder() {
        let mut store = PermissionStore::new();
        let perm = store.add("/home/user/docs".to_string());

        let removed = store.remove(&perm.id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().path, "/home/user/docs");
        assert!(store.list().is_empty());
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut store = PermissionStore::new();
        let removed = store.remove("nonexistent-id");
        assert!(removed.is_none());
    }

    #[test]
    fn test_is_path_allowed_exact() {
        let mut store = PermissionStore::new();
        store.add("/home/user/docs".to_string());

        assert!(store.is_path_allowed("/home/user/docs"));
    }

    #[test]
    fn test_is_path_allowed_nested() {
        let mut store = PermissionStore::new();
        store.add("/home/user/docs".to_string());

        assert!(store.is_path_allowed("/home/user/docs/file.txt"));
        assert!(store.is_path_allowed("/home/user/docs/subdir/file.txt"));
    }

    #[test]
    fn test_is_path_denied() {
        let mut store = PermissionStore::new();
        store.add("/home/user/docs".to_string());

        assert!(!store.is_path_allowed("/home/user/other"));
        assert!(!store.is_path_allowed("/etc/passwd"));
        assert!(!store.is_path_allowed("/home/user/doc")); // Prefix but not path prefix
    }

    #[test]
    fn test_is_path_denied_empty_store() {
        let store = PermissionStore::new();
        assert!(!store.is_path_allowed("/any/path"));
    }

    #[test]
    fn test_multiple_folders_access() {
        let mut store = PermissionStore::new();
        store.add("/home/user/docs".to_string());
        store.add("/home/user/projects".to_string());

        assert!(store.is_path_allowed("/home/user/docs/file.txt"));
        assert!(store.is_path_allowed("/home/user/projects/code.rs"));
        assert!(!store.is_path_allowed("/home/user/other/file.txt"));
    }

    #[test]
    fn test_default_impl() {
        let store = PermissionStore::default();
        assert!(store.list().is_empty());
    }
}
