# Feature: Phase 1 - Foundation

## Summary

Scaffold a production-ready Tauri v2 desktop application with React 19 frontend, TypeScript, Vite, Tailwind CSS, and shadcn/ui components. This phase establishes the project structure, basic UI shell (chat interface skeleton, sidebar, settings panel), and CI/CD pipeline for cross-platform builds on Windows, macOS, and Linux.

## User Story

As a **developer setting up LocalWork Hero**
I want to **have a runnable Tauri app with a modern React UI and CI/CD pipeline**
So that **subsequent phases can build on a solid, tested foundation**

## Problem Statement

LocalWork Hero needs a cross-platform desktop application foundation that:
1. Runs on Windows, macOS, and Linux with minimal bundle size (~10MB)
2. Uses modern React patterns with type safety
3. Has a chat-focused UI ready for LLM integration
4. Includes automated builds for all platforms via GitHub Actions

## Solution Statement

Create a Tauri v2 + React 19 + Vite 6 application using `create-tauri-app` as the base, enhanced with:
- shadcn/ui components for a polished, accessible UI
- Tailwind CSS v4 for styling
- Three-panel layout (sidebar, chat, settings)
- GitHub Actions workflow for multi-platform CI/CD

## Metadata

| Field | Value |
|-------|-------|
| Type | NEW_CAPABILITY |
| Complexity | MEDIUM |
| Systems Affected | Frontend (React), Backend (Rust/Tauri), CI/CD (GitHub Actions) |
| Dependencies | Tauri v2, React 19, Vite 6, Tailwind CSS v4, shadcn/ui |
| Estimated Tasks | 12 |

---

## UX Design

### Before State
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                              BEFORE STATE                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌─────────────────────────────────────────────────────────────────────┐    ║
║   │                                                                     │    ║
║   │                         EMPTY PROJECT                               │    ║
║   │                                                                     │    ║
║   │           Only PRD document exists, no code                         │    ║
║   │                                                                     │    ║
║   └─────────────────────────────────────────────────────────────────────┘    ║
║                                                                               ║
║   USER_FLOW: N/A - project not yet created                                    ║
║   PAIN_POINT: Cannot develop features without a foundation                    ║
║   DATA_FLOW: N/A                                                              ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### After State
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                               AFTER STATE                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ┌────────────────────────────────────────────────────────────────────────┐  ║
║   │ LocalWork Hero                                            [_][□][X]   │  ║
║   ├──────────┬──────────────────────────────────────────────┬─────────────┤  ║
║   │          │                                              │             │  ║
║   │ SIDEBAR  │              CHAT AREA                       │  SETTINGS   │  ║
║   │          │                                              │   PANEL     │  ║
║   │ ┌──────┐ │  ┌────────────────────────────────────┐     │  (hidden    │  ║
║   │ │ + New│ │  │ Welcome to LocalWork Hero!         │     │   by        │  ║
║   │ │ Chat │ │  │                                    │     │   default)  │  ║
║   │ └──────┘ │  │ Your local AI assistant is ready   │     │             │  ║
║   │          │  │ to help with file tasks.           │     │             │  ║
║   │ History  │  └────────────────────────────────────┘     │             │  ║
║   │ ────────│  │                                     │     │             │  ║
║   │ (empty)  │  ├────────────────────────────────────┤     │             │  ║
║   │          │  │ [Type your message here...]   [➤] │     │             │  ║
║   │          │  └────────────────────────────────────┘     │             │  ║
║   │          │                                              │             │  ║
║   │ ⚙ Setngs│                                              │             │  ║
║   └──────────┴──────────────────────────────────────────────┴─────────────┘  ║
║                                                                               ║
║   USER_FLOW: Launch app → See chat interface → Type message (no response yet) ║
║   VALUE_ADD: Runnable app skeleton ready for LLM integration                  ║
║   DATA_FLOW: User input captured → Ready for Phase 2 LLM connection           ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Interaction Changes
| Location | Before | After | User Impact |
|----------|--------|-------|-------------|
| Desktop | No app | Installable .exe/.dmg/.AppImage | Can launch LocalWork Hero |
| Main Window | N/A | Chat interface with sidebar | Familiar chat UX pattern |
| Settings | N/A | Settings panel (skeleton) | Prepared for future configuration |
| CI/CD | N/A | Automated builds on push | Consistent cross-platform releases |

---

## Mandatory Reading

**CRITICAL: Implementation agent MUST read these files before starting any task:**

| Priority | File | Lines | Why Read This |
|----------|------|-------|---------------|
| P0 | `.claude/PRPs/prds/localwork-hero.prd.md` | 1-200 | Understand product vision, architecture decisions |

**External Documentation:**

| Source | Section | Why Needed |
|--------|---------|------------|
| [Tauri v2 - Create Project](https://v2.tauri.app/start/create-project/) | Quick Start | Project scaffolding command |
| [Tauri v2 - Configuration](https://v2.tauri.app/develop/configuration-files/) | tauri.conf.json | Configure app metadata, window settings |
| [Tauri v2 - Calling Rust](https://v2.tauri.app/develop/calling-rust/) | Commands | Frontend-backend IPC pattern |
| [Tauri v2 - GitHub Actions](https://v2.tauri.app/distribute/pipelines/github/) | Workflow | CI/CD setup for multi-platform builds |
| [shadcn/ui Installation](https://ui.shadcn.com/docs/installation/vite) | Vite setup | Component library integration |
| [Tailwind CSS v4](https://tailwindcss.com/docs/installation/vite) | Vite setup | Styling framework |

---

## Patterns to Mirror

**NAMING_CONVENTION:**
```
Since this is a greenfield project, establish these conventions:
- React components: PascalCase (ChatArea.tsx, Sidebar.tsx)
- Hooks: camelCase with "use" prefix (useChat.ts)
- Utilities: camelCase (formatMessage.ts)
- Rust commands: snake_case (get_app_info)
- CSS classes: Tailwind utility classes, no custom CSS unless necessary
```

**PROJECT_STRUCTURE:**
```
localwork-hero/
├── src/                          # React frontend
│   ├── components/               # UI components
│   │   ├── ui/                   # shadcn/ui components (auto-generated)
│   │   ├── chat/                 # Chat-specific components
│   │   │   ├── ChatArea.tsx
│   │   │   ├── ChatInput.tsx
│   │   │   └── MessageList.tsx
│   │   ├── layout/               # Layout components
│   │   │   ├── Sidebar.tsx
│   │   │   └── SettingsPanel.tsx
│   │   └── shared/               # Shared/common components
│   ├── hooks/                    # Custom React hooks
│   ├── lib/                      # Utilities and helpers
│   │   └── utils.ts              # shadcn cn() utility
│   ├── App.tsx                   # Main app component
│   ├── main.tsx                  # React entry point
│   └── index.css                 # Global styles + Tailwind
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri entry point
│   │   ├── lib.rs                # Library exports
│   │   └── commands/             # Tauri commands (future)
│   │       └── mod.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Tauri configuration
│   ├── capabilities/             # Tauri capabilities
│   └── icons/                    # App icons
├── public/                       # Static assets
├── .github/
│   └── workflows/
│       └── ci.yml                # CI/CD workflow
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.ts
├── components.json               # shadcn/ui config
└── README.md
```

**TAURI_COMMAND_PATTERN:**
```rust
// src-tauri/src/main.rs
#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo {
        name: "LocalWork Hero".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    version: String,
}
```

**FRONTEND_INVOKE_PATTERN:**
```typescript
// src/lib/tauri.ts
import { invoke } from '@tauri-apps/api/core';

interface AppInfo {
  name: string;
  version: string;
}

export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>('get_app_info');
}
```

---

## Files to Change

| File | Action | Justification |
|------|--------|---------------|
| `package.json` | CREATE | Project manifest with dependencies |
| `tsconfig.json` | CREATE | TypeScript configuration |
| `vite.config.ts` | CREATE | Vite bundler configuration |
| `tailwind.config.ts` | CREATE | Tailwind CSS configuration |
| `components.json` | CREATE | shadcn/ui configuration |
| `src/main.tsx` | CREATE | React entry point |
| `src/App.tsx` | CREATE | Main app component with layout |
| `src/index.css` | CREATE | Global styles + Tailwind imports |
| `src/lib/utils.ts` | CREATE | shadcn cn() utility |
| `src/components/ui/*.tsx` | CREATE | shadcn/ui base components |
| `src/components/chat/ChatArea.tsx` | CREATE | Main chat container |
| `src/components/chat/ChatInput.tsx` | CREATE | Message input component |
| `src/components/chat/MessageList.tsx` | CREATE | Message display component |
| `src/components/layout/Sidebar.tsx` | CREATE | Navigation sidebar |
| `src/components/layout/SettingsPanel.tsx` | CREATE | Settings panel skeleton |
| `src-tauri/Cargo.toml` | CREATE | Rust dependencies |
| `src-tauri/tauri.conf.json` | CREATE | Tauri app configuration |
| `src-tauri/src/main.rs` | CREATE | Rust entry point |
| `src-tauri/src/lib.rs` | CREATE | Library exports |
| `.github/workflows/ci.yml` | CREATE | CI/CD pipeline |
| `README.md` | CREATE | Project documentation |

---

## NOT Building (Scope Limits)

Explicit exclusions to prevent scope creep:

- **LLM integration** - Phase 2 scope; chat UI is a visual shell only
- **File operations** - Phase 3 scope; no file system access
- **Terminal execution** - Phase 4 scope; no shell access
- **Conversation persistence** - Phase 5 scope; messages are ephemeral
- **Settings functionality** - Settings panel is UI only, no persistence
- **Model download** - Phase 2 scope; no model management
- **Auto-update** - Phase 6 scope; no updater integration
- **Code signing** - Phase 6 scope; unsigned dev builds only

---

## Step-by-Step Tasks

Execute in order. Each task is atomic and independently verifiable.

### Task 1: Initialize Tauri v2 + React Project

- **ACTION**: Scaffold new project using create-tauri-app
- **IMPLEMENT**:
  ```bash
  pnpm create tauri-app localwork-hero --template react-ts --manager pnpm
  cd localwork-hero
  pnpm install
  ```
- **VERIFY**: Project structure created with `src/` and `src-tauri/` directories
- **GOTCHA**: Use pnpm for faster installs and better monorepo support
- **VALIDATE**: `pnpm tauri dev` - app window opens

### Task 2: Configure Tauri Application Metadata

- **ACTION**: Update tauri.conf.json with app identity
- **IMPLEMENT**:
  ```json
  {
    "productName": "LocalWork Hero",
    "identifier": "com.localworkhero.app",
    "version": "0.1.0",
    "app": {
      "windows": [
        {
          "title": "LocalWork Hero",
          "width": 1200,
          "height": 800,
          "minWidth": 800,
          "minHeight": 600,
          "center": true,
          "resizable": true
        }
      ]
    }
  }
  ```
- **MIRROR**: Tauri v2 config structure from docs
- **VALIDATE**: `pnpm tauri dev` - window shows correct title and dimensions

### Task 3: Install and Configure Tailwind CSS v4

- **ACTION**: Add Tailwind CSS with Vite integration
- **IMPLEMENT**:
  ```bash
  pnpm add -D tailwindcss @tailwindcss/vite
  ```
  Update `vite.config.ts`:
  ```typescript
  import tailwindcss from '@tailwindcss/vite'

  export default defineConfig({
    plugins: [react(), tailwindcss()],
  })
  ```
  Update `src/index.css`:
  ```css
  @import "tailwindcss";
  ```
- **GOTCHA**: Tailwind v4 uses `@import "tailwindcss"` not directives
- **VALIDATE**: Tailwind classes work in components

### Task 4: Install and Configure shadcn/ui

- **ACTION**: Initialize shadcn/ui component library
- **IMPLEMENT**:
  ```bash
  pnpm dlx shadcn@latest init
  ```
  Select: TypeScript, Default style, Slate base color, CSS variables

  Add essential components:
  ```bash
  pnpm dlx shadcn@latest add button input card scroll-area separator
  ```
- **MIRROR**: shadcn/ui Vite installation guide
- **GOTCHA**: Ensure `components.json` has correct `aliases.components` path
- **VALIDATE**: Import and render a Button component

### Task 5: Create Layout Component Structure

- **ACTION**: Create the three-panel layout (Sidebar, Chat, Settings)
- **IMPLEMENT**: Create `src/components/layout/Layout.tsx`:
  ```typescript
  interface LayoutProps {
    children: React.ReactNode;
  }

  export function Layout({ children }: LayoutProps) {
    return (
      <div className="flex h-screen bg-background">
        <Sidebar />
        <main className="flex-1 flex flex-col">{children}</main>
      </div>
    );
  }
  ```
- **FILES**:
  - `src/components/layout/Layout.tsx` - Main layout wrapper
  - `src/components/layout/Sidebar.tsx` - Left navigation
  - `src/components/layout/SettingsPanel.tsx` - Right panel (toggleable)
- **VALIDATE**: `pnpm tauri dev` - layout renders with sidebar visible

### Task 6: Create Sidebar Component

- **ACTION**: Build collapsible sidebar with navigation
- **IMPLEMENT**: `src/components/layout/Sidebar.tsx`:
  ```typescript
  export function Sidebar() {
    return (
      <aside className="w-64 border-r bg-muted/40 flex flex-col">
        <div className="p-4 border-b">
          <Button className="w-full" variant="outline">
            <Plus className="mr-2 h-4 w-4" />
            New Chat
          </Button>
        </div>
        <ScrollArea className="flex-1 p-4">
          <div className="text-sm text-muted-foreground">
            No conversations yet
          </div>
        </ScrollArea>
        <div className="p-4 border-t">
          <Button variant="ghost" className="w-full justify-start">
            <Settings className="mr-2 h-4 w-4" />
            Settings
          </Button>
        </div>
      </aside>
    );
  }
  ```
- **IMPORTS**: Button, ScrollArea from shadcn/ui; Plus, Settings from lucide-react
- **VALIDATE**: Sidebar renders with "New Chat" button and Settings link

### Task 7: Create Chat Area Components

- **ACTION**: Build chat message display and input components
- **IMPLEMENT**:
  - `src/components/chat/ChatArea.tsx` - Container with welcome message
  - `src/components/chat/MessageList.tsx` - Scrollable message container
  - `src/components/chat/ChatInput.tsx` - Text input with send button

  ChatInput.tsx:
  ```typescript
  export function ChatInput() {
    const [message, setMessage] = useState('');

    const handleSubmit = (e: React.FormEvent) => {
      e.preventDefault();
      // Phase 2 will handle actual submission
      console.log('Message:', message);
      setMessage('');
    };

    return (
      <form onSubmit={handleSubmit} className="flex gap-2 p-4 border-t">
        <Input
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="Type your message here..."
          className="flex-1"
        />
        <Button type="submit" size="icon">
          <Send className="h-4 w-4" />
        </Button>
      </form>
    );
  }
  ```
- **GOTCHA**: Messages are ephemeral in Phase 1, no persistence
- **VALIDATE**: Can type in input, submit clears field

### Task 8: Create Settings Panel Skeleton

- **ACTION**: Build toggleable settings panel
- **IMPLEMENT**: `src/components/layout/SettingsPanel.tsx`:
  ```typescript
  interface SettingsPanelProps {
    isOpen: boolean;
    onClose: () => void;
  }

  export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
    if (!isOpen) return null;

    return (
      <aside className="w-80 border-l bg-background p-6">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-lg font-semibold">Settings</h2>
          <Button variant="ghost" size="icon" onClick={onClose}>
            <X className="h-4 w-4" />
          </Button>
        </div>
        <div className="space-y-6">
          <section>
            <h3 className="font-medium mb-2">Model</h3>
            <p className="text-sm text-muted-foreground">
              Model settings will appear here in Phase 2
            </p>
          </section>
          <Separator />
          <section>
            <h3 className="font-medium mb-2">Folders</h3>
            <p className="text-sm text-muted-foreground">
              Folder permissions will appear here in Phase 3
            </p>
          </section>
        </div>
      </aside>
    );
  }
  ```
- **VALIDATE**: Settings panel toggles open/closed

### Task 9: Wire Up Main App Component

- **ACTION**: Integrate all components in App.tsx
- **IMPLEMENT**: `src/App.tsx`:
  ```typescript
  import { useState } from 'react';
  import { Layout } from './components/layout/Layout';
  import { ChatArea } from './components/chat/ChatArea';
  import { SettingsPanel } from './components/layout/SettingsPanel';

  function App() {
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);

    return (
      <Layout
        onSettingsClick={() => setIsSettingsOpen(true)}
        isSettingsOpen={isSettingsOpen}
      >
        <ChatArea />
        <SettingsPanel
          isOpen={isSettingsOpen}
          onClose={() => setIsSettingsOpen(false)}
        />
      </Layout>
    );
  }

  export default App;
  ```
- **VALIDATE**: Full app renders with all panels, settings toggles

### Task 10: Add Basic Rust Command for Version Info

- **ACTION**: Create Tauri command to return app info
- **IMPLEMENT**: `src-tauri/src/main.rs`:
  ```rust
  #[tauri::command]
  fn get_app_info() -> AppInfo {
      AppInfo {
          name: "LocalWork Hero".to_string(),
          version: env!("CARGO_PKG_VERSION").to_string(),
      }
  }

  #[derive(serde::Serialize)]
  struct AppInfo {
      name: String,
      version: String,
  }

  #[cfg_attr(mobile, tauri::mobile_entry_point)]
  pub fn run() {
      tauri::Builder::default()
          .plugin(tauri_plugin_opener::init())
          .invoke_handler(tauri::generate_handler![get_app_info])
          .run(tauri::generate_context!())
          .expect("error while running tauri application");
  }
  ```
- **VALIDATE**: `pnpm tauri dev` - no compilation errors

### Task 11: Create Frontend Tauri Bridge

- **ACTION**: Add TypeScript wrapper for Tauri commands
- **IMPLEMENT**: `src/lib/tauri.ts`:
  ```typescript
  import { invoke } from '@tauri-apps/api/core';

  export interface AppInfo {
    name: string;
    version: string;
  }

  export async function getAppInfo(): Promise<AppInfo> {
    return invoke<AppInfo>('get_app_info');
  }
  ```

  Display version in App:
  ```typescript
  useEffect(() => {
    getAppInfo().then(info => {
      console.log(`${info.name} v${info.version}`);
    });
  }, []);
  ```
- **VALIDATE**: Console shows "LocalWork Hero v0.1.0" on app load

### Task 12: Create GitHub Actions CI/CD Workflow

- **ACTION**: Set up multi-platform build pipeline
- **IMPLEMENT**: `.github/workflows/ci.yml`:
  ```yaml
  name: 'CI'

  on:
    push:
      branches: [main]
    pull_request:
      branches: [main]

  jobs:
    build:
      permissions:
        contents: write
      strategy:
        fail-fast: false
        matrix:
          include:
            - platform: 'macos-latest'
              args: '--target aarch64-apple-darwin'
            - platform: 'macos-latest'
              args: '--target x86_64-apple-darwin'
            - platform: 'ubuntu-22.04'
              args: ''
            - platform: 'windows-latest'
              args: ''

      runs-on: ${{ matrix.platform }}
      steps:
        - uses: actions/checkout@v4

        - name: Install dependencies (Ubuntu only)
          if: matrix.platform == 'ubuntu-22.04'
          run: |
            sudo apt-get update
            sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

        - name: Setup Node
          uses: actions/setup-node@v4
          with:
            node-version: 'lts/*'

        - name: Setup pnpm
          uses: pnpm/action-setup@v4
          with:
            version: 9

        - name: Install Rust stable
          uses: dtolnay/rust-toolchain@stable
          with:
            targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

        - name: Rust cache
          uses: swatinem/rust-cache@v2
          with:
            workspaces: './src-tauri -> target'

        - name: Install frontend dependencies
          run: pnpm install

        - name: Build
          run: pnpm tauri build ${{ matrix.args }}

        - name: Upload artifacts
          uses: actions/upload-artifact@v4
          with:
            name: build-${{ matrix.platform }}${{ matrix.args && '-arm64' || '' }}
            path: |
              src-tauri/target/release/bundle/
            if-no-files-found: warn
  ```
- **GOTCHA**: Ubuntu needs webkit2gtk and other system dependencies
- **VALIDATE**: Push to GitHub, verify workflows run (may fail on first push until secrets configured)

---

## Testing Strategy

### Manual Testing Checklist

| Test Case | Steps | Expected Result |
|-----------|-------|-----------------|
| App launches | `pnpm tauri dev` | Window opens at 1200x800 |
| Window title | Launch app | Title shows "LocalWork Hero" |
| Sidebar visible | Launch app | Left sidebar with "New Chat" button |
| Chat input works | Type in input, press Enter | Input clears, console logs message |
| Settings toggle | Click Settings in sidebar | Right panel slides in |
| Settings close | Click X in settings | Right panel closes |
| Window resizable | Drag window edges | Resizes, respects min 800x600 |
| Version displayed | Open dev console | Shows "LocalWork Hero v0.1.0" |

### Build Verification

| Platform | Command | Expected |
|----------|---------|----------|
| Dev mode | `pnpm tauri dev` | Window opens, hot reload works |
| Production build | `pnpm tauri build` | Creates installer in `src-tauri/target/release/bundle/` |

---

## Validation Commands

### Level 1: STATIC_ANALYSIS

```bash
pnpm run lint && pnpm run typecheck
```

**EXPECT**: Exit 0, no errors or warnings

### Level 2: BUILD_VERIFICATION

```bash
pnpm tauri build
```

**EXPECT**: Creates platform-specific installer without errors

### Level 3: DEV_MODE

```bash
pnpm tauri dev
```

**EXPECT**: App window opens, UI renders correctly

---

## Acceptance Criteria

- [ ] `pnpm tauri dev` launches the app successfully
- [ ] Window title shows "LocalWork Hero"
- [ ] Sidebar displays with "New Chat" button and "Settings" link
- [ ] Chat area shows welcome message
- [ ] Chat input accepts text and clears on submit
- [ ] Settings panel toggles open/closed
- [ ] `getAppInfo()` command returns correct version
- [ ] `pnpm tauri build` produces installer without errors
- [ ] GitHub Actions workflow configured for all 3 platforms
- [ ] Window respects minimum size (800x600)

---

## Completion Checklist

- [ ] All tasks completed in dependency order
- [ ] Each task validated immediately after completion
- [ ] Level 1: Static analysis passes
- [ ] Level 2: Production build succeeds
- [ ] Level 3: Dev mode works correctly
- [ ] All acceptance criteria met

---

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Tauri v2 breaking changes | LOW | HIGH | Pin exact versions in package.json and Cargo.toml |
| shadcn/ui incompatibility | LOW | MEDIUM | Test each component after installation |
| Platform-specific build failures | MEDIUM | MEDIUM | Test on all platforms early via CI/CD |
| Tailwind v4 migration issues | LOW | LOW | Follow official migration guide exactly |

---

## Notes

**Architecture Decisions:**
- Using pnpm for faster installs and better disk space efficiency
- shadcn/ui chosen over other component libraries for customizability and no runtime dependency
- Tailwind v4 chosen for modern Vite integration (uses @tailwindcss/vite plugin)
- Three-panel layout (sidebar, chat, settings) mirrors familiar chat app patterns

**Future Considerations:**
- Phase 2 will add LLM inference, requiring significant changes to ChatArea
- Phase 3/4 will need new permission dialogs integrated into settings
- Phase 5 will add conversation persistence, requiring state management

**Dependencies to Lock:**
- Tauri CLI: ^2.0.0
- @tauri-apps/api: ^2.0.0
- React: ^19.0.0
- Vite: ^6.0.0
- Tailwind CSS: ^4.0.0

**References:**
- [Tauri v2 Create Project](https://v2.tauri.app/start/create-project/)
- [Tauri v2 GitHub Actions](https://v2.tauri.app/distribute/pipelines/github/)
- [shadcn/ui Vite Installation](https://ui.shadcn.com/docs/installation/vite)
- [dannysmith/tauri-template](https://github.com/dannysmith/tauri-template) - Reference architecture
