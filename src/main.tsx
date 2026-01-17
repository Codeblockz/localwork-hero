import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

// Initialize MCP bridge for AI-assisted debugging in development
if (import.meta.env.DEV) {
  import("tauri-plugin-mcp").then(({ initMcpBridge }) => {
    initMcpBridge().catch((err) => {
      console.warn("[MCP] Bridge initialization failed:", err);
    });
  });
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
