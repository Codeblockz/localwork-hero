import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  name: string;
  version: string;
}

export interface ModelInfo {
  id: string;
  name: string;
  filename: string;
  size_bytes: number;
  path: string | null;
  downloaded: boolean;
}

export interface Message {
  role: "user" | "assistant";
  content: string;
}

export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("get_app_info");
}

export async function listModels(): Promise<ModelInfo[]> {
  return invoke<ModelInfo[]>("list_models");
}

export async function downloadModel(
  repoId: string,
  filename: string
): Promise<string> {
  return invoke<string>("download_model", { repoId, filename });
}

export async function loadModel(modelPath: string): Promise<void> {
  return invoke<void>("load_model", { modelPath });
}

export async function sendMessage(messages: Message[]): Promise<string> {
  return invoke<string>("send_message", { messages });
}
