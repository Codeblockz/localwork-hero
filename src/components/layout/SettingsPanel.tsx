import { X, Download, Check, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Progress } from "@/components/ui/progress";
import { type ModelInfo } from "@/lib/tauri";

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  models: ModelInfo[];
  selectedModel: string | null;
  onSelectModel: (modelId: string) => void;
  onDownloadModel: (modelId: string) => void;
  downloadingModel: string | null;
  downloadProgress: number | null;
  isLoadingModel: boolean;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  if (bytes < 1024 * 1024 * 1024)
    return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  return (bytes / (1024 * 1024 * 1024)).toFixed(1) + " GB";
}

export function SettingsPanel({
  isOpen,
  onClose,
  models,
  selectedModel,
  onSelectModel,
  onDownloadModel,
  downloadingModel,
  downloadProgress,
  isLoadingModel,
}: SettingsPanelProps) {
  if (!isOpen) return null;

  return (
    <aside className="w-80 border-l bg-background p-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-lg font-semibold">Settings</h2>
        <Button variant="ghost" size="icon" onClick={onClose}>
          <X className="h-4 w-4" />
        </Button>
      </div>
      <div className="space-y-6">
        <section>
          <h3 className="font-medium mb-3">Model</h3>
          <div className="space-y-3">
            {models.map((model) => {
              const isSelected = selectedModel === model.id;
              const isDownloading = downloadingModel === model.id;
              const isLoading = isLoadingModel && isSelected;

              return (
                <div
                  key={model.id}
                  className={`p-3 rounded-lg border ${
                    isSelected ? "border-primary bg-primary/5" : "border-border"
                  }`}
                >
                  <div className="flex items-center justify-between mb-1">
                    <span className="font-medium text-sm">{model.name}</span>
                    <span className="text-xs text-muted-foreground">
                      {formatBytes(model.size_bytes)}
                    </span>
                  </div>

                  {isDownloading && downloadProgress !== null ? (
                    <div className="mt-2">
                      <Progress value={downloadProgress} className="h-2" />
                      <p className="text-xs text-muted-foreground mt-1">
                        Downloading... {downloadProgress.toFixed(0)}%
                      </p>
                    </div>
                  ) : model.downloaded ? (
                    <Button
                      variant={isSelected ? "default" : "outline"}
                      size="sm"
                      className="w-full mt-2"
                      onClick={() => onSelectModel(model.id)}
                      disabled={isLoading}
                    >
                      {isLoading ? (
                        <>
                          <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                          Loading...
                        </>
                      ) : isSelected ? (
                        <>
                          <Check className="h-4 w-4 mr-2" />
                          Selected
                        </>
                      ) : (
                        "Select"
                      )}
                    </Button>
                  ) : (
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
                  )}
                </div>
              );
            })}

            {models.length === 0 && (
              <p className="text-sm text-muted-foreground">
                No models available. Check your connection.
              </p>
            )}
          </div>
        </section>
        <Separator />
        <section>
          <h3 className="font-medium mb-2">Folders</h3>
          <p className="text-sm text-muted-foreground">
            Folder permissions will appear here in Phase 3
          </p>
        </section>
      </div>
    </aside>
  );
}
