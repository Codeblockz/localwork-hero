# LocalWork Hero - Development Guidelines

## Project Overview

LocalWork Hero is a cross-platform desktop application that brings Cowork-style agentic AI to everyone—running entirely on local hardware with open-source models. Built with Tauri v2 (Rust backend), React 19 (TypeScript frontend), and llama.cpp for inference.

## Core Principles

### KISS - Keep It Simple, Stupid
- Write the simplest code that solves the problem
- Avoid clever solutions when straightforward ones work
- If code needs extensive comments to explain, simplify it instead
- Prefer readable code over "elegant" code

### YAGNI - You Aren't Gonna Need It
- Only implement features that are explicitly required NOW
- Don't add configuration options "just in case"
- Don't build abstractions for hypothetical future needs
- Delete speculative code, don't comment it out

### DRY - Don't Repeat Yourself
- Extract repeated logic into functions only when it's used 3+ times
- Don't create premature abstractions for 2 similar pieces of code
- Duplication is better than the wrong abstraction

## Coding Standards

### General
- Write self-documenting code with clear naming
- Keep functions small and focused (single responsibility)
- Fail fast with clear error messages
- No magic numbers - use named constants

### TypeScript/React
- Use TypeScript strict mode
- Prefer `const` over `let`, never use `var`
- Use functional components with hooks
- Keep components under 150 lines; split if larger
- Colocate related code (component + styles + tests)

### Rust
- Follow Rust idioms - use `?` for error propagation
- Prefer `Result` over panics for recoverable errors
- Keep Tauri commands thin - delegate to service modules
- Use meaningful error types, not string errors

## Debugging & Fixing Rules

### DO NOT
- Add backwards compatibility shims or migration code
- Rename unused variables to `_var` - delete them entirely
- Add defensive checks for impossible states
- Create wrapper functions that just call another function
- Add logging "just to be safe"
- Refactor unrelated code while fixing a bug

### DO
- Fix the actual root cause, not symptoms
- Remove dead code completely
- Keep fixes minimal and focused
- Test the specific scenario that was broken
- If something is unused, delete it

## File Organization

```
src/                    # React frontend
  components/           # UI components
    ui/                 # shadcn/ui primitives
    chat/               # Chat feature components
    layout/             # Layout components
  hooks/                # Custom React hooks
  lib/                  # Utilities

src-tauri/              # Rust backend
  src/
    main.rs             # Entry point
    commands/           # Tauri command handlers
    services/           # Business logic
```

## Commit Standards

- Write commits in imperative mood: "Add feature" not "Added feature"
- Keep commits atomic - one logical change per commit
- No "WIP" or "fix" commits - squash before merging

## Dependencies

- Add dependencies only when they provide significant value
- Prefer standard library solutions over external packages
- Audit new dependencies for bundle size impact
- Pin exact versions in production

## Testing

- Test behavior, not implementation
- Don't mock what you don't own
- Keep tests simple and readable
- Delete tests for deleted features

## What NOT to Build

Per the PRD, we are explicitly NOT building:
- Cloud sync (local-only by design)
- Team/enterprise features (single-user focus)
- Mobile apps (desktop only)
- Usage monitoring/analytics (privacy-first)
- API/developer integrations (end-user product)
