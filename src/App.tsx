import { useState, useEffect } from "react";
import { Layout } from "./components/layout/Layout";
import { ChatArea } from "./components/chat/ChatArea";
import { SettingsPanel } from "./components/layout/SettingsPanel";
import { getAppInfo } from "./lib/tauri";

function App() {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  useEffect(() => {
    getAppInfo().then((info) => {
      console.log(`${info.name} v${info.version}`);
    });
  }, []);

  return (
    <Layout onSettingsClick={() => setIsSettingsOpen(true)}>
      <ChatArea />
      <SettingsPanel
        isOpen={isSettingsOpen}
        onClose={() => setIsSettingsOpen(false)}
      />
    </Layout>
  );
}

export default App;
