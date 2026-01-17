import { ScrollArea } from "@/components/ui/scroll-area";

export function MessageList() {
  return (
    <ScrollArea className="flex-1 p-4">
      <div className="max-w-3xl mx-auto">
        <div className="text-center py-12">
          <h2 className="text-2xl font-semibold mb-2">
            Welcome to LocalWork Hero!
          </h2>
          <p className="text-muted-foreground">
            Your local AI assistant is ready to help with file tasks.
          </p>
        </div>
      </div>
    </ScrollArea>
  );
}
