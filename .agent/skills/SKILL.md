---
name: zarishnote-builder
description: Use when writing implementation code for ZarishNote from the blueprint specs. Covers Tauri v2, Svelte 5, Rust, Wasmtime, Milkdown, LanceDB, and the .znrc config system.
---

# ZarishNote Builder Skill

Use this skill when the user asks you to write, scaffold, or implement any part of the ZarishNote application based on the blueprint specifications.

## Key reference documents

| File | What it covers |
|---|---|
| `001-concept/001-vision.md` | Project vision and positioning |
| `003-architecture/001-system-architecture.md` | Tauri layers, IPC, data flow |
| `003-architecture/003-tech-stack.md` | All library versions and rationale |
| `003-architecture/002-directory-structure.md` | Full source tree layout |
| `002-specifications/001-core-editor/001-editor-spec.md` | Editor features and keyboard shortcuts |
| `002-specifications/001-core-editor/003-znrc-schema.md` | Complete .znrc config schema |
| `002-specifications/002-sandbox-engine/001-sandbox-spec.md` | Wasmtime integration and capability model |
| `002-specifications/003-ai-engine/002-ai-providers.md` | Provider API endpoints and auth |
| `004-security/003-sandbox-implementation.md` | Rust sandbox implementation sketch |
| `005-roadmap/001-phase-one.md` | Week-by-week build tasks |
| `TODO.md` | Master task tracker |

## Tech stack reminders

- **Framework:** Tauri v2, Rust 2024 edition, native WebView
- **Frontend:** Svelte 5 with runes, TypeScript 6.x, Vite 8
- **Package manager:** pnpm 11.x
- **Editor core:** Milkdown 7.x (ProseMirror-based)
- **Sandbox:** Wasmtime 45.x (Rust crate)
- **Vector store:** LanceDB (embedded, local)
- **Embeddings:** FastEmbed-rs (local ONNX)
- **Git engine:** git2-rs 0.19+
- **Key storage:** Tauri keychain plugin
- **Voice:** Whisper.cpp via Rust FFI
- **Python:** markitdown (Microsoft) for ingestion

## Code conventions

- Rust: use `anyhow` for error handling, `serde` for serialization
- TypeScript: strict mode, prefer types over interfaces for data shapes
- Svelte: use `$state` runes, avoid stores for component-local state
- CSS: utility-first with app-level CSS variables for theming

## GitHub access

- Active user: `codeandbrain` (has repo + workflow scopes)
- Alt accounts available: `arwazarish` (has workflow scope too)
- Org: `zarishsphere` (you are admin)
- Code repo: `zarishsphere/zs-note` (create if it doesn't exist)
- Blueprint repo: specifications live in `docs/` within the `zarishsphere/zs-note` repo (no separate blueprint repo)
