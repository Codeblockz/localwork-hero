# Feature: Phase 2 - LLM Integration

## Summary

Integrate llama.cpp into LocalWork Hero to enable local LLM inference with model download management and basic chat functionality. Users will be able to download GGUF models, select which model to use, and have streaming conversations with a local AI assistant—all running entirely on their hardware without any data leaving their machine.

## User Story

As a non-technical user
I want to chat with a local AI assistant that runs on my own computer
So that I can get help with tasks privately without paying for cloud subscriptions

## Problem Statement

The application currently has a chat UI shell but no actual AI capability. Users see a welcome message but cannot interact with any model. Phase 2 bridges this gap by integrating llama.cpp for local inference, implementing a model download manager with progress UI, and connecting the chat interface to actually communicate with the LLM.

## Solution Statement

Use llama-cpp-2 Rust bindings to embed llama.cpp directly into the Tauri backend. Implement Tauri commands for model management (list, download, delete) and chat inference (send message, stream response). Update the React frontend to display real messages, show loading states during inference, and provide model settings in the SettingsPanel.

## Metadata

| Field            | Value                                                |
| ---------------- | ---------------------------------------------------- |
| Type             | NEW_CAPABILITY                                       |
| Complexity       | HIGH                                                 |
| Systems Affected | src-tauri/src/, src/components/chat/, src/lib/tauri.ts, src/App.tsx |
| Dependencies     | llama-cpp-2 (Rust), hf-hub (Rust), tokio (Rust)      |
| Estimated Tasks  | 15                                                   |

---

## UX Design

### Before State

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                              BEFORE STATE                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌───────────┐    ┌─────────────────────────────────┐    ┌───────────────┐  ║
║   │  Sidebar  │    │          Chat Area              │    │   Settings    │  ║
║   │           │    │                                 │    │    Panel      │  ║
║   │ [New Chat]│    │   "Welcome to LocalWork Hero!"  │    │               │  ║
║   │           │    │   "Your local AI assistant..."  │    │  Model:       │  ║
║   │ No chats  │    │                                 │    │  "Coming in   │  ║
║   │           │    │       (static placeholder)      │    │   Phase 2"    │  ║
║   │           │    │                                 │    │               │  ║
║   │ [Settings]│    │  ┌───────────────────────────┐  │    │  Folders:     │  ║
║   │           │    │  │ Type message... [Send]    │  │    │  "Phase 3"    │  ║
║   └───────────┘    └──┴───────────────────────────┴──┘    └───────────────┘  ║
║                                                                               ║
║   USER_FLOW: User types message → logs to console → nothing happens          ║
║   PAIN_POINT: No AI model, no responses, chat is non-functional              ║
║   DATA_FLOW: message → console.log("Message:", message) → void               ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### After State

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                               AFTER STATE                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌───────────┐    ┌─────────────────────────────────┐    ┌───────────────┐  ║
║   │  Sidebar  │    │          Chat Area              │    │   Settings    │  ║
║   │           │    │                                 │    │    Panel      │  ║
║   │ [New Chat]│    │  User: How do I organize files? │    │               │  ║
║   │           │    │                                 │    │  Model:       │  ║
║   │ Chat 1    │    │  Assistant: I can help you...   │    │  [Qwen3-4B ▼] │  ║
║   │           │    │  ████████░░ (streaming...)      │    │               │  ║
║   │           │    │                                 │    │  [Download    │  ║
║   │           │    │                                 │    │   Model]      │  ║
║   │ [Settings]│    │  ┌───────────────────────────┐  │    │  ▓▓▓▓░░ 45%   │  ║
║   │           │    │  │ Type message... [Send]    │  │    │               │  ║
║   └───────────┘    └──┴───────────────────────────┴──┘    └───────────────┘  ║
║                                                                               ║
║   USER_FLOW: User types → sends to LLM → streaming response → displayed      ║
║   VALUE_ADD: Real AI conversations, model selection, download with progress  ║
║   DATA_FLOW: message → Tauri IPC → llama.cpp → streaming tokens → UI         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝

╔═══════════════════════════════════════════════════════════════════════════════╗
║                          MODEL DOWNLOAD FLOW                                   ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌─────────────────────────────────────────────────────────────────────────┐ ║
║   │                        Settings Panel                                    │ ║
║   │  ┌───────────────────────────────────────────────────────────────────┐  │ ║
║   │  │  Model: [Select a model ▼]                                        │  │ ║
║   │  │                                                                   │  │ ║
║   │  │  Available Models:                                                │  │ ║
║   │  │  ┌─────────────────────────────────────────────────────────────┐ │  │ ║
║   │  │  │ ○ Qwen3-4B-Instruct (Q4_K_M)          2.5 GB   [Download]  │ │  │ ║
║   │  │  │   Downloading...  ▓▓▓▓▓▓▓░░░░░  1.2GB / 2.5GB             │ │  │ ║
║   │  │  ├─────────────────────────────────────────────────────────────┤ │  │ ║
║   │  │  │ ● Llama-3.2-3B-Instruct (Q4_K_M)     1.8 GB   ✓ Ready      │ │  │ ║
║   │  │  └─────────────────────────────────────────────────────────────┘ │  │ ║
║   │  │                                                                   │  │ ║
║   │  │  Custom Model: [Load GGUF file...]                               │  │ ║
║   │  └───────────────────────────────────────────────────────────────────┘  │ ║
║   └─────────────────────────────────────────────────────────────────────────┘ ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Interaction Changes

| Location | Before | After | User Impact |
|----------|--------|-------|-------------|
| `ChatInput.tsx` | Logs message to console | Sends to LLM via Tauri | Messages get AI responses |
| `MessageList.tsx` | Static welcome text | Dynamic message list with streaming | See conversation history |
| `SettingsPanel.tsx` | Placeholder text | Model selector, download button, progress bar | Can manage models |
| `App.tsx` | Only settings state | Messages, loading, models, selectedModel states | Full chat state management |
| Chat experience | Non-functional | Full streaming chat | Complete AI assistant |

---

## Mandatory Reading

**CRITICAL: Implementation agent MUST read these files before starting any task:**

| Priority | File | Lines | Why Read This |
|----------|------|-------|---------------|
| P0 | `src-tauri/src/lib.rs` | 1-22 | Pattern for Tauri commands and registration |
| P0 | `src/lib/tauri.ts` | 1-10 | Pattern for TypeScript command wrappers |
| P1 | `src/App.tsx` | 1-27 | Current state management pattern |
| P1 | `src/components/chat/ChatInput.tsx` | 1-30 | Chat input pattern to extend |
| P1 | `src/components/chat/MessageList.tsx` | 1-18 | Message display pattern to extend |
| P1 | `src/components/layout/SettingsPanel.tsx` | 1-38 | Settings UI pattern to extend |
| P2 | `src/components/ui/button.tsx` | 1-50 | shadcn/ui Button usage |
| P2 | `src-tauri/Cargo.toml` | 1-20 | Current Rust dependencies |

**External Documentation:**

| Source | Section | Why Needed |
|--------|---------|------------|
| [llama-cpp-2 docs](https://docs.rs/llama-cpp-2) | Model loading, inference | Core inference API |
| [hf-hub crate](https://docs.rs/hf-hub) | Model download | HuggingFace API for downloads |
| [Tauri v2 Commands](https://v2.tauri.app/develop/calling-rust/) | Commands, Events | IPC patterns for streaming |
| [Tauri v2 Events](https://v2.tauri.app/develop/calling-frontend/) | Event emission | Backend-to-frontend streaming |

---

## Patterns to Mirror

**TAURI_COMMAND_PATTERN:**
```rust
// SOURCE: src-tauri/src/lib.rs:1-13
// COPY THIS PATTERN:
#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    version: String,
}

#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo {
        name: "LocalWork Hero".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}
```

**COMMAND_REGISTRATION_PATTERN:**
```rust
// SOURCE: src-tauri/src/lib.rs:15-22
// COPY THIS PATTERN:
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**TYPESCRIPT_WRAPPER_PATTERN:**
```typescript
// SOURCE: src/lib/tauri.ts:1-10
// COPY THIS PATTERN:
import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  name: string;
  version: string;
}

export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("get_app_info");
}
```

**COMPONENT_PROPS_PATTERN:**
```typescript
// SOURCE: src/components/layout/SettingsPanel.tsx:5-8
// COPY THIS PATTERN:
interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}
```

**REACT_STATE_PATTERN:**
```typescript
// SOURCE: src/App.tsx:7-8
// COPY THIS PATTERN:
const [isSettingsOpen, setIsSettingsOpen] = useState(false);
```

**FORM_SUBMIT_PATTERN:**
```typescript
// SOURCE: src/components/chat/ChatInput.tsx:9-15
// COPY THIS PATTERN:
const handleSubmit = (e: React.FormEvent) => {
  e.preventDefault();
  if (!message.trim()) return;
  // Process message
  setMessage("");
};
```

---

## Files to Change

| File | Action | Justification |
| ---- | ------ | ------------- |
| `src-tauri/Cargo.toml` | UPDATE | Add llama-cpp-2, hf-hub, tokio, dirs dependencies |
| `src-tauri/src/lib.rs` | UPDATE | Add model and chat Tauri commands |
| `src-tauri/src/models/mod.rs` | CREATE | Model management module |
| `src-tauri/src/models/download.rs` | CREATE | Model download with progress |
| `src-tauri/src/models/types.rs` | CREATE | Model data types |
| `src-tauri/src/inference/mod.rs` | CREATE | Inference orchestration |
| `src-tauri/src/inference/llama.rs` | CREATE | llama.cpp wrapper |
| `src/lib/tauri.ts` | UPDATE | Add model and chat command wrappers |
| `src/App.tsx` | UPDATE | Add message state, model state, chat handlers |
| `src/components/chat/ChatInput.tsx` | UPDATE | Add onSend callback, loading state |
| `src/components/chat/MessageList.tsx` | UPDATE | Render message array, streaming indicator |
| `src/components/chat/ChatArea.tsx` | UPDATE | Pass messages and handlers to children |
| `src/components/layout/SettingsPanel.tsx` | UPDATE | Add model selector, download UI |
| `src/components/ui/progress.tsx` | CREATE | Progress bar component (shadcn) |
| `src/components/ui/select.tsx` | CREATE | Select dropdown component (shadcn) |

---

## NOT Building (Scope Limits)

Explicit exclusions to prevent scope creep:

- **Conversation history persistence** - Will be in-memory only; persistence is future work
- **Multiple concurrent chats** - Single conversation at a time; sidebar "New Chat" clears messages
- **Model fine-tuning** - Only inference; no training capabilities
- **GPU acceleration setup** - CPU-only initially; GPU can be added as enhancement
- **Streaming token-by-token UI** - Initial version accumulates response then displays; true streaming is enhancement
- **Context window management** - No conversation pruning; will just error on overflow
- **Custom system prompts** - Fixed system prompt; customization is future work
- **Model parameter tuning** - No temperature/top_p sliders; default parameters only
- **Model deletion** - Can only add models; deletion is future work

---

## Step-by-Step Tasks

Execute in order. Each task is atomic and independently verifiable.

### Task 1: UPDATE `src-tauri/Cargo.toml` - Add dependencies

- **ACTION**: ADD Rust dependencies for LLM integration
- **IMPLEMENT**: Add llama-cpp-2, hf-hub, tokio, dirs, thiserror crates
- **MIRROR**: Follow existing dependency format in `Cargo.toml:15-19`
- **ADD**:
  ```toml
  llama-cpp-2 = "0.1"
  hf-hub = { version = "0.3", features = ["tokio"] }
  tokio = { version = "1", features = ["rt-multi-thread", "sync"] }
  dirs = "5"
  thiserror = "1"
  ```
- **GOTCHA**: llama-cpp-2 requires clang installed for build; document in README
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 2: CREATE `src-tauri/src/models/types.rs` - Model data types

- **ACTION**: CREATE type definitions for models
- **IMPLEMENT**: ModelInfo struct with fields: id, name, size, path, downloaded
- **PATTERN**: Use `#[derive(serde::Serialize, serde::Deserialize, Clone)]`
- **TYPES**:
  ```rust
  pub struct ModelInfo {
      pub id: String,
      pub name: String,
      pub size_bytes: u64,
      pub path: Option<String>,
      pub downloaded: bool,
  }

  pub struct DownloadProgress {
      pub model_id: String,
      pub bytes_downloaded: u64,
      pub total_bytes: u64,
      pub percent: f32,
  }
  ```
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 3: CREATE `src-tauri/src/models/mod.rs` - Model module

- **ACTION**: CREATE module root with re-exports
- **IMPLEMENT**: Module declarations for download and types
- **CONTENT**:
  ```rust
  pub mod download;
  pub mod types;

  pub use types::{ModelInfo, DownloadProgress};
  ```
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 4: CREATE `src-tauri/src/models/download.rs` - Download logic

- **ACTION**: CREATE model download functionality
- **IMPLEMENT**: Functions to download GGUF models from HuggingFace
- **IMPORTS**: `use hf_hub::api::sync::ApiBuilder;`
- **PATTERN**: Use hf-hub with progress tracking, save to local models directory
- **FUNCTIONS**:
  - `get_models_dir() -> PathBuf` - Returns `~/.localwork-hero/models`
  - `download_model(model_id: &str, filename: &str) -> Result<PathBuf, Error>`
  - `list_downloaded_models() -> Vec<ModelInfo>`
- **GOTCHA**: Use `dirs::data_local_dir()` for cross-platform path
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 5: CREATE `src-tauri/src/inference/mod.rs` - Inference module

- **ACTION**: CREATE inference module root
- **IMPLEMENT**: Module declarations for llama
- **CONTENT**:
  ```rust
  pub mod llama;

  pub use llama::LlamaInference;
  ```
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 6: CREATE `src-tauri/src/inference/llama.rs` - llama.cpp wrapper

- **ACTION**: CREATE llama.cpp inference wrapper
- **IMPLEMENT**: Struct wrapping llama-cpp-2 for text generation
- **IMPORTS**:
  ```rust
  use llama_cpp_2::llama_backend::LlamaBackend;
  use llama_cpp_2::model::{LlamaModel, AddBos, Special};
  use llama_cpp_2::model::params::LlamaModelParams;
  use llama_cpp_2::context::params::LlamaContextParams;
  use llama_cpp_2::llama_batch::LlamaBatch;
  use llama_cpp_2::sampling::LlamaSampler;
  ```
- **PATTERN**: Initialize backend once, load model, create context, run inference
- **FUNCTIONS**:
  - `LlamaInference::new() -> Result<Self, Error>` - Initialize backend
  - `load_model(&mut self, path: &Path) -> Result<(), Error>`
  - `generate(&mut self, prompt: &str, max_tokens: u32) -> Result<String, Error>`
- **GOTCHA**: Backend must be initialized before model loading; keep backend alive
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 7: UPDATE `src-tauri/src/lib.rs` - Add Tauri commands

- **ACTION**: UPDATE to add model and chat commands
- **IMPLEMENT**: New commands for list_models, download_model, send_message
- **IMPORTS**: Add module declarations at top
- **ADD**:
  ```rust
  mod models;
  mod inference;

  use std::sync::Mutex;
  use tauri::State;

  struct AppState {
      inference: Mutex<Option<inference::LlamaInference>>,
  }
  ```
- **COMMANDS**:
  - `list_models() -> Vec<ModelInfo>`
  - `download_model(model_id: String) -> Result<(), String>`
  - `load_model(model_path: String) -> Result<(), String>`
  - `send_message(prompt: String) -> Result<String, String>`
- **REGISTER**: Add all commands to `generate_handler![]`
- **GOTCHA**: Use `State<AppState>` for shared inference instance
- **VALIDATE**: `cd src-tauri && cargo check`

### Task 8: UPDATE `src/lib/tauri.ts` - Add TypeScript wrappers

- **ACTION**: UPDATE to add model and chat command wrappers
- **IMPLEMENT**: TypeScript interfaces and async functions
- **MIRROR**: `src/lib/tauri.ts:1-10` for function style
- **ADD**:
  ```typescript
  export interface ModelInfo {
    id: string;
    name: string;
    size_bytes: number;
    path: string | null;
    downloaded: boolean;
  }

  export interface Message {
    role: "user" | "assistant";
    content: string;
  }

  export async function listModels(): Promise<ModelInfo[]> {
    return invoke<ModelInfo[]>("list_models");
  }

  export async function downloadModel(modelId: string): Promise<void> {
    return invoke<void>("download_model", { modelId });
  }

  export async function loadModel(modelPath: string): Promise<void> {
    return invoke<void>("load_model", { modelPath });
  }

  export async function sendMessage(prompt: string): Promise<string> {
    return invoke<string>("send_message", { prompt });
  }
  ```
- **VALIDATE**: `pnpm run typecheck`

### Task 9: CREATE `src/components/ui/progress.tsx` - Progress bar

- **ACTION**: CREATE shadcn/ui Progress component
- **IMPLEMENT**: Install via shadcn CLI or create manually
- **COMMAND**: `npx shadcn@latest add progress`
- **GOTCHA**: If shadcn CLI fails, create manually following existing ui component patterns
- **VALIDATE**: `pnpm run typecheck`

### Task 10: CREATE `src/components/ui/select.tsx` - Select dropdown

- **ACTION**: CREATE shadcn/ui Select component
- **IMPLEMENT**: Install via shadcn CLI or create manually
- **COMMAND**: `npx shadcn@latest add select`
- **GOTCHA**: Requires @radix-ui/react-select dependency
- **VALIDATE**: `pnpm run typecheck`

### Task 11: UPDATE `src/components/chat/MessageList.tsx` - Render messages

- **ACTION**: UPDATE to accept and render message array
- **IMPLEMENT**: Map messages to UI, show user vs assistant styles, streaming indicator
- **PROPS**: `messages: Message[]`, `isLoading: boolean`
- **PATTERN**: User messages right-aligned, assistant left-aligned
- **UI**:
  ```typescript
  interface MessageListProps {
    messages: Message[];
    isLoading: boolean;
  }

  // Show welcome when empty, map messages when populated
  // Show "Thinking..." indicator when isLoading
  ```
- **MIRROR**: Use ScrollArea pattern from existing component
- **VALIDATE**: `pnpm run typecheck`

### Task 12: UPDATE `src/components/chat/ChatInput.tsx` - Add send callback

- **ACTION**: UPDATE to call parent onSend handler
- **IMPLEMENT**: Add onSend prop, disabled state during loading
- **PROPS**: `onSend: (message: string) => void`, `disabled: boolean`
- **PATTERN**: Call onSend in handleSubmit, disable button when disabled prop true
- **UI**: Show loading spinner on button when disabled
- **MIRROR**: `src/components/chat/ChatInput.tsx:9-15` for submit pattern
- **VALIDATE**: `pnpm run typecheck`

### Task 13: UPDATE `src/components/chat/ChatArea.tsx` - Wire up chat

- **ACTION**: UPDATE to pass messages and handlers through
- **IMPLEMENT**: Accept props from parent, pass to children
- **PROPS**: `messages: Message[]`, `isLoading: boolean`, `onSend: (msg: string) => void`
- **PATTERN**: Pass-through component connecting App state to chat UI
- **VALIDATE**: `pnpm run typecheck`

### Task 14: UPDATE `src/components/layout/SettingsPanel.tsx` - Model UI

- **ACTION**: UPDATE to add model management UI
- **IMPLEMENT**: Model selector dropdown, download button, progress indicator
- **PROPS**: Add `models: ModelInfo[]`, `selectedModel: string`, `onSelectModel: (id: string) => void`, `onDownloadModel: (id: string) => void`, `downloadProgress: number | null`
- **UI**:
  - Show list of available models with download status
  - Download button for undownloaded models
  - Progress bar during download
  - Radio/select for choosing active model
- **IMPORTS**: Add Select, Progress, Button components
- **VALIDATE**: `pnpm run typecheck`

### Task 15: UPDATE `src/App.tsx` - Integrate all state

- **ACTION**: UPDATE to manage all chat and model state
- **IMPLEMENT**: Add state for messages, models, selectedModel, isLoading, downloadProgress
- **STATE**:
  ```typescript
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [selectedModel, setSelectedModel] = useState<string>("");
  const [downloadProgress, setDownloadProgress] = useState<number | null>(null);
  ```
- **HANDLERS**:
  - `handleSendMessage` - Add user message, call sendMessage, add response
  - `handleDownloadModel` - Call downloadModel, update progress
  - `handleSelectModel` - Call loadModel, update selectedModel
- **USEEFFECT**: On mount, call listModels to populate models state
- **PATTERN**: Lift state to App, pass down to ChatArea and SettingsPanel
- **VALIDATE**: `pnpm run typecheck`

---

## Testing Strategy

### Manual Testing Checklist

| Test Case | Steps | Expected Result |
|-----------|-------|-----------------|
| Model list loads | Open app, open settings | See available models |
| Model downloads | Click download on model | Progress bar shows, completes |
| Model loads | Select downloaded model | No error, ready to chat |
| Send message | Type message, click send | Message appears in chat |
| Receive response | After send | AI response appears |
| Loading state | Send message | Input disabled, "Thinking..." shows |
| Empty message | Click send with empty input | Nothing happens |
| Error handling | Send without model loaded | Error message displayed |

### Edge Cases Checklist

- [ ] Empty message submission (should be blocked)
- [ ] Message while loading (should be blocked)
- [ ] Model download cancellation (future - not in scope)
- [ ] Very long messages (should work up to context limit)
- [ ] Network failure during download (should show error)
- [ ] Invalid model path (should show error)
- [ ] No models downloaded (should prompt to download)

---

## Validation Commands

### Level 1: STATIC_ANALYSIS

```bash
pnpm run typecheck && cd src-tauri && cargo check
```

**EXPECT**: Exit 0, no TypeScript or Rust errors

### Level 2: BUILD

```bash
pnpm tauri build
```

**EXPECT**: Builds successfully for current platform

### Level 3: RUNTIME

```bash
pnpm tauri dev
```

**EXPECT**: App launches, UI renders, no console errors

### Level 4: MANUAL_VALIDATION

1. Open app
2. Open Settings panel
3. Verify model list shows available models
4. Download a small test model (if network available)
5. Select the model
6. Send a test message "Hello, how are you?"
7. Verify response appears
8. Verify loading states work correctly

---

## Acceptance Criteria

- [ ] App compiles and runs without errors
- [ ] Settings panel shows available models
- [ ] User can download a model with progress indicator
- [ ] User can select a downloaded model
- [ ] User can send a message and receive a response
- [ ] Loading state shows while waiting for response
- [ ] Messages display in chat with user/assistant distinction
- [ ] Error states display appropriately

---

## Completion Checklist

- [ ] All 15 tasks completed in dependency order
- [ ] Each task validated immediately after completion
- [ ] Level 1: Static analysis passes (`pnpm run typecheck && cargo check`)
- [ ] Level 2: Build succeeds (`pnpm tauri build`)
- [ ] Level 3: Runtime works (`pnpm tauri dev`)
- [ ] Level 4: Manual validation passes
- [ ] All acceptance criteria met

---

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
| ---- | ---------- | ------ | ---------- |
| llama-cpp-2 build issues (clang) | MEDIUM | HIGH | Document clang requirement; test on all platforms |
| Model download failures | MEDIUM | MEDIUM | Add retry logic; clear error messages |
| Memory pressure with large models | LOW | MEDIUM | Start with small model (3-4B); document requirements |
| llama-cpp-2 API changes | LOW | HIGH | Pin specific version in Cargo.toml |
| HuggingFace rate limiting | LOW | LOW | Use hf-hub caching; add backoff |

---

## Notes

**Default Model Strategy:**
Per PRD, default to Qwen3-4B-Instruct (Q4_K_M, ~2.5GB). For initial implementation, hardcode a list of recommended models rather than querying HuggingFace dynamically.

**Hardcoded Model List:**
```rust
vec![
    ModelInfo {
        id: "Qwen/Qwen3-4B-Instruct-GGUF".to_string(),
        name: "Qwen3 4B Instruct".to_string(),
        filename: "qwen3-4b-instruct-q4_k_m.gguf".to_string(),
        size_bytes: 2_500_000_000,
        downloaded: false,
        path: None,
    },
]
```

**Streaming Consideration:**
Initial implementation will wait for full response before displaying. True token-by-token streaming requires Tauri events and is more complex. Can be added as enhancement after basic chat works.

**State Management:**
Using React useState at App level is sufficient for MVP. If state grows complex, consider React Context or Zustand in future phases.

**Research Sources:**
- [llama-cpp-2 GitHub](https://github.com/utilityai/llama-cpp-rs) - Rust bindings documentation
- [hf-hub crate](https://docs.rs/hf-hub) - HuggingFace model downloads
- [Tauri v2 Commands](https://v2.tauri.app/develop/calling-rust/) - IPC documentation
- [tauri-local-lm](https://github.com/dillondesilva/tauri-local-lm) - Reference implementation
