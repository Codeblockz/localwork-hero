import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Layout } from "./components/layout/Layout";
import { ChatArea } from "./components/chat/ChatArea";
import { SettingsPanel } from "./components/layout/SettingsPanel";
import {
  getAppInfo,
  listModels,
  downloadModel,
  loadModel,
  sendMessage,
  type Message,
  type ModelInfo,
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

  // Load app info and models on mount
  useEffect(() => {
    getAppInfo().then((info) => {
      console.log(`${info.name} v${info.version}`);
    });

    refreshModels();
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
    setMessages((prev) => [...prev, userMessage]);
    setIsLoading(true);

    try {
      const response = await sendMessage(content);
      const assistantMessage: Message = { role: "assistant", content: response };
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
      />
    </Layout>
  );
}

export default App;
