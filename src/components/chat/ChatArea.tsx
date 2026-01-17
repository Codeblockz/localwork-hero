import { ChatInput } from "./ChatInput";
import { MessageList } from "./MessageList";
import { type Message } from "@/lib/tauri";

interface ChatAreaProps {
  messages: Message[];
  isLoading: boolean;
  onSend: (message: string) => void;
}

export function ChatArea({ messages, isLoading, onSend }: ChatAreaProps) {
  return (
    <div className="flex-1 flex flex-col">
      <MessageList messages={messages} isLoading={isLoading} />
      <ChatInput onSend={onSend} disabled={isLoading} />
    </div>
  );
}
