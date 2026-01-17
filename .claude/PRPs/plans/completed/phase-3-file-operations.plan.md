# Feature: Phase 3 - File Operations

## Summary

Implement sandboxed file system access with user-controlled folder permissions. Users can grant/revoke access to specific folders, and the application provides file operation tools (read, write, create, delete, move, list) that only work within granted folders. This enables the AI agent to safely automate file tasks while maintaining user control and privacy.

## User Story

As a non-technical user
I want to grant my AI assistant access to specific folders
So that it can help me organize, read, and manage files without accessing my entire system

## Problem Statement

Users want AI help with file tasks (organizing downloads, extracting data, batch renaming) but currently the app cannot access any files. The user has no way to grant folder access, and no file operations are available for the AI to use.

## Solution Statement

Add Tauri plugins for file system access and dialogs, create a folder permission management UI in the Settings panel, and implement Rust backend commands for file operations that enforce permission boundaries. Permissions persist across sessions using tauri-plugin-persisted-scope.

## Metadata

| Field            | Value |
|------------------|-------|
| Type             | NEW_CAPABILITY |
| Complexity       | MEDIUM |
| Systems Affected | Rust backend, React frontend, Tauri config |
| Dependencies     | tauri-plugin-fs, tauri-plugin-dialog, tauri-plugin-persisted-scope |
| Estimated Tasks  | 12 |

---

## UX Design

### Before State
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                              BEFORE STATE                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌─────────────┐         ┌─────────────┐         ┌─────────────┐            ║
║   │   User      │ ──────► │   Chat      │ ──────► │  AI says    │            ║
║   │   "Organize │         │   Input     │         │  "I can't   │            ║
║   │   Downloads"│         │             │         │  access     │            ║
║   └─────────────┘         └─────────────┘         │  files"     │            ║
║                                                   └─────────────┘            ║
║                                                                               ║
║   ┌─────────────────────────────────────────────────────────────────┐        ║
║   │ Settings Panel                                                   │        ║
║   │ ┌───────────────────────────────────────────────────────────┐   │        ║
║   │ │ Model: Qwen3-4B ✓                                         │   │        ║
║   │ └───────────────────────────────────────────────────────────┘   │        ║
║   │ ┌───────────────────────────────────────────────────────────┐   │        ║
║   │ │ Folders                                                    │   │        ║
║   │ │ "Folder permissions will appear here in Phase 3"          │   │        ║
║   │ └───────────────────────────────────────────────────────────┘   │        ║
║   └─────────────────────────────────────────────────────────────────┘        ║
║                                                                               ║
║   USER_FLOW: Chat → Request file task → AI cannot help                       ║
║   PAIN_POINT: No file access, placeholder text, no value for file tasks      ║
║   DATA_FLOW: None - file system is completely inaccessible                   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### After State
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                               AFTER STATE                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌─────────────┐         ┌─────────────┐         ┌─────────────┐            ║
║   │   User      │ ──────► │   Chat      │ ──────► │  AI reads   │            ║
║   │   "Organize │         │   Input     │         │  files,     │            ║
║   │   Downloads"│         │             │         │  moves them │            ║
║   └─────────────┘         └─────────────┘         └──────┬──────┘            ║
║                                                          │                    ║
║                                                          ▼                    ║
║                                                   ┌─────────────┐            ║
║                                                   │ Confirmation│            ║
║                                                   │ Dialog      │            ║
║                                                   │ [Approve]   │            ║
║                                                   └─────────────┘            ║
║                                                                               ║
║   ┌─────────────────────────────────────────────────────────────────┐        ║
║   │ Settings Panel                                                   │        ║
║   │ ┌───────────────────────────────────────────────────────────┐   │        ║
║   │ │ Model: Qwen3-4B ✓                                         │   │        ║
║   │ └───────────────────────────────────────────────────────────┘   │        ║
║   │ ┌───────────────────────────────────────────────────────────┐   │        ║
║   │ │ Folders                                                    │   │        ║
║   │ │ ┌─────────────────────────────────────────────────────┐   │   │        ║
║   │ │ │ ~/Downloads                              [Revoke]   │   │   │        ║
║   │ │ └─────────────────────────────────────────────────────┘   │   │        ║
║   │ │ ┌─────────────────────────────────────────────────────┐   │   │        ║
║   │ │ │ ~/Documents/Projects                     [Revoke]   │   │   │        ║
║   │ │ └─────────────────────────────────────────────────────┘   │   │        ║
║   │ │ [+ Add Folder]                                            │   │        ║
║   │ └───────────────────────────────────────────────────────────┘   │        ║
║   └─────────────────────────────────────────────────────────────────┘        ║
║                                                                               ║
║   USER_FLOW: Settings → Add Folder → Pick directory → Folder granted         ║
║              Chat → Request file task → AI operates → Confirm → Done         ║
║   VALUE_ADD: File automation enabled, user controls access, persists         ║
║   DATA_FLOW: Frontend → Tauri dialog → OS picker → Rust scope → Persist     ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Interaction Changes
| Location | Before | After | User Impact |
|----------|--------|-------|-------------|
| Settings Panel | Placeholder text | Folder list + Add button | Can manage folder access |
| Add Folder | N/A | OS folder picker dialog | Intuitive folder selection |
| Folder List | N/A | Shows path + Revoke button | Can see and remove access |
| File Operations | Not possible | Available to AI agent | AI can automate file tasks |
| App Restart | N/A | Permissions restored | No need to re-grant folders |

---

## Mandatory Reading

**CRITICAL: Implementation agent MUST read these files before starting any task:**

| Priority | File | Lines | Why Read This |
|----------|------|-------|---------------|
| P0 | `src-tauri/src/lib.rs` | 1-145 | Pattern for Tauri commands, state management, event emission |
| P0 | `src/lib/tauri.ts` | 1-43 | Pattern for TypeScript wrappers and type definitions |
| P1 | `src/components/layout/SettingsPanel.tsx` | 1-133 | Where folder UI integrates, existing component patterns |
| P1 | `src-tauri/Cargo.toml` | 1-30 | Current dependencies, where to add plugins |
| P1 | `src-tauri/capabilities/default.json` | 1-10 | Capability configuration pattern |
| P2 | `src-tauri/src/models/download.rs` | all | Pattern for module organization |
| P2 | `src/App.tsx` | 1-160 | State management patterns, event listening |

**External Documentation:**
| Source | Section | Why Needed |
|--------|---------|------------|
| [Tauri FS Plugin](https://v2.tauri.app/plugin/file-system) | Entire page | File operations API, scope management |
| [Tauri Dialog Plugin](https://v2.tauri.app/plugin/dialog) | open() function | Directory picker dialog |
| [Tauri Persisted Scope](https://v2.tauri.app/plugin/persisted-scope) | Setup section | Must init after FS plugin |
| [FsExt Trait](https://v2.tauri.app/plugin/file-system) | Rust section | allow_directory, scope.allowed() |

---

## Patterns to Mirror

**NAMING_CONVENTION:**
```rust
// SOURCE: src-tauri/src/lib.rs:22-28
// COPY THIS PATTERN for new commands:
#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo {
        name: "LocalWork Hero".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}
```

**ERROR_HANDLING:**
```rust
// SOURCE: src-tauri/src/lib.rs:82-98
// COPY THIS PATTERN - commands return Result<T, String>:
#[tauri::command]
fn load_model(state: State<AppState>, model_path: String) -> Result<(), String> {
    let mut inference_guard = state.inference.lock().map_err(|e| e.to_string())?;
    // ...
    Ok(())
}
```

**TYPESCRIPT_WRAPPER:**
```typescript
// SOURCE: src/lib/tauri.ts:30-35
// COPY THIS PATTERN for invoke wrappers:
export async function downloadModel(
  repoId: string,
  filename: string
): Promise<string> {
  return invoke<string>("download_model", { repoId, filename });
}
```

**TYPE_DEFINITION:**
```typescript
// SOURCE: src/lib/tauri.ts:8-15
// COPY THIS PATTERN for interfaces:
export interface ModelInfo {
  id: string;
  name: string;
  filename: string;
  size_bytes: number;
  path: string | null;
  downloaded: boolean;
}
```

**SETTINGS_SECTION:**
```typescript
// SOURCE: src/components/layout/SettingsPanel.tsx:124-129
// REPLACE THIS placeholder with folder management:
<section>
  <h3 className="font-medium mb-2">Folders</h3>
  <p className="text-sm text-muted-foreground">
    Folder permissions will appear here in Phase 3
  </p>
</section>
```

**BUTTON_WITH_ICON:**
```typescript
// SOURCE: src/components/layout/SettingsPanel.tsx:101-110
// COPY THIS PATTERN for buttons with icons:
<Button
  variant="outline"
  size="sm"
  className="w-full mt-2"
  onClick={() => onDownloadModel(model.id)}
  disabled={downloadingModel !== null}
>
  <Download className="h-4 w-4 mr-2" />
  Download
</Button>
```

---

## Files to Change

| File | Action | Justification |
|------|--------|---------------|
| `src-tauri/Cargo.toml` | UPDATE | Add fs, dialog, persisted-scope plugin dependencies |
| `src-tauri/capabilities/default.json` | UPDATE | Add fs and dialog permissions |
| `src-tauri/src/lib.rs` | UPDATE | Add plugins init, new commands |
| `src-tauri/src/files/mod.rs` | CREATE | Public exports for files module |
| `src-tauri/src/files/types.rs` | CREATE | FolderPermission struct |
| `src-tauri/src/files/permissions.rs` | CREATE | Permission check logic |
| `src-tauri/src/files/operations.rs` | CREATE | File operation functions |
| `src/lib/tauri.ts` | UPDATE | Add folder/file types and wrappers |
| `src/components/layout/SettingsPanel.tsx` | UPDATE | Replace placeholder with folder UI |
| `src/App.tsx` | UPDATE | Add folder state management |

---

## NOT Building (Scope Limits)

Explicit exclusions to prevent scope creep:

- **File confirmation dialogs**: Out of scope for Phase 3 - needed for Phase 5 (Agent Core) when AI requests operations
- **File browser/explorer UI**: Not building a full file manager, just permission management
- **Drag-and-drop folder granting**: Using OS native dialog only
- **Recursive permission display**: Not showing subfolders in UI, just top-level grants
- **File operation history/undo**: Not tracking operations in this phase
- **Binary file support**: Text files only for read/write initially

---

## Step-by-Step Tasks

Execute in order. Each task is atomic and independently verifiable.

### Task 1: UPDATE `src-tauri/Cargo.toml` - Add plugin dependencies

- **ACTION**: Add Tauri plugin crates for fs, dialog, and persisted-scope
- **IMPLEMENT**: Add these dependencies to `[dependencies]` section:
  ```toml
  tauri-plugin-fs = "2"
  tauri-plugin-dialog = "2"
  tauri-plugin-persisted-scope = "2"
  ```
- **MIRROR**: Follow existing dependency pattern in Cargo.toml:15-25
- **GOTCHA**: Versions must be compatible with tauri = "2"
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 2: UPDATE `src-tauri/capabilities/default.json` - Add permissions

- **ACTION**: Add fs and dialog plugin permissions
- **IMPLEMENT**: Update permissions array:
  ```json
  {
    "$schema": "../gen/schemas/desktop-schema.json",
    "identifier": "default",
    "description": "Capability for the main window",
    "windows": ["main"],
    "permissions": [
      "core:default",
      "opener:default",
      "fs:default",
      "dialog:default"
    ]
  }
  ```
- **MIRROR**: Existing permission format in default.json:6-8
- **GOTCHA**: fs:default enables basic operations; scope is managed dynamically via FsExt
- **VALIDATE**: `pnpm tauri dev` should start without capability errors

### Task 3: CREATE `src-tauri/src/files/mod.rs` - Module exports

- **ACTION**: Create files module with public exports
- **IMPLEMENT**:
  ```rust
  pub mod types;
  pub mod permissions;
  pub mod operations;

  pub use types::FolderPermission;
  pub use permissions::*;
  pub use operations::*;
  ```
- **MIRROR**: `src-tauri/src/models/mod.rs` pattern
- **VALIDATE**: `cargo check` (after creating submodules)

### Task 4: CREATE `src-tauri/src/files/types.rs` - Type definitions

- **ACTION**: Define FolderPermission struct for frontend/backend communication
- **IMPLEMENT**:
  ```rust
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct FolderPermission {
      pub id: String,
      pub path: String,
      pub granted_at: u64,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct FileInfo {
      pub name: String,
      pub path: String,
      pub is_directory: bool,
      pub size: u64,
      pub modified: u64,
  }
  ```
- **MIRROR**: `src-tauri/src/models/types.rs` struct pattern with serde derives
- **GOTCHA**: Use u64 for timestamps (Unix epoch), String for paths (cross-platform)
- **VALIDATE**: `cargo check`

### Task 5: CREATE `src-tauri/src/files/permissions.rs` - Permission management

- **ACTION**: Create functions to manage folder permissions via Tauri scope
- **IMPLEMENT**:
  ```rust
  use std::collections::HashMap;
  use std::sync::Mutex;
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
          Self { folders: HashMap::new() }
      }

      pub fn add(&mut self, path: String) -> FolderPermission {
          let id = Uuid::new_v4().to_string();
          let granted_at = SystemTime::now()
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs();
          let perm = FolderPermission { id: id.clone(), path, granted_at };
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

  pub fn grant_folder_to_scope(app: &AppHandle, path: &str) -> Result<(), String> {
      let scope = app.fs_scope();
      scope.allow_directory(path, true)
          .map_err(|e| format!("Failed to grant scope: {}", e))
  }
  ```
- **MIRROR**: State management pattern from lib.rs:18-20
- **GOTCHA**: Use `allow_directory(path, true)` - second arg enables recursive access
- **IMPORTS**: Add `uuid = "1"` to Cargo.toml
- **VALIDATE**: `cargo check`

### Task 6: CREATE `src-tauri/src/files/operations.rs` - File operations

- **ACTION**: Create file operation functions that check permissions
- **IMPLEMENT**:
  ```rust
  use std::fs;
  use std::path::Path;

  use super::types::FileInfo;
  use super::permissions::PermissionStore;

  pub fn list_directory(
      store: &PermissionStore,
      path: &str,
  ) -> Result<Vec<FileInfo>, String> {
      if !store.is_path_allowed(path) {
          return Err("Access denied: folder not in granted permissions".to_string());
      }

      let entries = fs::read_dir(path)
          .map_err(|e| format!("Failed to read directory: {}", e))?;

      let mut files = Vec::new();
      for entry in entries {
          let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
          let metadata = entry.metadata()
              .map_err(|e| format!("Failed to read metadata: {}", e))?;

          files.push(FileInfo {
              name: entry.file_name().to_string_lossy().to_string(),
              path: entry.path().to_string_lossy().to_string(),
              is_directory: metadata.is_dir(),
              size: metadata.len(),
              modified: metadata.modified()
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
  ```
- **MIRROR**: Error handling pattern from lib.rs with `.map_err(|e| format!(...))`
- **GOTCHA**: Check permissions BEFORE any operation to fail fast
- **VALIDATE**: `cargo check`

### Task 7: UPDATE `src-tauri/Cargo.toml` - Add uuid dependency

- **ACTION**: Add uuid crate for permission IDs
- **IMPLEMENT**: Add to dependencies:
  ```toml
  uuid = { version = "1", features = ["v4"] }
  ```
- **VALIDATE**: `cargo check`

### Task 8: UPDATE `src-tauri/src/lib.rs` - Add plugins and commands

- **ACTION**: Initialize plugins, add state, register new commands
- **IMPLEMENT**:
  1. Add module declaration: `mod files;`
  2. Import new types: `use files::{FolderPermission, FileInfo, permissions::PermissionStore};`
  3. Update AppState:
     ```rust
     struct AppState {
         inference: Mutex<Option<inference::LlamaInference>>,
         permissions: Mutex<PermissionStore>,
     }
     ```
  4. Add new commands:
     ```rust
     #[tauri::command]
     fn grant_folder(app: AppHandle, state: State<AppState>, path: String) -> Result<FolderPermission, String> {
         files::permissions::grant_folder_to_scope(&app, &path)?;
         let mut store = state.permissions.lock().map_err(|e| e.to_string())?;
         Ok(store.add(path))
     }

     #[tauri::command]
     fn revoke_folder(state: State<AppState>, id: String) -> Result<(), String> {
         let mut store = state.permissions.lock().map_err(|e| e.to_string())?;
         store.remove(&id).ok_or_else(|| "Folder not found".to_string())?;
         Ok(())
     }

     #[tauri::command]
     fn list_folders(state: State<AppState>) -> Result<Vec<FolderPermission>, String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         Ok(store.list())
     }

     #[tauri::command]
     fn list_files(state: State<AppState>, path: String) -> Result<Vec<FileInfo>, String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         files::operations::list_directory(&store, &path)
     }

     #[tauri::command]
     fn read_file(state: State<AppState>, path: String) -> Result<String, String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         files::operations::read_file(&store, &path)
     }

     #[tauri::command]
     fn write_file(state: State<AppState>, path: String, content: String) -> Result<(), String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         files::operations::write_file(&store, &path, &content)
     }

     #[tauri::command]
     fn create_file(state: State<AppState>, path: String, content: String) -> Result<(), String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         files::operations::create_file(&store, &path, &content)
     }

     #[tauri::command]
     fn delete_file(state: State<AppState>, path: String) -> Result<(), String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         files::operations::delete_file(&store, &path)
     }

     #[tauri::command]
     fn move_file(state: State<AppState>, src: String, dest: String) -> Result<(), String> {
         let store = state.permissions.lock().map_err(|e| e.to_string())?;
         files::operations::move_file(&store, &src, &dest)
     }
     ```
  5. Update run() to init plugins and state:
     ```rust
     pub fn run() {
         let mut builder = tauri::Builder::default()
             .plugin(tauri_plugin_fs::init())
             .plugin(tauri_plugin_dialog::init())
             .plugin(tauri_plugin_persisted_scope::init())
             .plugin(tauri_plugin_opener::init())
             .manage(AppState {
                 inference: Mutex::new(None),
                 permissions: Mutex::new(PermissionStore::new()),
             })
             .invoke_handler(tauri::generate_handler![
                 get_app_info,
                 list_models,
                 download_model,
                 load_model,
                 send_message,
                 grant_folder,
                 revoke_folder,
                 list_folders,
                 list_files,
                 read_file,
                 write_file,
                 create_file,
                 delete_file,
                 move_file
             ]);
         // ... rest unchanged
     }
     ```
- **MIRROR**: Command pattern from lib.rs:22-28, state from lib.rs:82-98
- **GOTCHA**: fs plugin MUST init BEFORE persisted-scope plugin!
- **VALIDATE**: `cargo check && cargo build`

### Task 9: UPDATE `src/lib/tauri.ts` - Add TypeScript types and wrappers

- **ACTION**: Add folder/file types and invoke wrappers
- **IMPLEMENT**: Add after existing exports:
  ```typescript
  // Folder permissions
  export interface FolderPermission {
    id: string;
    path: string;
    granted_at: number;
  }

  export interface FileInfo {
    name: string;
    path: string;
    is_directory: boolean;
    size: number;
    modified: number;
  }

  export async function grantFolder(path: string): Promise<FolderPermission> {
    return invoke<FolderPermission>("grant_folder", { path });
  }

  export async function revokeFolder(id: string): Promise<void> {
    return invoke<void>("revoke_folder", { id });
  }

  export async function listFolders(): Promise<FolderPermission[]> {
    return invoke<FolderPermission[]>("list_folders");
  }

  export async function listFiles(path: string): Promise<FileInfo[]> {
    return invoke<FileInfo[]>("list_files", { path });
  }

  export async function readFile(path: string): Promise<string> {
    return invoke<string>("read_file", { path });
  }

  export async function writeFile(path: string, content: string): Promise<void> {
    return invoke<void>("write_file", { path, content });
  }

  export async function createFile(path: string, content: string): Promise<void> {
    return invoke<void>("create_file", { path, content });
  }

  export async function deleteFile(path: string): Promise<void> {
    return invoke<void>("delete_file", { path });
  }

  export async function moveFile(src: string, dest: string): Promise<void> {
    return invoke<void>("move_file", { src, dest });
  }
  ```
- **MIRROR**: Existing wrapper pattern from tauri.ts:30-35
- **VALIDATE**: `pnpm typecheck`

### Task 10: UPDATE `src/App.tsx` - Add folder state management

- **ACTION**: Add state and handlers for folder permissions
- **IMPLEMENT**:
  1. Import new functions: `import { ..., listFolders, grantFolder, revokeFolder, type FolderPermission } from "@/lib/tauri";`
  2. Add dialog import: `import { open } from "@tauri-apps/plugin-dialog";`
  3. Add state:
     ```typescript
     const [grantedFolders, setGrantedFolders] = useState<FolderPermission[]>([]);
     ```
  4. Load folders on mount (add to existing useEffect or create new):
     ```typescript
     useEffect(() => {
       listFolders().then(setGrantedFolders).catch(console.error);
     }, []);
     ```
  5. Add handler functions:
     ```typescript
     const handleGrantFolder = async () => {
       const selected = await open({ directory: true, multiple: false });
       if (selected && typeof selected === "string") {
         try {
           const perm = await grantFolder(selected);
           setGrantedFolders((prev) => [...prev, perm]);
         } catch (err) {
           console.error("Failed to grant folder:", err);
           alert(`Failed to grant folder access: ${err}`);
         }
       }
     };

     const handleRevokeFolder = async (id: string) => {
       try {
         await revokeFolder(id);
         setGrantedFolders((prev) => prev.filter((f) => f.id !== id));
       } catch (err) {
         console.error("Failed to revoke folder:", err);
         alert(`Failed to revoke folder access: ${err}`);
       }
     };
     ```
  6. Pass to SettingsPanel:
     ```typescript
     <SettingsPanel
       // ... existing props
       grantedFolders={grantedFolders}
       onGrantFolder={handleGrantFolder}
       onRevokeFolder={handleRevokeFolder}
     />
     ```
- **MIRROR**: Existing async handler pattern from App.tsx:51-85
- **GOTCHA**: Must install @tauri-apps/plugin-dialog: `pnpm add @tauri-apps/plugin-dialog`
- **VALIDATE**: `pnpm typecheck`

### Task 11: Install frontend plugin dependency

- **ACTION**: Add dialog plugin to frontend
- **IMPLEMENT**: Run `pnpm add @tauri-apps/plugin-dialog`
- **VALIDATE**: Check package.json has the dependency

### Task 12: UPDATE `src/components/layout/SettingsPanel.tsx` - Add folder UI

- **ACTION**: Replace placeholder with folder management UI
- **IMPLEMENT**:
  1. Add imports:
     ```typescript
     import { FolderPlus, Trash2 } from "lucide-react";
     import { type FolderPermission } from "@/lib/tauri";
     ```
  2. Update props interface:
     ```typescript
     interface SettingsPanelProps {
       // ... existing props
       grantedFolders: FolderPermission[];
       onGrantFolder: () => void;
       onRevokeFolder: (id: string) => void;
     }
     ```
  3. Add props to destructuring
  4. Replace the Folders section (lines 124-129) with:
     ```typescript
     <section>
       <div className="flex justify-between items-center mb-3">
         <h3 className="font-medium">Folders</h3>
         <Button variant="ghost" size="sm" onClick={onGrantFolder}>
           <FolderPlus className="h-4 w-4 mr-1" />
           Add
         </Button>
       </div>
       <div className="space-y-2">
         {grantedFolders.map((folder) => (
           <div
             key={folder.id}
             className="flex items-center justify-between p-2 rounded-lg border bg-muted/50"
           >
             <span className="text-sm truncate flex-1 mr-2" title={folder.path}>
               {folder.path}
             </span>
             <Button
               variant="ghost"
               size="icon"
               className="h-8 w-8 text-muted-foreground hover:text-destructive"
               onClick={() => onRevokeFolder(folder.id)}
             >
               <Trash2 className="h-4 w-4" />
             </Button>
           </div>
         ))}
         {grantedFolders.length === 0 && (
           <p className="text-sm text-muted-foreground">
             No folders granted. Click Add to allow file access.
           </p>
         )}
       </div>
     </section>
     ```
- **MIRROR**: Button pattern from SettingsPanel.tsx:101-110, list pattern from lines 51-114
- **GOTCHA**: Use truncate class with title attr for long paths
- **VALIDATE**: `pnpm typecheck && pnpm tauri dev`

---

## Testing Strategy

### Manual Testing Checklist

| Test Case | Steps | Expected Result |
|-----------|-------|-----------------|
| Add folder | Settings → Add → Pick folder | Folder appears in list |
| Revoke folder | Click trash icon on folder | Folder removed from list |
| Persist across restart | Add folder → Restart app | Folder still in list |
| File operations | Grant folder → Use AI to list files | Files listed correctly |
| Permission denied | Try to read file outside granted folders | Error returned |

### Edge Cases Checklist

- [ ] Cancel folder picker dialog - state unchanged
- [ ] Grant same folder twice - handle gracefully (allow or dedupe)
- [ ] Revoke non-existent folder - error handled
- [ ] Read file in nested subfolder of granted folder - should work
- [ ] Path with spaces and special characters - works on all platforms
- [ ] Very long path names - UI handles with truncation

---

## Validation Commands

### Level 1: STATIC_ANALYSIS

```bash
cd src-tauri && cargo check && cargo clippy
pnpm typecheck
```

**EXPECT**: Exit 0, no errors or warnings

### Level 2: BUILD_TEST

```bash
pnpm tauri build --debug
```

**EXPECT**: Build succeeds without errors

### Level 3: MANUAL_VALIDATION

1. Run `pnpm tauri dev`
2. Open Settings panel
3. Click "Add" in Folders section
4. Select a folder via OS dialog
5. Verify folder appears in list
6. Close and reopen app
7. Verify folder persists
8. Click trash icon to revoke
9. Verify folder is removed

---

## Acceptance Criteria

- [ ] User can grant folder access via OS native dialog
- [ ] Granted folders display in Settings panel
- [ ] User can revoke folder access
- [ ] Permissions persist across app restarts
- [ ] File operations work within granted folders
- [ ] File operations fail outside granted folders
- [ ] No TypeScript or Rust compilation errors
- [ ] App builds successfully for all platforms

---

## Completion Checklist

- [ ] Task 1: Cargo.toml updated with plugin dependencies
- [ ] Task 2: Capabilities updated with fs/dialog permissions
- [ ] Task 3: files/mod.rs created
- [ ] Task 4: files/types.rs created
- [ ] Task 5: files/permissions.rs created
- [ ] Task 6: files/operations.rs created
- [ ] Task 7: uuid dependency added
- [ ] Task 8: lib.rs updated with plugins and commands
- [ ] Task 9: tauri.ts updated with types and wrappers
- [ ] Task 10: App.tsx updated with state and handlers
- [ ] Task 11: Dialog plugin installed
- [ ] Task 12: SettingsPanel.tsx updated with folder UI
- [ ] Level 1: Static analysis passes
- [ ] Level 2: Build succeeds
- [ ] Level 3: Manual testing passes

---

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| persisted-scope doesn't restore permissions | Medium | High | Test persistence explicitly; fall back to storing in app config if needed |
| Cross-platform path handling issues | Medium | Medium | Use std::path::Path consistently; test on Win/Mac/Linux |
| Dialog plugin version mismatch | Low | Medium | Pin versions to match tauri v2 |
| Large directory listings slow | Low | Low | Add pagination in future if needed |

---

## Notes

- **Phase 4 Integration**: Terminal execution (Phase 4) can run in parallel since it doesn't depend on file operations
- **Phase 5 Dependency**: Agent Core will use these file operations; ensure stable API
- **Future Enhancement**: File operation confirmation dialogs should be added in Phase 5 when the agent requests operations
- **Security Note**: Permissions are enforced at the Rust layer, so even malicious frontend code cannot bypass them
- **Platform Testing**: macOS may require additional entitlements for certain directories; test thoroughly
