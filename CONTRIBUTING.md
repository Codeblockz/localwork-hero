# Contributing to LocalWork Hero

Thank you for your interest in contributing! This guide will help you get started.

## Development Setup

### Prerequisites

- Node.js LTS
- pnpm v9+
- Rust stable
- Platform-specific dependencies (see README)

### Getting Started

```bash
git clone https://github.com/Codeblockz/localwork-hero.git
cd localwork-hero
pnpm install
pnpm tauri dev
```

## How to Contribute

### Reporting Bugs

1. Check existing issues to avoid duplicates
2. Use the bug report template
3. Include:
   - OS and version
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable

### Suggesting Features

1. Check the roadmap in README
2. Open a feature request issue
3. Describe the problem it solves
4. Keep it focused on end-user needs

### Submitting Code

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Run checks: `pnpm run typecheck`
5. Commit with conventional format: `feat: add cool feature`
6. Push and open a pull request

## Code Style

### TypeScript/React

- Use TypeScript strict mode
- Prefer `const` over `let`
- Functional components with hooks
- Keep components under 150 lines
- Use `@/` path alias for imports

### Rust

- Follow Rust idioms
- Use `?` for error propagation
- Keep Tauri commands thin
- Use meaningful error types

### Commits

Use conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code change that neither fixes nor adds
- `docs:` Documentation only
- `test:` Adding tests
- `chore:` Maintenance

## What We're NOT Building

Per our PRD, we avoid:

- Cloud sync (local-only by design)
- Team/enterprise features
- Mobile apps
- Usage monitoring/analytics
- API/developer integrations

## Pull Request Process

1. Update documentation if needed
2. Ensure CI passes
3. Request review from maintainers
4. Address feedback
5. Squash and merge when approved

## Questions?

Open a discussion or reach out to maintainers.
