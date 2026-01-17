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
