# LocalWork Hero

A free, local, cross-platform desktop AI assistant that brings Cowork-style agentic capabilities to everyone—running entirely on your hardware with open-source models.

## Features

- **100% Local**: All processing happens on your machine. No cloud, no subscriptions, no data leaving your device.
- **Cross-Platform**: Works on Windows, macOS, and Linux.
- **Chat Interface**: Familiar chat-based UX for natural interaction.
- **Agentic Capabilities** (coming soon): File operations, terminal commands, and web browsing with human approval.

## Tech Stack

- **Frontend**: React 19, TypeScript, Vite, Tailwind CSS v4, shadcn/ui
- **Backend**: Tauri v2 (Rust)
- **Inference**: llama.cpp (coming in Phase 2)
- **Default Model**: Qwen3-4B-Instruct (coming in Phase 2)

## Prerequisites

### All Platforms
- [Node.js](https://nodejs.org/) (LTS)
- [pnpm](https://pnpm.io/) v9+
- [Rust](https://rustup.rs/) (stable)

### Linux (Ubuntu/Pop!_OS/Debian)
```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libcairo2-dev libglib2.0-dev librsvg2-dev patchelf
```

### macOS
Xcode Command Line Tools:
```bash
xcode-select --install
```

### Windows
- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- WebView2 (pre-installed on Windows 11)

## Development

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Project Structure

```
localwork-hero/
├── src/                    # React frontend
│   ├── components/
│   │   ├── ui/             # shadcn/ui components
│   │   ├── chat/           # Chat components
│   │   └── layout/         # Layout components
│   ├── lib/                # Utilities
│   └── App.tsx             # Main app component
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   └── lib.rs          # Tauri commands
│   └── tauri.conf.json     # Tauri configuration
└── .github/workflows/      # CI/CD
```

## Roadmap

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Foundation (Tauri + React scaffold) | ✅ Complete |
| 2 | LLM Integration (llama.cpp, model download) | Pending |
| 3 | File Operations (sandboxed file access) | Pending |
| 4 | Terminal Execution (with approval flow) | Pending |
| 5 | Agent Core (planning, tool orchestration) | Pending |
| 6 | Polish & Packaging (installers, auto-update) | Pending |

## License

MIT
