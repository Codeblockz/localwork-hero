mod files;
mod inference;
mod models;

use std::path::Path;
use std::sync::Mutex;

#[cfg(debug_assertions)]
use log::info;
use files::{FileInfo, FolderPermission, PermissionStore};
use models::{download, ModelInfo};
use tauri::{AppHandle, Emitter, State};

#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    version: String,
}

struct AppState {
    inference: Mutex<Option<inference::LlamaInference>>,
    permissions: Mutex<PermissionStore>,
}

#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo {
        name: "LocalWork Hero".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[tauri::command]
fn list_models() -> Vec<ModelInfo> {
    download::get_available_models()
}

#[derive(Clone, serde::Serialize)]
struct DownloadProgress {
    model_id: String,
    downloaded: u64,
    total: u64,
    percent: f32,
}

#[tauri::command]
async fn download_model(
    app: AppHandle,
    repo_id: String,
    filename: String,
) -> Result<String, String> {
    let app_handle = app.clone();
    let model_id = repo_id.clone();

    // Run in a blocking task to not block the async runtime
    tokio::task::spawn_blocking(move || {
        let path = download::download_model_with_progress(&repo_id, &filename, |downloaded, total| {
            let percent = if total > 0 {
                (downloaded as f32 / total as f32) * 100.0
            } else {
                0.0
            };

            let _ = app_handle.emit(
                "download-progress",
                DownloadProgress {
                    model_id: model_id.clone(),
                    downloaded,
                    total,
                    percent,
                },
            );
        })
        .map_err(|e| e.to_string())?;

        path.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Invalid path".to_string())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
fn load_model(state: State<AppState>, model_path: String) -> Result<(), String> {
    let mut inference_guard = state.inference.lock().map_err(|e| e.to_string())?;

    // Initialize inference if not already done
    if inference_guard.is_none() {
        let inf =
            inference::LlamaInference::new().map_err(|e| format!("Failed to init backend: {}", e))?;
        *inference_guard = Some(inf);
    }

    // Load the model
    if let Some(ref mut inf) = *inference_guard {
        inf.load_model(Path::new(&model_path))
            .map_err(|e| format!("Failed to load model: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
fn send_message(
    state: State<AppState>,
    messages: Vec<inference::Message>,
) -> Result<String, String> {
    let inference_guard = state.inference.lock().map_err(|e| e.to_string())?;

    let inf = inference_guard
        .as_ref()
        .ok_or_else(|| "No model loaded. Please load a model first.".to_string())?;

    if !inf.is_model_loaded() {
        return Err("No model loaded. Please load a model first.".to_string());
    }

    inf.generate(&messages, 512)
        .map_err(|e| format!("Inference error: {}", e))
}

#[tauri::command]
fn grant_folder(
    app: AppHandle,
    state: State<AppState>,
    path: String,
) -> Result<FolderPermission, String> {
    files::permissions::grant_folder_to_scope(&app, &path)?;
    let mut store = state.permissions.lock().map_err(|e| e.to_string())?;
    Ok(store.add(path))
}

#[tauri::command]
fn revoke_folder(state: State<AppState>, id: String) -> Result<(), String> {
    let mut store = state.permissions.lock().map_err(|e| e.to_string())?;
    store
        .remove(&id)
        .ok_or_else(|| "Folder not found".to_string())?;
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
fn read_text_file(state: State<AppState>, path: String) -> Result<String, String> {
    let store = state.permissions.lock().map_err(|e| e.to_string())?;
    files::operations::read_file(&store, &path)
}

#[tauri::command]
fn write_text_file(state: State<AppState>, path: String, content: String) -> Result<(), String> {
    let store = state.permissions.lock().map_err(|e| e.to_string())?;
    files::operations::write_file(&store, &path, &content)
}

#[tauri::command]
fn create_text_file(state: State<AppState>, path: String, content: String) -> Result<(), String> {
    let store = state.permissions.lock().map_err(|e| e.to_string())?;
    files::operations::create_file(&store, &path, &content)
}

#[tauri::command]
fn delete_fs_file(state: State<AppState>, path: String) -> Result<(), String> {
    let store = state.permissions.lock().map_err(|e| e.to_string())?;
    files::operations::delete_file(&store, &path)
}

#[tauri::command]
fn move_fs_file(state: State<AppState>, src: String, dest: String) -> Result<(), String> {
    let store = state.permissions.lock().map_err(|e| e.to_string())?;
    files::operations::move_file(&store, &src, &dest)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
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
            read_text_file,
            write_text_file,
            create_text_file,
            delete_fs_file,
            move_fs_file
        ]);

    // Enable MCP plugin for AI-assisted debugging in development builds
    #[cfg(debug_assertions)]
    {
        info!("Development build detected, enabling MCP plugin for AI debugging");
        builder = builder.plugin(tauri_plugin_mcp::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
