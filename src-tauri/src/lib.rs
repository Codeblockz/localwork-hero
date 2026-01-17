mod inference;
mod models;

use std::path::Path;
use std::sync::Mutex;

#[cfg(debug_assertions)]
use log::info;
use models::{download, ModelInfo};
use tauri::{AppHandle, Emitter, State};

#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    version: String,
}

struct AppState {
    inference: Mutex<Option<inference::LlamaInference>>,
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
fn send_message(state: State<AppState>, prompt: String) -> Result<String, String> {
    let inference_guard = state.inference.lock().map_err(|e| e.to_string())?;

    let inf = inference_guard
        .as_ref()
        .ok_or_else(|| "No model loaded. Please load a model first.".to_string())?;

    if !inf.is_model_loaded() {
        return Err("No model loaded. Please load a model first.".to_string());
    }

    inf.generate(&prompt, 512)
        .map_err(|e| format!("Inference error: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            inference: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            list_models,
            download_model,
            load_model,
            send_message
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
