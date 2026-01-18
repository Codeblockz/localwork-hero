# Bug: App Crashes During UI Testing

## Summary
The app crashes intermittently when testing tool use functionality through the Tauri MCP plugin. The crash appears to happen when sending messages with the tool-enabled inference.

## Symptoms
1. App becomes unresponsive after clicking "Send" button
2. `mcp__tauri-mcp__click` returns "Error: App is starting up. Please wait and try again."
3. `mcp__tauri-mcp__app_status` shows `"status": "not_running"`
4. No error logs captured in frontend console

## Steps to Reproduce
1. Launch app with `pnpm tauri dev` or via MCP `launch_app`
2. Open Settings, select a model
3. Grant folder access (e.g., `/home/ryan/FAFO/localwork-hero/src-tauri/TempTest`)
4. Close settings
5. Type a message like "List files in /home/ryan/FAFO/localwork-hero/src-tauri/TempTest"
6. Click Send button
7. App crashes silently

## Previous Bug (Fixed)
Earlier, we encountered `"Insufficient Space of 512"` error because the `LlamaBatch` was created with a fixed 512 token capacity, but the system prompt with tool definitions exceeded this.

**Fix applied:** Changed batch size to be dynamic:
```rust
let batch_size = std::cmp::max(2048, tokens.len() + 512);
let mut batch = LlamaBatch::new(batch_size, 1);
```

## Current Investigation
The crash happens silently without captured logs. Possible causes:
1. **Rust panic** in the `send_message_with_tools` command that isn't being caught
2. **Memory issue** with large context/batch sizes
3. **Race condition** between MCP plugin and inference thread
4. **Tauri IPC timeout** if inference takes too long (local LLM can be slow)

## Environment
- Platform: Linux (Ubuntu)
- Tauri: v2
- Model: Qwen 0.5B (quantized)
- Backend: llama.cpp via llama-cpp-2 crate

## Recommended Next Steps
1. Add more granular error logging in `send_message_with_tools`
2. Wrap inference call in `catch_unwind` to capture panics
3. Test inference directly via CLI without Tauri to isolate the issue
4. Check if issue is specific to MCP plugin interaction
5. Add timeout handling for long-running inference operations
