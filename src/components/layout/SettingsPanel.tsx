import { X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
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
          <h3 className="font-medium mb-2">Model</h3>
          <p className="text-sm text-muted-foreground">
            Model settings will appear here in Phase 2
          </p>
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
