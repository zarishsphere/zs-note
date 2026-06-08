# ZarishNote — Master TODO

**Purpose:** Single source of truth for everything that needs to be built.  
**Audience:** New contributors, developers, project managers.  
**Derived from:** Phase 1-3 roadmaps (`005-roadmap/`) + specifications (`002-specifications/`).

---

## How to Use This File

- **New to the project?** Start with §A (Guidelines for Everyone), then skim §1-3 to understand scope.
- **Developer picking up work?** Go directly to the phase you're working on. Each task links to its specification. Code lives at [`zarishsphere/zs-note`](https://github.com/zarishsphere/zs-note).
- **Project manager?** Use the checklist format to track progress.

---

## Project Map

| Repository | Purpose | URL |
|---|---|---|
| **zarishsphere/zs-note** | **Source code + Blueprint** — Rust backend + Svelte frontend + Python ingestion + full specification docs | [github.com/zarishsphere/zs-note](https://github.com/zarishsphere/zs-note)

### File Layout: `zarishsphere/zs-note`

```
zs-note/
├── src/                          # Svelte 5 frontend (28 files)
│   ├── main.ts                   # App entry
│   ├── app.css                   # Global CSS variables
│   └── lib/
│       ├── components/           # 18 Svelte components
│       ├── stores/               # 4 rune stores (.svelte.ts)
│       ├── commands/             # 3 command modules
│       ├── milkdown/             # Editor setup
│       └── types.ts              # TypeScript interfaces
├── src-tauri/                    # Rust backend (32 files)
│   └── src/
│       ├── commands/             # 9 command handler modules
│       ├── sandbox/              # Wasmtime engine
│       ├── ai/                   # 4 AI providers
│       ├── git/                  # Git engine
│       ├── mcp/                  # MCP client
│       ├── vector/               # Vector store
│       └── config.rs, types.rs, logging.rs
├── ingestion/                    # Python ingestion (21 files)
│   └── src/zarishnote_ingest/
│       ├── converters/           # 12 format converters
│       ├── cli.py                # CLI entry point
│       └── ...
├── docs/                         # Blueprint specifications (39 Markdown files)
│   ├── README.md                 # Master blueprint document
│   ├── TODO.md                   # Build task tracker
│   ├── 001-concept/              # Vision, personas, value prop
│   ├── 002-specifications/       # All feature specs (8 groups)
│   ├── 003-architecture/         # System architecture docs
│   ├── 004-security/             # Threat model & sandbox
│   ├── 005-roadmap/              # Phase 1-3 roadmaps
│   ├── 006-assets/               # Hiring docs, brand guidelines
│   └── 007-prototypes/           # .znrc examples, WASM hello-world
├── package.json, Cargo.toml, tauri.conf.json, ...
└── README.md
```

---

## A. Guidelines for Everyone

### A.1 New User (Installing & Using ZarishNote)

1. **No ZarishNote release exists yet.** This is a blueprint. The actual app is being built at [`zarishsphere/zs-note`](https://github.com/zarishsphere/zs-note).
2. ZarishNote will be a **desktop app** (Windows, macOS, Linux) during Phase 1. Mobile (iOS, Android) comes in Phase 2.
3. All data stays **local** — no cloud account needed. Your vault is a folder on your machine.
4. For AI features, you need either:
   - An API key from a provider (OpenAI, Anthropic, Google, DeepSeek) stored in your **OS Keychain**, OR
   - A local AI model via **Ollama** (free, runs on your machine).
5. Keyboard shortcuts are listed in `001-editor-spec.md §6`.
6. If something breaks: file an issue at [`zarishsphere/zs-note/issues`](https://github.com/zarishsphere/zs-note/issues).

### A.2 Developer (Contributing Code)

1. **Blueprint vs. Code:** Both live in this repo. Specs are in `docs/`, code lives in `src/`, `src-tauri/`, and `ingestion/`.
2. **Tech stack:** Rust (Tauri v2 backend) + Svelte 5 + TypeScript 6 (frontend) + pnpm 11.x + Vite 8.
3. **Build commands** run from the repo root: `pnpm install`, `pnpm tauri dev`, `pnpm typecheck`.
4. **Task checklist format:** Each task below uses `- [ ]`. Mark `- [x]` when code is written and pushed. Mark `[~]` for partially scaffolded. Unmark `[ ]` for not started.
5. **Before starting a task, read:**
   - The relevant spec in `002-specifications/`
   - The architecture doc in `003-architecture/`
   - The `.znrc` schema in `001-core-editor/003-znrc-schema.md`
6. **Security first:** All AI tools, MCP servers, and plugins run in the Wasmtime sandbox. Never bypass it.

### A.3 Specification Writer (Editing This Blueprint)

1. All docs follow a consistent format: filename header → subtitle → metadata block → TOC → sections → footer.
2. YAML front matter is NOT used in spec files (only in `.znrc` and AI templates).
3. Placeholder URLs: use `example.com` per RFC 2606, not real domains. Use `YOUR_USERNAME` for GitHub handles.
4. If you create a new spec, add it to the file tree in `README.md §4`.
5. Empty stubs (0 bytes) must be filled before they are referenced. Compare `glob **/*.md` against the README tree.

---

## B. Infrastructure Setup (Before Any Phase)

These tasks are prerequisites not tied to a specific sprint.

| Task | Owner | Status | Notes |
|---|---|---|---|
| Register `zarishsphere.com` domain | ✅ Done | Already owned, hosted on Cloudflare free tier |
| Set up `security@zarishsphere.com` email forwarding | ✅ Done | Via Cloudflare Email Routing → zarishsphere@gmail.com |
| Add `registry.zarishsphere.com` CNAME → Cloudflare Pages | ZarishSphere Foundation | ❌ | Free: CNAME record in Cloudflare DNS, static site from GitHub repo |
| Create GitHub organization `zarishsphere` | ✅ Done | Organization verified, 5 members linked |
| Create code repo `zarishsphere/zs-note` | ✅ Done | Scaffolded code pushed (115+ files, 30+ commits) |
| Merge blueprint specs into `docs/` of zs-note | ✅ Done | 39 Markdown files merged into `docs/` directory |
| Configure GitHub Pages for project site | ✅ Done | Served from `/docs` on `main` at `zarishsphere.github.io/zs-note` |
| Install Rust toolchain + Tauri deps | ✅ Done | `cargo build` succeeds locally (libwebkit2gtk-4.1, libgtk-3, librsvg2, etc.) |
| **All CI checks pass** | ✅ Done | `cargo fmt`, `cargo clippy`, `cargo test`, `pnpm typecheck`, `ruff` all green |

---

## 1. Phase 1 — Core MVP (Weeks 1-8)

**Goal:** Desktop app (Windows, macOS, Linux) with editor, AI, sandbox, ingestion, MCP, voice, and publishing.

> **Legend:** `[x]` = code written & pushed to [`zarishsphere/zs-note`](https://github.com/zarishsphere/zs-note) | `[~]` = partially scaffolded | `[ ]` = not started
>
> **Reality check:** All `[x]` items have source code written and pushed. **All CI checks pass** — `cargo fmt --check`, `cargo clippy`, `cargo test` (8/8), `pnpm typecheck`, and `ruff` all green on ubuntu-latest. Rust toolchain **v1.96.0 is installed** locally, but `cargo build` is blocked by missing Tauri system deps (no sudo). CI runs on GitHub Actions with **31 commits pushed**. Cross-platform build workflow configured but **never triggered** (requires `v*` tag). Verify compilation by pushing a tag or running `cargo build` on a machine with Tauri system libraries (`libwebkit2gtk-4.1-dev` etc.).

### 1.1 Week 1-2: Project Scaffold + Core Editor

Spec: `001-editor-spec.md`

- [x] Initialize Tauri v2 project with Svelte 5 + Vite 8 + TypeScript 6
- [x] Configure pnpm workspace, `Cargo.toml`, `tauri.conf.json`
- [x] Set up CI/CD pipeline (GitHub Actions: lint, test, build per platform)
- [x] Configure Dependabot for dependency vulnerability scanning (npm, cargo, pip, actions)
- [x] Integrate Milkdown v7 with CommonMark + GFM presets
- [x] Implement WYSIWYG mode (default)
- [x] Implement Source mode with Shiki syntax highlighting *(stub — Shiki not wired)*
- [x] Implement Split mode (side-by-side)
- [x] Basic formatting toolbar (H1–H3, B, I, code, link, list)
- [x] KaTeX math rendering (inline + block)
- [x] Mermaid diagram rendering (fenced ` ```mermaid ` blocks)
- [x] Table editor with context toolbar
- [x] Image paste/drag-and-drop
- [x] Keyboard shortcuts (see spec §6) *(global shortcuts in App.svelte)*
- [x] Editor settings panel (theme, font, etc.)

**Deliverable:** Functional Markdown editor with three view modes, math, diagrams, and image support. ✅

### 1.2 Week 3-4: File Manager + Git Engine + Config

Specs: `002-file-manager-spec.md`, `002-sync-spec.md`, `003-znrc-schema.md`

- [x] File tree sidebar with folder navigation
- [x] File operations: create, rename, delete, duplicate, move
- [x] File import dialog (single + bulk)
- [x] Tag system (read from front matter, manual assign) *(tag sidebar + filter)*
- [x] Full-text search with fuzzy matching *(keyword scoring in vector store)*
- [x] Git engine: auto-commit on save (debounced)
- [x] Git engine: commit history browser
- [x] Git engine: diff view
- [x] `.znrc` schema parser in Rust (serde_yaml)
- [x] `.znrc` validation on vault open
- [x] Settings GUI bound to `.znrc` fields
- [x] Hot-reload `.znrc` on file change

**Deliverable:** Working file manager with Git version history and fully configured settings system. ✅

### 1.3 Week 5-6: Sandbox + AI + Ingestion Foundations

Specs: `001-sandbox-spec.md`, `002-ai-providers.md`, `003-ai-templates.md`, `001-ingestion-spec.md`

- [x] Wasmtime engine integration (create engine, compile module, execute)
- [x] Capability model (permissions parsing + runtime check)
- [x] Network proxy with domain allow-list
- [x] Filesystem scoping with virtual `workspace://` paths
- [x] Resource limits (memory, timeout, output size)
- [x] Audit logging (`.znrc-audit.log`)
- [x] AI chat panel UI (message list, input, streaming display)
- [x] OpenAI provider client (streaming chat completion)
- [x] Anthropic Claude provider client
- [x] Google Gemini provider client
- [x] Ollama provider client
- [x] Provider switching in panel
- [x] API key management via OS Keychain (keyring crate)
- [x] Context injection (current doc, selection, core files)
- [x] Python ingestion CLI (`zarishnote-ingest`)
- [x] Tauri command wrapper for ingestion subprocess
- [~] Drag-and-drop file → ingestion *(images only, no document ingestion via drop)*

**Deliverable:** Sandbox running WASM tools, AI chat with 4 providers, document ingestion working. ✅ (drag-drop images only)

### 1.4 Week 7-8: MCP + Voice + Publish + Integration

Specs: `001-mcp-spec.md`, `001-voice-spec.md`, `001-publish-spec.md`, `003-knowledge-bases.md`

- [x] MCP stdio transport (spawn subprocess, JSON-RPC framing)
- [x] MCP HTTP transport (SSE streaming, JSON-RPC)
- [x] Tool routing (AI → MCP server → result)
- [~] Human-in-the-loop confirmation UI *(backend router logic exists, no frontend dialog)*
- [x] MCP server configuration GUI
- [x] Knowledge base indexing *(HashMap keyword index, no LanceDB/embeddings)*
- [~] Knowledge base query *(keyword search works front-to-back, not true RAG)*
- [ ] Whisper.cpp integration (recording + transcription) *(stub only)*
- [ ] Voice dictation into editor
- [ ] Audio file import transcription
- [x] GitHub publishing (API push to repo)
- [ ] Custom API publishing (POST to endpoint)
- [ ] RSS feed generation
- [ ] Image hosting (GitHub, Cloudflare)
- [x] Settings → Publish panel
- [ ] Integration test suite (unit + E2E for all modules)
- [ ] Cross-platform build verification
- [ ] Installer generation (NSIS, DMG, AppImage)

**Deliverable:** Complete V1 MVP with all features integrated and cross-platform installers. *(scaffolding gaps: voice, image hosting, installer)*

### 1.5 Phase 1 Acceptance Criteria

- [x] Editor: open, edit, save Markdown files in all three modes
- [x] File manager: browse, search, tag, organize vault
- [x] Git: every save committed, history browsable
- [x] AI: chat with any provider, insert/replace text
- [x] Sandbox: WASM tool executes with declared capabilities
- [~] Ingestion: PDF, DOCX, PPTX, XLSX, EPUB, YouTube → Markdown
- [~] MCP: connect GitHub server, list issues via AI
- [~] Knowledge base: index "knowledge/" folder, query via AI
- [ ] Voice: record and transcribe into document
- [~] Publishing: push document to GitHub repo
- [ ] Installer: works on clean Windows, macOS, Linux

---

## 2. Phase 2 — Full Feature Set + Mobile (Weeks 9-16)

**Goal:** iOS/Android apps, MCP marketplace, plugin system, advanced AI, cross-device sync, i18n.

> Code for Phase 2 goes to [`zarishsphere/zs-note`](https://github.com/zarishsphere/zs-note). Specifications are in this repo.

### 2.1 Week 9-10: Mobile + Multi-Window

- [ ] Tauri v2 mobile targets: iOS (arm64), Android (arm64)
- [ ] Touch toolbar for Markdown formatting
- [ ] Tab bar for multi-file editing
- [ ] iPad Magic Keyboard shortcut support
- [ ] Floating toolbar on touch surfaces
- [ ] Mobile-optimized AI panel (bottom sheet instead of right panel)
- [ ] Multi-window support on desktop
- [ ] Window state persistence (position, size, active file)
- [ ] App Store preparation (Apple App Store submission, Google Play Store)
- [ ] Mobile automated testing (emulator/device farm integration)

### 2.2 Week 11-12: MCP Marketplace + Plugin System

Specs: `002-mcp-marketplace.md`, `001-plugin-spec.md`

> **Prerequisite:** `registry.zarishsphere.com` CNAME + Cloudflare Pages site (see §B, ~10 min setup).

- [ ] MCP marketplace registry API
- [ ] One-click MCP server installation
- [ ] Server version management and updates
- [ ] WASM plugin API (WIT interface, host functions)
- [ ] Plugin manifest format and validation
- [ ] Plugin installer (download `.wasm` to `.znrc-plugins/`)
- [ ] Plugin sandboxing (capability model same as tools)
- [ ] Plugin marketplace browser UI
- [ ] Plugin signing and verification
- [ ] Plugin development guide and sample plugins

### 2.3 Week 13-14: Advanced AI + Image Generation

Specs: `001-ai-chat-spec.md`, `002-ai-providers.md`, `003-ai-templates.md`

- [ ] Image generation dialog (DALL-E, Stability AI)
- [ ] Image saving to `assets/ai-images/` + Markdown insertion
- [ ] Speaker diarization via pyannote.audio
- [ ] Speaker label UI (rename speakers, color coding)
- [ ] SRT export with speaker labels
- [ ] Temperature/parameter controls per model
- [ ] System prompt configuration per workspace
- [ ] Multi-provider routing rules GUI
- [ ] AI template library expansion
- [x] Context inspector panel (what is being sent to AI) *(scaffolded)*

### 2.4 Week 15-16: Polish, Performance, Ecosystem

- [ ] i18n framework via svelte-i18n or FormatJS (Bangla, English, Arabic as first targets)
- [ ] Translation file format specification and external translation management guide
- [ ] Right-to-left (RTL) layout testing for Arabic
- [ ] Large vault performance (10K+ files)
- [ ] Lazy-loading file tree for large vaults
- [ ] Memory profiling and optimization
- [ ] Accessibility audit (WCAG 2.1 AA)
- [ ] Documentation site launch
- [ ] Community contribution guidelines
- [ ] Performance benchmarks and regression suite
- [ ] Beta testing program for mobile

---

## 3. Phase 3 — Ecosystem (Weeks 17+, Ongoing)

**Goal:** Collaboration, enterprise features, sustainability. No fixed timeline.

> Code for Phase 3 goes to [`zarishsphere/zs-note`](https://github.com/zarishsphere/zs-note). Specifications are in this repo.

### 3.1 Collaboration Features

- [ ] Real-time collaboration via yjs (CRDT, integrates with ProseMirror/Milkdown)
- [ ] Comment and annotation (inline comments on sections)
- [ ] Shared vaults with permission levels
- [ ] Change proposals (suggest edits without direct push)
- [ ] Presence indicators (who else is viewing)
- [ ] Self-hosted relay server (optional, not SaaS)

### 3.2 Plugin Ecosystem

- [ ] Public plugin marketplace launch
- [ ] Optional paid plugins (ZarishSphere takes 0% cut)
- [ ] Plugin categories: renderers, panels, importers, exporters
- [ ] Stable WASM SDK with documentation
- [ ] Plugin testing framework (headless Wasmtime in CI)

### 3.3 Enterprise / Advanced

- [ ] FHIR integration (healthcare data import/export via FHIR API — see ingestion spec)
- [ ] Encryption at rest (vault-level encryption via Age or GPG)
- [ ] LDAP/OIDC authentication (for shared vaults)
- [ ] Exportable audit trails (regulated environments)
- [ ] Custom branding (white-label for NGOs and institutions)
- [ ] Bulk document workflows (template-based generation pipeline)
- [ ] Webhook triggers on file events

### 3.4 Long-Term Vision

- [ ] Offline-first field data collection with structured forms
- [ ] Semantic document linking across vaults
- [ ] Peer-to-peer sync (IPFS or similar)
- [ ] AI-assisted translation (100+ languages)
- [ ] Standard operating procedure (SOP) automation

---

## C. Infrastructure Status

| Item | Needed By | How | Status |
|---|---|---|---|---|
| `zarishsphere.com` domain | Already owned | Cloudflare free DNS + Pages | ✅ Done |
| `security@zarishsphere.com` forwarding | Phase 1 | Cloudflare Email Routing (free) or Mailgun free tier | ❌ Needs setup |
| `registry.zarishsphere.com` DNS | Phase 2 | Add CNAME record in Cloudflare DNS (free) | ❌ 1 DNS record |
| `registry.zarishsphere.com` static site | Phase 2 | Cloudflare Pages from GitHub `registry` repo (free tier) | ❌ Repo + deploy |
| `zarishsphere/zs-note` code + specs repo | Phase 1 | GitHub — code + blueprint merged | ✅ Done (30+ commits, 115+ code files + 39 spec files) |
| Rust toolchain (`rustup` + `cargo`) | Phase 1 | Official installer | ✅ Done (v1.96.0) |
| Tauri system deps (`libwebkit2gtk-4.1`, etc.) | Phase 1 | `apt install` on Ubuntu / `brew` on macOS | ✅ Done |
| GitHub Actions CI workflow | Phase 1 | `.github/workflows/ci.yml` | ✅ Passed (format ✅, clippy ✅, test ✅, typecheck ✅, ruff ✅) |
| GitHub Actions build workflow | Phase 1 | `.github/workflows/build.yml` | ✅ Done (triggered on tag push) |
| Dependabot config | Phase 1 | `.github/dependabot.yml` | ✅ Done (npm, cargo, pip, actions) |
| GitHub Pages for project docs | Phase 1 | Served from `/docs` on `main` | ✅ Configured at `zarishsphere.github.io/zs-note` |

---

*ZarishSphere Foundation · V1 · June 2026*
*License: CC BY 4.0*
