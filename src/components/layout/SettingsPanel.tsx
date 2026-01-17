import { X, Download, Check, Loader2, FolderPlus, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Progress } from "@/components/ui/progress";
import { type ModelInfo, type FolderPermission } from "@/lib/tauri";

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
  grantedFolders: FolderPermission[];
  onGrantFolder: () => void;
  onRevokeFolder: (id: string) => void;
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
  grantedFolders,
  onGrantFolder,
  onRevokeFolder,
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
      </div>
    </aside>
  );
}
