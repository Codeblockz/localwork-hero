import { useEffect, useRef, useState } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { type Message, type ToolCall } from "@/lib/tauri";
import {
  Loader2,
  ChevronDown,
  ChevronRight,
  FileText,
  FolderOpen,
  FilePlus,
  FileEdit,
  Trash2,
  Move,
  Wrench,
} from "lucide-react";

interface MessageListProps {
  messages: Message[];
  isLoading: boolean;
}

function getToolIcon(toolName: string) {
  switch (toolName) {
    case "list_files":
      return FolderOpen;
    case "read_file":
      return FileText;
    case "create_file":
      return FilePlus;
    case "write_file":
      return FileEdit;
    case "delete_file":
      return Trash2;
    case "move_file":
      return Move;
    default:
      return Wrench;
  }
}

function ToolCallDisplay({ toolCall }: { toolCall: ToolCall }) {
  const [isExpanded, setIsExpanded] = useState(false);
  const Icon = getToolIcon(toolCall.name);
  const isError = toolCall.result?.startsWith("Error:");

  return (
    <div className="mt-2 border rounded-md bg-muted/50 overflow-hidden">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full flex items-center gap-2 px-3 py-2 hover:bg-muted/80 transition-colors text-left"
      >
        {isExpanded ? (
          <ChevronDown className="h-4 w-4 shrink-0" />
        ) : (
          <ChevronRight className="h-4 w-4 shrink-0" />
        )}
        <Icon className="h-4 w-4 shrink-0" />
        <span className="font-mono text-sm font-medium">{toolCall.name}</span>
        {toolCall.result && (
          <span
            className={`ml-auto text-xs px-2 py-0.5 rounded ${
              isError
                ? "bg-destructive/20 text-destructive"
                : "bg-green-500/20 text-green-700 dark:text-green-400"
            }`}
          >
            {isError ? "Error" : "Success"}
          </span>
        )}
      </button>
      {isExpanded && (
        <div className="px-3 pb-3 pt-1 space-y-2 border-t">
          <div>
            <span className="text-xs text-muted-foreground font-medium">
              Arguments:
            </span>
            <pre className="mt-1 text-xs bg-background rounded p-2 overflow-x-auto">
              {JSON.stringify(toolCall.arguments, null, 2)}
            </pre>
          </div>
          {toolCall.result && (
            <div>
              <span className="text-xs text-muted-foreground font-medium">
                Result:
              </span>
              <pre
                className={`mt-1 text-xs rounded p-2 overflow-x-auto whitespace-pre-wrap ${
                  isError ? "bg-destructive/10" : "bg-background"
                }`}
              >
                {toolCall.result}
              </pre>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export function MessageList({ messages, isLoading }: MessageListProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, isLoading]);

  if (messages.length === 0 && !isLoading) {
    return (
      <ScrollArea className="flex-1 p-4">
        <div className="max-w-3xl mx-auto">
          <div className="text-center py-12">
            <h2 className="text-2xl font-semibold mb-2">
              Welcome to LocalWork Hero!
            </h2>
            <p className="text-muted-foreground">
              Your local AI assistant is ready to help. Select a model in
              Settings to get started.
            </p>
          </div>
        </div>
      </ScrollArea>
    );
  }

  return (
    <ScrollArea className="flex-1 p-4">
      <div className="max-w-3xl mx-auto space-y-4">
        {messages.map((message, index) => (
          <div
            key={index}
            className={`flex ${message.role === "user" ? "justify-end" : "justify-start"}`}
          >
            <div
              className={`max-w-[80%] rounded-lg px-4 py-2 ${
                message.role === "user"
                  ? "bg-primary text-primary-foreground"
                  : "bg-muted"
              }`}
            >
              <p className="whitespace-pre-wrap">{message.content}</p>
              {message.tool_calls && message.tool_calls.length > 0 && (
                <div className="mt-2 space-y-2">
                  {message.tool_calls.map((toolCall, tcIndex) => (
                    <ToolCallDisplay key={tcIndex} toolCall={toolCall} />
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-muted rounded-lg px-4 py-2 flex items-center gap-2">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Thinking...</span>
            </div>
          </div>
        )}
        <div ref={bottomRef} />
      </div>
    </ScrollArea>
  );
}
