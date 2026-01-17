# LocalWork Hero - Development Guidelines

## Project Overview

LocalWork Hero is a cross-platform desktop application that brings Cowork-style agentic AI to everyone—running entirely on local hardware with open-source models. Built with Tauri v2 (Rust backend), React 19 (TypeScript frontend), and llama.cpp for inference.

## Quick Reference

```bash
pnpm install          # Install dependencies
pnpm tauri dev        # Run in dev mode
pnpm tauri build      # Production build
pnpm run typecheck    # Type check
```

## Core Principles

### KISS - Keep It Simple, Stupid
- Write the simplest code that solves the problem
- Avoid clever solutions when straightforward ones work
- If code needs extensive comments to explain, simplify it instead

### YAGNI - You Aren't Gonna Need It
- Only implement features that are explicitly required NOW
- Don't add configuration options "just in case"
- Don't build abstractions for hypothetical future needs

### DRY - Don't Repeat Yourself
- Extract repeated logic into functions only when it's used 3+ times
- Duplication is better than the wrong abstraction

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | React 19, TypeScript, Vite 7 |
| Styling | Tailwind CSS v4, shadcn/ui |
| Backend | Tauri v2, Rust |
| IPC | Tauri invoke/commands |
| Inference | llama.cpp (Phase 2) |

## File Organization

```
localwork-hero/
├── src/                          # React frontend
│   ├── components/
│   │   ├── ui/                   # shadcn/ui primitives (button, input, etc.)
│   │   ├── chat/                 # Chat feature components
│   │   │   ├── ChatArea.tsx      # Main chat container
│   │   │   ├── ChatInput.tsx     # Message input
│   │   │   └── MessageList.tsx   # Message display
│   │   └── layout/               # Layout components
│   │       ├── Layout.tsx        # Main layout wrapper
│   │       ├── Sidebar.tsx       # Left navigation
│   │       └── SettingsPanel.tsx # Right settings panel
│   ├── lib/
│   │   ├── utils.ts              # shadcn cn() utility
│   │   └── tauri.ts              # Tauri command wrappers
│   ├── App.tsx                   # Main app component
│   ├── main.tsx                  # React entry point
│   └── index.css                 # Tailwind + shadcn styles
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   └── lib.rs                # Tauri commands
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
├── .claude/PRPs/                 # Planning artifacts
│   ├── prds/                     # Product requirement docs
│   ├── plans/                    # Implementation plans
│   └── reports/                  # Implementation reports
└── .github/workflows/ci.yml      # CI/CD pipeline
```

## Coding Standards

### TypeScript/React
- Use TypeScript strict mode
- Prefer `const` over `let`, never use `var`
- Use functional components with hooks
- Keep components under 150 lines; split if larger
- Use `@/` path alias for imports

### Rust
- Follow Rust idioms - use `?` for error propagation
- Prefer `Result` over panics for recoverable errors
- Keep Tauri commands thin - delegate to service modules
- Use meaningful error types, not string errors

### Tauri Commands
```rust
// In lib.rs - define commands
#[tauri::command]
fn get_app_info() -> AppInfo { ... }

// Register in run()
.invoke_handler(tauri::generate_handler![get_app_info])
```

```typescript
// In lib/tauri.ts - wrap for frontend
export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>('get_app_info');
}
```

## Debugging & Fixing Rules

### DO NOT
- Add backwards compatibility shims
- Rename unused variables to `_var` - delete them
- Add defensive checks for impossible states
- Refactor unrelated code while fixing a bug

### DO
- Fix the actual root cause, not symptoms
- Remove dead code completely
- Keep fixes minimal and focused
- If something is unused, delete it

## Commit Standards

- Imperative mood: "Add feature" not "Added feature"
- Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`
- Keep commits atomic - one logical change per commit

## What NOT to Build

Per the PRD, we are explicitly NOT building:
- Cloud sync (local-only by design)
- Team/enterprise features (single-user focus)
- Mobile apps (desktop only)
- Usage monitoring/analytics (privacy-first)
- API/developer integrations (end-user product)

## Current Status

**Phase 1: Foundation** - Complete
- Tauri v2 + React 19 scaffold
- Tailwind CSS v4 + shadcn/ui
- Three-panel layout (sidebar, chat, settings)
- GitHub Actions CI/CD

**Next: Phase 2 - LLM Integration**
- llama.cpp integration
- Model download manager
- Basic chat with local model
