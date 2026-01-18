import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { Layout } from "./components/layout/Layout";
import { ChatArea } from "./components/chat/ChatArea";
import { SettingsPanel } from "./components/layout/SettingsPanel";
import {
  getAppInfo,
  listModels,
  downloadModel,
  loadModel,
  sendMessageWithTools,
  listFolders,
  grantFolder,
  revokeFolder,
  type Message,
  type ModelInfo,
  type FolderPermission,
} from "./lib/tauri";

interface DownloadProgressEvent {
  model_id: string;
  downloaded: number;
  total: number;
  percent: number;
}

function App() {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [selectedModel, setSelectedModel] = useState<string | null>(null);
  const [downloadingModel, setDownloadingModel] = useState<string | null>(null);
  const [downloadProgress, setDownloadProgress] = useState<number | null>(null);
  const [isLoadingModel, setIsLoadingModel] = useState(false);
  const [grantedFolders, setGrantedFolders] = useState<FolderPermission[]>([]);

  // Load app info, models, and folders on mount
  useEffect(() => {
    getAppInfo().then((info) => {
      console.log(`${info.name} v${info.version}`);
    });

    refreshModels();
    listFolders().then(setGrantedFolders).catch(console.error);
  }, []);

  const refreshModels = async () => {
    try {
      const modelList = await listModels();
      setModels(modelList);
    } catch (err) {
      console.error("Failed to list models:", err);
    }
  };

  const handleDownloadModel = async (modelId: string) => {
    const model = models.find((m) => m.id === modelId);
    if (!model) return;

    setDownloadingModel(modelId);
    setDownloadProgress(0);

    // Listen for real progress events from Rust
    const unlisten = await listen<DownloadProgressEvent>(
      "download-progress",
      (event) => {
        if (event.payload.model_id === modelId) {
          setDownloadProgress(event.payload.percent);
        }
      }
    );

    try {
      const downloadedPath = await downloadModel(model.id, model.filename);

      // Update the model in state with the downloaded path
      setModels((prev) =>
        prev.map((m) =>
          m.id === modelId ? { ...m, downloaded: true, path: downloadedPath } : m
        )
      );
    } catch (err) {
      console.error("Download failed:", err);
      alert(`Download failed: ${err}`);
    } finally {
      unlisten();
      setDownloadingModel(null);
      setDownloadProgress(null);
    }
  };

  const handleSelectModel = async (modelId: string) => {
    const model = models.find((m) => m.id === modelId);
    if (!model || !model.path) {
      console.error("Model not found or not downloaded");
      return;
    }

    setIsLoadingModel(true);

    try {
      await loadModel(model.path);
      setSelectedModel(modelId);
    } catch (err) {
      console.error("Failed to load model:", err);
    } finally {
      setIsLoadingModel(false);
    }
  };

  const handleSendMessage = async (content: string) => {
    if (!selectedModel) {
      setIsSettingsOpen(true);
      return;
    }

    // Add user message
    const userMessage: Message = { role: "user", content };
    const updatedMessages = [...messages, userMessage];
    setMessages(updatedMessages);
    setIsLoading(true);

    try {
      // Send full conversation history to the backend with tool support
      const response = await sendMessageWithTools(updatedMessages);
      const assistantMessage: Message = {
        role: "assistant",
        content: response.content,
        tool_calls: response.tool_calls.length > 0 ? response.tool_calls : undefined,
      };
      setMessages((prev) => [...prev, assistantMessage]);
    } catch (err) {
      console.error("Inference failed:", err);
      // Add error message to chat
      const errorMessage: Message = {
        role: "assistant",
        content: `Error: ${err}`,
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

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

  return (
    <Layout onSettingsClick={() => setIsSettingsOpen(true)}>
      <ChatArea
        messages={messages}
        isLoading={isLoading}
        onSend={handleSendMessage}
      />
      <SettingsPanel
        isOpen={isSettingsOpen}
        onClose={() => setIsSettingsOpen(false)}
        models={models}
        selectedModel={selectedModel}
        onSelectModel={handleSelectModel}
        onDownloadModel={handleDownloadModel}
        downloadingModel={downloadingModel}
        downloadProgress={downloadProgress}
        isLoadingModel={isLoadingModel}
        grantedFolders={grantedFolders}
        onGrantFolder={handleGrantFolder}
        onRevokeFolder={handleRevokeFolder}
      />
    </Layout>
  );
}

export default App;
