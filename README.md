# LocalWork Hero

[![CI](https://github.com/Codeblockz/localwork-hero/actions/workflows/ci.yml/badge.svg)](https://github.com/Codeblockz/localwork-hero/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19-61dafb)](https://react.dev)

A free, local, cross-platform desktop AI assistant that brings Cowork-style agentic capabilities to everyone—running entirely on your hardware with open-source models.

![LocalWork Hero Screenshot](docs/screenshot.png)
<!-- TODO: Add actual screenshot -->

## Why LocalWork Hero?

- **100% Local** — All processing happens on your machine. No cloud, no subscriptions, no data leaving your device.
- **Privacy First** — Your conversations and files stay on your computer. Always.
- **Cross-Platform** — Works on Windows, macOS, and Linux.
- **Open Source** — MIT licensed. Inspect, modify, and contribute.

## Features

- Chat-based interface for natural interaction
- Local LLM inference via llama.cpp ⚠️ **Experimental** - may crash on some systems
- Sandboxed file operations with user approval
- Terminal command execution with approval flow
- Agentic task planning and execution

> **Note:** LLM inference is under active development. If you experience crashes when sending messages, please [open an issue](https://github.com/Codeblockz/localwork-hero/issues) with your system details.

## Quick Start

### Prerequisites

| Platform | Requirements |
|----------|-------------|
| **All** | [Node.js LTS](https://nodejs.org/), [pnpm](https://pnpm.io/) v9+, [Rust](https://rustup.rs/) stable |
| **Linux** | `sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libcairo2-dev libglib2.0-dev librsvg2-dev patchelf` |
| **macOS** | Xcode CLI: `xcode-select --install` |
| **Windows** | [VS C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), WebView2 (pre-installed on Win11) |

### Installation

```bash
# Clone the repository
git clone https://github.com/Codeblockz/localwork-hero.git
cd localwork-hero

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | React 19, TypeScript, Vite 7, Tailwind CSS v4, shadcn/ui |
| Backend | Tauri v2 (Rust) |
| Inference | llama.cpp |
| Default Model | Qwen3-4B-Instruct |

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
│   └── tauri.conf.json     # Tauri config
└── .github/workflows/      # CI/CD
```

## Roadmap

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Foundation (Tauri + React scaffold) | ✅ Complete |
| 2 | LLM Integration (llama.cpp, model download) | ⚠️ Experimental |
| 3 | File Operations (sandboxed file access) | ✅ Complete |
| 4 | Terminal Execution (with approval flow) | 🚧 In Progress |
| 5 | Agent Core (planning, tool orchestration) | Pending |
| 6 | Polish & Packaging (installers, auto-update) | Pending |

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting a PR.

## License

[MIT](LICENSE) — Free for personal and commercial use.

## Acknowledgments

Built with [Tauri](https://tauri.app), [React](https://react.dev), [llama.cpp](https://github.com/ggerganov/llama.cpp), and [shadcn/ui](https://ui.shadcn.com).
