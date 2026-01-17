# LocalWork Hero

## Problem Statement

Non-technical users want agentic AI assistance for everyday desktop tasks (file management, data extraction, automation) but face barriers: Anthropic's Cowork costs $100-200/month, is Mac-only, and sends data to the cloud. Existing open-source alternatives (Cline, Aider, OpenHands) target developers with CLI/IDE interfaces, leaving mainstream users without options.

## Evidence

- Cowork launched Jan 2025 with $100-200/month pricing, Mac-only availability
- 60% of developers plan to use Ollama for local LLM work (2025 StackOverflow Survey)
- No existing tool combines: local LLMs + desktop GUI + agentic file/terminal access + non-technical UX
- Open-source LLMs (Llama 3, Qwen, Phi-3) now capable enough for agentic tasks on consumer hardware

## Proposed Solution

A cross-platform desktop application that brings Cowork-style agentic AI to everyone—running entirely on local hardware with open-source models. Built with Tauri for the UI, llama.cpp for inference, and a LangGraph-based agent for orchestration. Users grant folder access, and the AI can read, edit, create files, run terminal commands, and eventually browse the web—all without data leaving their machine.

## Key Hypothesis

We believe a free, local, cross-platform Cowork alternative will enable non-technical users to automate desktop tasks privately.
We'll know we're right when users complete multi-step file/terminal tasks without touching the command line.

## What We're NOT Building

- Cloud sync - local-only by design
- Team/enterprise features - single-user focus
- Mobile apps - desktop only
- Usage monitoring/analytics - privacy-first
- API/developer integrations - end-user product

## Success Metrics

| Metric | Target | How Measured |
|--------|--------|--------------|
| Task completion rate | >70% of started tasks completed | Local telemetry (opt-in) |
| Hardware compatibility | Runs on 8GB RAM, CPU-only | QA testing matrix |
| First-task time | <5 minutes from install | User testing |
| Cross-platform parity | Feature parity across Win/Mac/Linux | Release checklist |

## Open Questions

- [ ] How to handle model downloads for users with slow internet?
- [ ] Should terminal access be enabled by default or require explicit opt-in?
- [ ] How to communicate AI limitations to non-technical users?

---

## Users & Context

**Primary User**
- **Who**: Non-technical knowledge workers (admins, researchers, small business owners) who want AI automation but can't/won't use CLI tools or pay $100+/month
- **Current behavior**: Manual file organization, copy-paste between apps, no automation
- **Trigger**: Repetitive task frustration ("I wish I could just tell my computer to organize these files")
- **Success state**: Task completed through natural language, no technical knowledge required

**Job to Be Done**
When I have repetitive file/data tasks on my desktop, I want to describe what I need in plain English, so I can get it done without learning technical tools or paying for expensive subscriptions.

**Non-Users**
- Developers who prefer CLI (use Claude Code, Aider, etc.)
- Users who need cloud/team features
- Enterprise customers requiring compliance/audit features
- Users without 8GB+ RAM

---

## Solution Detail

### Core Capabilities (MoSCoW)

| Priority | Capability | Rationale |
|----------|------------|-----------|
| Must | Sandboxed file operations (read/write/create in granted folders) | Core value prop—safe file automation |
| Must | Terminal command execution with approval flow | Enables powerful automation while maintaining safety |
| Must | Local LLM inference via llama.cpp | Privacy, no API costs, offline capable |
| Must | Default model bundled/auto-downloaded | Zero-config first run for non-technical users |
| Must | Cross-platform (Windows, macOS, Linux) | "For everyone" positioning |
| Must | Chat-based interface | Familiar UX pattern |
| Should | Text-based web browsing (fetch + parse) | Extends utility without multimodal requirement |
| Should | Accessibility-tree web browsing (DOM parsing) | Richer web interaction on modest hardware |
| Should | Custom GGUF model loading | Power users bring their own models |
| Could | Skills system (loadable capability modules) | Extensibility for specific workflows |
| Could | MCP server connections | Integration with external tools |
| Could | Visual web browsing (multimodal) | Full Cowork parity for capable hardware |
| Won't | Cloud sync | Out of scope—local only |
| Won't | Multi-user/team features | Out of scope—single user |
| Won't | Mobile | Out of scope—desktop only |

### MVP Scope

**v1.0 - Core Agent**
- Desktop app installer for Windows, macOS, Linux
- Chat interface with conversation history
- Folder permission system (user grants access to specific folders)
- File operations: read, write, create, delete, move, rename within granted folders
- Terminal execution with human-in-the-loop approval
- Default model (Qwen3-4B-Instruct Q4_K_M, ~2.5GB) auto-downloaded on first run
- Basic settings (model selection, folder permissions)

### User Flow

```
Install → First Run → Download Model → Grant Folder Access → Chat → Agent Plans → User Approves → Agent Executes → Results Shown
```

1. User downloads installer for their OS
2. First run: app downloads default model (Qwen3-4B Q4_K_M, ~2.5GB)
3. User grants access to one or more folders
4. User describes task in chat ("organize my downloads by file type")
5. Agent creates plan, shows user what it will do
6. User approves (or modifies)
7. Agent executes, streaming progress
8. Results displayed, conversation continues

---

## Technical Approach

**Feasibility**: HIGH - All components are proven, open-source, well-documented

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                    │
│  ┌─────────────────────────────────────────────────┐   │
│  │              React Frontend (UI)                │   │
│  │  - Chat interface                               │   │
│  │  - Settings panel                               │   │
│  │  - Folder permission manager                    │   │
│  │  - Execution approval dialogs                   │   │
│  └─────────────────────────────────────────────────┘   │
│                         │ IPC                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │              Rust Backend (Tauri)               │   │
│  │  - File system operations (sandboxed)           │   │
│  │  - Terminal execution (sandboxed)               │   │
│  │  - Model management                             │   │
│  │  - llama.cpp bindings (llama-cpp-rs)            │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                              │
│  ┌─────────────────────────────────────────────────┐   │
│  │              Agent Layer (Rust or Python)       │   │
│  │  - Planning & reasoning                         │   │
│  │  - Tool orchestration                           │   │
│  │  - Conversation management                      │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                         │
         ┌───────────────┴───────────────┐
         ▼                               ▼
┌─────────────────┐            ┌─────────────────┐
│   llama.cpp     │            │  User's Files   │
│   (embedded)    │            │  (sandboxed)    │
│                 │            │                 │
│  GGUF models    │            │  Granted        │
│  ~150 tok/s     │            │  folders only   │
└─────────────────┘            └─────────────────┘
```

### Key Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Desktop framework | Tauri | ~10MB bundle vs Electron's 150MB+, Rust safety, cross-platform |
| LLM inference | llama.cpp (embedded via llama-cpp-rs) | Proven, ~150 tok/s, CPU+GPU, GGUF ecosystem |
| Agent framework | Custom LangGraph-style or Rust-native | Avoid Python dependency for simpler distribution |
| Model format | GGUF | Industry standard, quantization options, broad model support |
| Sandboxing | Folder permission model | Simple, understandable, cross-platform compatible |
| IPC | Tauri's built-in IPC | Type-safe, performant |

### Technical Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| llama-cpp-rs bindings issues | Medium | Fallback to spawning llama.cpp binary as subprocess |
| Model download failures (large files) | Medium | Resumable downloads, multiple mirror sources, progress UI |
| Cross-platform terminal differences | Medium | Abstract terminal layer, platform-specific adapters |
| Agent reasoning quality with small models | Medium | Careful prompt engineering, structured output, fallback behaviors |
| Memory pressure on 8GB machines | Low | Aggressive model quantization (Q4_K_S), clear hardware requirements |

---

## Implementation Phases

<!--
  STATUS: pending | in-progress | complete
  PARALLEL: phases that can run concurrently (e.g., "with 3" or "-")
  DEPENDS: phases that must complete first (e.g., "1, 2" or "-")
  PRP: link to generated plan file once created
-->

| # | Phase | Description | Status | Parallel | Depends | PRP Plan |
|---|-------|-------------|--------|----------|---------|----------|
| 1 | Foundation | Tauri app scaffold, basic UI shell, project structure | complete | - | - | [phase-1-foundation.plan.md](../plans/completed/phase-1-foundation.plan.md) |
| 2 | LLM Integration | llama.cpp integration, model download, basic chat | complete | - | 1 | [phase-2-llm-integration.plan.md](../plans/completed/phase-2-llm-integration.plan.md) |
| 3 | File Operations | Sandboxed file system access, folder permissions | in-progress | with 4 | 2 | [phase-3-file-operations.plan.md](../plans/phase-3-file-operations.plan.md) |
| 4 | Terminal Execution | Sandboxed terminal with approval flow | pending | with 3 | 2 | - |
| 5 | Agent Core | Planning, tool orchestration, conversation management | pending | - | 3, 4 | - |
| 6 | Polish & Packaging | Installers, auto-update, first-run experience | pending | - | 5 | - |
| 7 | Web Browsing | Text-based fetch + parse, accessibility tree | pending | - | 5 | - |
| 8 | Skills System | Loadable skill modules, skill discovery | pending | - | 5 | - |
| 9 | MCP Integration | MCP server connections, tool exposure | pending | - | 5 | - |

### Phase Details

**Phase 1: Foundation**
- **Goal**: Runnable Tauri app with basic UI structure
- **Scope**:
  - Tauri + React project setup
  - Basic chat UI (input, message list, sidebar)
  - Settings panel skeleton
  - CI/CD pipeline for all three platforms
- **Success signal**: App builds and runs on Windows, macOS, Linux

**Phase 2: LLM Integration**
- **Goal**: Chat with local LLM works end-to-end
- **Scope**:
  - llama-cpp-rs integration (or subprocess fallback)
  - Model download manager with progress UI
  - Default model selection and bundling strategy
  - Basic inference (prompt → response streaming)
  - Model settings (context length, temperature)
- **Success signal**: User can chat with local model, see streaming responses

**Phase 3: File Operations**
- **Goal**: Agent can safely read/write files in granted folders
- **Scope**:
  - Folder permission grant UI
  - File operation tools (read, write, create, delete, move, list)
  - Permission enforcement layer
  - File operation confirmation dialogs
- **Success signal**: "Organize files in my Downloads folder by type" works

**Phase 4: Terminal Execution**
- **Goal**: Agent can run terminal commands with user approval
- **Scope**:
  - Terminal execution abstraction (cross-platform)
  - Command approval UI (show command, explain risk)
  - Output capture and display
  - Working directory management
- **Success signal**: "Run npm install in my project folder" works with approval

**Phase 5: Agent Core**
- **Goal**: Intelligent multi-step task execution
- **Scope**:
  - Planning layer (break task into steps)
  - Tool selection and orchestration
  - Error handling and recovery
  - Conversation context management
  - Structured output parsing from LLM
- **Success signal**: Multi-step tasks complete autonomously with minimal user intervention

**Phase 6: Polish & Packaging**
- **Goal**: Production-ready distribution
- **Scope**:
  - Platform installers (MSI, DMG, AppImage/deb)
  - Auto-update mechanism
  - First-run onboarding flow
  - Error reporting (opt-in)
  - Performance optimization
- **Success signal**: Non-technical user can install and use within 5 minutes

**Phase 7: Web Browsing**
- **Goal**: Agent can fetch and understand web content
- **Scope**:
  - HTTP fetch with HTML parsing
  - Text extraction from web pages
  - Accessibility tree parsing (like Playwright snapshot)
  - URL validation and safety checks
- **Success signal**: "Summarize the content at this URL" works

**Phase 8: Skills System**
- **Goal**: Extensible capability modules
- **Scope**:
  - Skill definition format (SKILL.md + assets)
  - Skill discovery and loading
  - Built-in starter skills
  - Skill marketplace/repository (future)
- **Success signal**: User can add a skill that teaches agent new workflows

**Phase 9: MCP Integration**
- **Goal**: Connect to external tools via MCP
- **Scope**:
  - MCP client implementation
  - Server discovery and connection UI
  - Tool exposure to agent
  - Security model for MCP tools
- **Success signal**: Agent can use tools from connected MCP servers

### Parallelism Notes

Phases 3 (File Operations) and 4 (Terminal Execution) can run in parallel as they touch different domains—file system vs. process execution. Both depend on Phase 2's working LLM integration to test end-to-end.

Phases 7, 8, and 9 are independent post-core features that could be developed in parallel by different contributors once Phase 5 is complete.

---

## Decisions Log

| Decision | Choice | Alternatives | Rationale |
|----------|--------|--------------|-----------|
| License | MIT | Apache 2.0, AGPL | Maximum adoption, matches dependencies (llama.cpp, Tauri) |
| Desktop framework | Tauri | Electron, Qt, Flutter | Smallest bundle, Rust safety, modern |
| LLM backend | llama.cpp | Ollama, vLLM, MLX | Best cross-platform performance on consumer hardware |
| Agent framework | Custom (Rust/LangGraph-style) | DeepAgents, LangChain | Avoid Python dependency for cleaner distribution |
| Sandboxing | Folder permissions | VM, containers, OS sandbox | Simplest cross-platform approach for v1 |
| Model format | GGUF | SafeTensors, ONNX | Largest ecosystem, quantization flexibility |
| Web browsing approach | Text-based first | Visual/multimodal | Works on all hardware, no multimodal requirement |
| Default model | Qwen3-4B-Instruct (Q4_K_M) | xLAM-3B, Llama 3.2 3B, Phi-3 | Best balance of tool calling + instruction following + writing quality; 262K context; fits 8GB RAM (~2.5GB GGUF) |

---

## Research Summary

**Market Context**
- Anthropic Cowork: $100-200/month, Mac-only, cloud-based, launched Jan 2025
- No direct competitor offers: local + GUI + agentic + non-technical UX
- Existing tools (Cline, Aider, OpenHands) target developers
- 60% of developers plan to use local LLM tools (Ollama specifically)

**Technical Context**
- llama.cpp: ~150 tok/s on consumer hardware, proven, cross-platform
- Tauri: Modern Rust-based desktop framework, ~10MB bundles
- Qwen3-4B-Instruct: Best small model for tool calling + instruction following + writing; 262K context; ~2.5GB GGUF (Q4_K_M)
- xLAM models: Purpose-built for function calling but weaker at general tasks; good fallback for low-end hardware
- LangChain DeepAgents: Reference architecture for planning + tools + skills

**Competitive Landscape**

| Tool | Local LLM | Desktop GUI | Non-technical UX | File/Terminal |
|------|-----------|-------------|------------------|---------------|
| Cowork | No | Yes | Yes | Yes |
| Cline | Yes | No (VS Code) | No | Yes |
| Aider | Yes | No (CLI) | No | Yes |
| OpenHands | Yes | Web | Partial | Yes |
| **LocalWork Hero** | Yes | Yes | Yes | Yes |

---

*Generated: 2025-01-15*
*Status: DRAFT - needs validation*
