import { ChatInput } from "./ChatInput";
import { MessageList } from "./MessageList";

export function ChatArea() {
  return (
    <div className="flex-1 flex flex-col">
      <MessageList />
      <ChatInput />
    </div>
  );
}
