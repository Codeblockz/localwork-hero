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

export interface ToolCall {
  id: string;
  name: string;
  arguments: Record<string, unknown>;
  result?: string;
}

export interface Message {
  role: "user" | "assistant" | "tool";
  content: string;
  tool_calls?: ToolCall[];
}

export interface AgentResponse {
  content: string;
  tool_calls: ToolCall[];
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

export async function sendMessageWithTools(messages: Message[]): Promise<AgentResponse> {
  return invoke<AgentResponse>("send_message_with_tools", { messages });
}

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

export async function readTextFile(path: string): Promise<string> {
  return invoke<string>("read_text_file", { path });
}

export async function writeTextFile(path: string, content: string): Promise<void> {
  return invoke<void>("write_text_file", { path, content });
}

export async function createTextFile(path: string, content: string): Promise<void> {
  return invoke<void>("create_text_file", { path, content });
}

export async function deleteFsFile(path: string): Promise<void> {
  return invoke<void>("delete_fs_file", { path });
}

export async function moveFsFile(src: string, dest: string): Promise<void> {
  return invoke<void>("move_fs_file", { src, dest });
}
