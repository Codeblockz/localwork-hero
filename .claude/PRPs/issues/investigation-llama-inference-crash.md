# Investigation: LLM Inference Segfault During Chat Message

**Issue**: Free-form (no GitHub issue)
**Type**: BUG
**Investigated**: 2026-01-18T12:00:00Z

### Assessment

| Metric     | Value    | Reasoning                                                                                           |
| ---------- | -------- | --------------------------------------------------------------------------------------------------- |
| Severity   | CRITICAL | App crashes completely with segfault when core feature (chat) is used; no workaround exists         |
| Complexity | MEDIUM   | Changes confined to 1-2 files in inference module; requires understanding llama-cpp-2 API           |
| Confidence | MEDIUM   | Multiple potential causes identified; exact crash point needs runtime verification with debug build |

---

## Problem Statement

The LocalWork Hero app crashes with a segfault when a user sends a chat message after selecting a model. The crash occurs in native llama-cpp-2 code during inference, specifically during `ctx.decode()` or `sampler.sample()` calls. Model loading succeeds, but the first inference attempt causes an immediate crash with no error message.

---

## Analysis

### Root Cause / Change Rationale

The crash appears to be caused by insufficient context initialization combined with potential batch size issues. The code uses `LlamaContextParams::default()` without setting critical parameters like `n_ctx` (context size), which may result in a context too small for the model or tokenized prompt.

### Evidence Chain

WHY: App crashes with segfault during inference
↓ BECAUSE: Native llama-cpp-2 code encounters invalid memory access
Evidence: Stack trace shows crash in `libc/__libc_start_main` originating from `localwork-hero` binary

↓ BECAUSE: Context or batch parameters are insufficient for the model/prompt
Evidence: `src-tauri/src/inference/llama.rs:72` - `let ctx_params = LlamaContextParams::default();`

↓ BECAUSE: Default context params don't specify n_ctx, n_batch, or thread count
Evidence: No explicit configuration after `default()` call

↓ ROOT CAUSE: `LlamaContextParams::default()` creates a context with potentially inadequate settings for Qwen2.5 models
Evidence: `src-tauri/src/inference/llama.rs:72-76`:
```rust
let ctx_params = LlamaContextParams::default();

let mut ctx = model
    .new_context(&self.backend, ctx_params)
    .map_err(|e| InferenceError::ContextError(e.to_string()))?;
```

### Secondary Contributing Factors

1. **Batch size mismatch** - Basic `generate()` uses fixed 512-token batch (line 99), but tool-aware version dynamically sizes it (line 190-191)
2. **Sampler index calculation** - `batch.n_tokens() - 1` at line 122 could be problematic if batch state is unexpected
3. **Previous fix attempt** - Commit `6bf35bd` attempted to fix inference crash by changing sampler approach, but underlying context issue persists

### Affected Files

| File                                    | Lines   | Action | Description                                       |
| --------------------------------------- | ------- | ------ | ------------------------------------------------- |
| `src-tauri/src/inference/llama.rs`      | 72-76   | UPDATE | Add explicit context parameters                   |
| `src-tauri/src/inference/llama.rs`      | 99      | UPDATE | Increase batch size or make dynamic               |
| `src-tauri/src/inference/llama.rs`      | 164-168 | UPDATE | Same context params fix for tool-aware variant    |

### Integration Points

- `src-tauri/src/lib.rs:127` - `send_message` command calls `inf.generate()`
- `src-tauri/src/lib.rs:156` - `send_message_with_tools` calls `inf.generate_with_tools()`
- `src/App.tsx:129` - Frontend triggers `sendMessageWithTools()`

### Git History

- **Introduced**: `877ff8c` - 2026-01-17 - "feat: add Phase 2 LLM integration with llama.cpp"
- **Previous fix attempt**: `6bf35bd` - 2026-01-17 - "fix: resolve inference crash and add conversation history"
- **Implication**: Original implementation had crash, fix was partial - context params were never addressed

---

## Implementation Plan

### Step 1: Configure explicit context parameters

**File**: `src-tauri/src/inference/llama.rs`
**Lines**: 72-76
**Action**: UPDATE

**Current code:**
```rust
// Line 72-76
let ctx_params = LlamaContextParams::default();

let mut ctx = model
    .new_context(&self.backend, ctx_params)
    .map_err(|e| InferenceError::ContextError(e.to_string()))?;
```

**Required change:**
```rust
// Configure context with explicit parameters for Qwen2.5 models
let ctx_params = LlamaContextParams::default()
    .with_n_ctx(std::num::NonZeroU32::new(2048))  // Context window size
    .with_n_batch(512);  // Batch size for prompt processing

let mut ctx = model
    .new_context(&self.backend, ctx_params)
    .map_err(|e| InferenceError::ContextError(e.to_string()))?;
```

**Why**: Explicit context size prevents llama.cpp from using potentially inadequate defaults

---

### Step 2: Apply same fix to generate_with_tools

**File**: `src-tauri/src/inference/llama.rs`
**Lines**: 164-168
**Action**: UPDATE

**Current code:**
```rust
// Line 164-168
let ctx_params = LlamaContextParams::default();

let mut ctx = model
    .new_context(&self.backend, ctx_params)
    .map_err(|e| InferenceError::ContextError(e.to_string()))?;
```

**Required change:**
```rust
// Configure context with explicit parameters - larger for tool definitions
let ctx_params = LlamaContextParams::default()
    .with_n_ctx(std::num::NonZeroU32::new(4096))  // Larger context for tools
    .with_n_batch(512);

let mut ctx = model
    .new_context(&self.backend, ctx_params)
    .map_err(|e| InferenceError::ContextError(e.to_string()))?;
```

**Why**: Tool-aware variant needs larger context for system prompt with tool definitions

---

### Step 3: Increase batch size for basic generate

**File**: `src-tauri/src/inference/llama.rs`
**Lines**: 99
**Action**: UPDATE

**Current code:**
```rust
// Line 99
let mut batch = LlamaBatch::new(512, 1);
```

**Required change:**
```rust
// Use dynamic batch size like tool-aware variant
let batch_size = std::cmp::max(1024, tokens.len() + 256);
let mut batch = LlamaBatch::new(batch_size, 1);
```

**Why**: Fixed 512-token batch may overflow with longer conversation history

---

### Step 4: Add safety check before sampling

**File**: `src-tauri/src/inference/llama.rs`
**Lines**: 119-122
**Action**: UPDATE

**Current code:**
```rust
// Lines 119-122
for _ in 0..max_tokens {
    // Sample next token - use batch.n_tokens() - 1 as the index
    // After batch.clear(), batch only has 1 token, so index should be 0
    let new_token = sampler.sample(&ctx, batch.n_tokens() - 1);
```

**Required change:**
```rust
for _ in 0..max_tokens {
    // Safety check: ensure batch has tokens before sampling
    let n_tokens = batch.n_tokens();
    if n_tokens == 0 {
        return Err(InferenceError::InferenceError("Batch is empty".to_string()));
    }
    let new_token = sampler.sample(&ctx, n_tokens - 1);
```

**Why**: Prevents potential underflow/invalid index if batch is unexpectedly empty

---

## Patterns to Follow

**From llama-cpp-2 examples - context configuration:**
```rust
// SOURCE: llama-cpp-2 crate documentation
// Pattern for context initialization with explicit params
let ctx_params = LlamaContextParams::default()
    .with_n_ctx(std::num::NonZeroU32::new(2048))
    .with_n_batch(512);
```

**From codebase - dynamic batch sizing:**
```rust
// SOURCE: src-tauri/src/inference/llama.rs:190-191
// Pattern for dynamic batch allocation
let batch_size = std::cmp::max(2048, tokens.len() + 512);
let mut batch = LlamaBatch::new(batch_size, 1);
```

---

## Edge Cases & Risks

| Risk/Edge Case              | Mitigation                                                          |
| --------------------------- | ------------------------------------------------------------------- |
| n_ctx too large for GPU     | Start with 2048/4096, can reduce if memory issues                   |
| Batch overflow              | Dynamic sizing with safety margin                                   |
| Empty batch during sampling | Add explicit check before `batch.n_tokens() - 1`                    |
| Thread safety               | Existing Mutex protection should be sufficient for single-user      |

---

## Validation

### Automated Checks

```bash
# Type check Rust code
cd src-tauri && cargo check

# Run any existing tests
cargo test

# Build in debug mode for better error messages
cargo build
```

### Manual Verification

1. Launch app with `pnpm tauri dev`
2. Open Settings, select the Qwen2.5 1.5B model
3. Send a simple message like "Hello! What is 2+2?"
4. Verify response appears without crash
5. Test multi-turn conversation (send 2-3 messages)
6. Test with tool usage (if applicable)

---

## Scope Boundaries

**IN SCOPE:**
- Context parameter configuration in `llama.rs`
- Batch size adjustments
- Safety checks for sampling index

**OUT OF SCOPE (do not touch):**
- Model download functionality
- Frontend UI components
- Tool execution logic (tools.rs)
- File permission system
- Upgrading llama-cpp-2 version (could be follow-up if fix doesn't work)

---

## Metadata

- **Investigated by**: Claude
- **Timestamp**: 2026-01-18T12:00:00Z
- **Artifact**: `.claude/PRPs/issues/investigation-llama-inference-crash.md`
