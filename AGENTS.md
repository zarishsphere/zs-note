# ZarishNote — Agent Guide

The directory (`docs/`) contains the **specification/blueprint** for the ZarishNote project.  
The actual code repository: <https://github.com/zarishsphere/zs-note>

## Structure

Numbered directories (`001-concept` through `007-prototypes`) contain Markdown specification documents. Each subdirectory has a natural reading order via its numeric prefix.

## Recommended reading order (from README)

1. `001-concept/001-vision.md` — why it exists
2. `003-architecture/001-system-architecture.md` — overall shape
3. `002-specifications/002-sandbox-engine/001-sandbox-spec.md` — sandbox is the key differentiator
4. `002-specifications/001-core-editor/003-znrc-schema.md` — `.znrc` is the workspace "brain"
5. `005-roadmap/001-phase-one.md` — where implementation starts

## Tech stack (for reference when writing specs or prototypes)

- **Framework:** Tauri v2, Rust backend, native WebView
- **Frontend:** Svelte 5, TypeScript 6, Vite 8
- **Package manager:** pnpm 11.x
- **Editor core:** Milkdown (ProseMirror) 7.x
- **Sandbox:** Wasmtime 45.x (Rust crate)
- **Vector store:** LanceDB (embedded, local)
- **CI/CD:** GitHub Actions

## No commands to run

This documentation directory has no `package.json`, no build scripts, no tests, no CI workflows. It is purely Markdown documentation. Edits are prose changes only — no linting, typechecking, or verification commands exist.

## Master TODO

`/TODO.md` is the single source of truth for all build tasks across all three phases. It includes guidelines for new users and developers, infrastructure prerequisites, and per-sprint task checklists. Start there if you need to understand what still needs to be done.

## URL conventions when writing specs

- Use `example.com` per RFC 2606 for placeholder domains, not real or author-specific domains.
- Use `YOUR_USERNAME` as placeholder for GitHub handles (not personal handles like `arwazarish`).
- All `registry.zarishsphere.com` URLs are Phase 2 and can be hosted via Cloudflare Pages on your existing domain.
- `security@zarishsphere.com` requires email forwarding setup — annotate with this note.

## Prototype files

The `007-prototypes/` directory contains:
- `001-zrc-examples/default.znrc` — minimal starter `.znrc` config (populated)
- `001-zrc-examples/advanced.znrc` — full-featured `.znrc` example (populated)
- `002-sandbox-hello/README.md` — instructions to build a hello-world WASM test module

All prototype files are populated (none are 0 bytes).

## GitHub identity & access

| Detail | Value |
|---|---|
| Active GitHub user | **codeandbrain** (Mohammad Ariful ISLAM) |
| Alt accounts on this machine | arwazarish, devopsariful, healthbgd (all same person) |
| GitHub org | **zarishsphere** (verified org, free plan) |
| Role in org | **admin** — full access |
| Org members (5 total) | codeandbrain (you), arwazarish, bgd-cpms, devopsariful, healthbgd |
| Org public repos | 1 currently (`zarishsphere/zs-note`) |
| `zs-note` repo on GitHub | **exists** — has scaffolded code (115+ files, 25 commits) at `zarishsphere/zs-note` |
| Git remote | **origin** — `git@github.com:zarishsphere/zs-note.git` |
| Git protocol | SSH (key loaded: `health-pm Lenovo Ubuntu`) |
| Git user (global) | `Mohammad Ariful Islam <zarishsphere@gmail.com>` |

### Important notes for agents

- The `zs-note` code repo at `zarishsphere/zs-note` **exists** with scaffolded code (115+ files, 30+ commits). **All CI checks pass** (format, clippy, tests, typecheck). Agents can clone, read, and push to it.
- Blueprint specifications live in `docs/` of this repo — no separate blueprint repository exists.
- Use **`codeandbrain`** for authenticated GH operations (it's the active account with repo + workflow scopes).
- The `arwazarish` account is available but inactive — switch with `gh auth switch -u arwazarish` if needed.
- All org members are the same person's aliases. Do not treat them as separate people.
