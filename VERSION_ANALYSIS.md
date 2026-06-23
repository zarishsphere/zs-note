# ZarishNote — Version Analysis & Remediation (June 23, 2026)

## Executive Summary

Audited `package.json`, `pyproject.toml`, `Cargo.toml`, `tsconfig.json`, and CI workflows for version discrepancies. Found **2 critical** version mismatches in Milkdown ecosystem (would crash `pnpm install`), **4 medium-severity** outdated packages, and **1 configuration drift** in tsconfig.json. All issues have been remediated.

---

## 1. Frontend — `package.json`

### Critical: Nonexistent versions in Milkdown sub-packages

The previous "fix" bumped all Milkdown packages to `^7.21.2`, but **`@milkdown/plugin-math` and `@milkdown/plugin-table` do not share the core versioning**:

| Package | Incorrect Specifier | Actual Latest | Fix Applied |
|---|---|---|---|
| `@milkdown/plugin-math` | `^7.21.2` (does not exist) | `7.5.9` | `^7.5.9` |
| `@milkdown/plugin-table` | `^7.21.2` (does not exist) | `5.3.1` | `^5.3.1` |

Verification via npm registry confirmed latest published versions.

### Medium: Outdated transitive packages

| Package | Old Specifier | Latest | Fix Applied |
|---|---|---|---|
| `vite` | `^8.0.16` | `8.1.0` | `^8.1.0` |
| `svelte` | `^5.56.2` | `5.56.3` | `^5.56.3` |
| `svelte-check` | `^4.6.0` | `4.7.0` | `^4.7.0` |
| `@tauri-apps/cli` | `^2.11.2` | `2.11.3` | `^2.11.3` |

### Not changed (already at latest)

`@tauri-apps/api@^2.11.1`, all Tauri plugins, `katex@0.17.0`, `mermaid@11.15.0`, `typescript@6.0.3`, `@sveltejs/vite-plugin-svelte@7.1.2`

---

## 2. Frontend — `tsconfig.json`

| Field | Old Value | New Value | Rationale |
|---|---|---|---|
| `target` | `ES2021` | `ESNext` | TypeScript 6 + Vite 8 + Svelte 5 support all modern JS features |
| `lib` | `["ES2021", ...]` | `["ESNext", ...]` | Match target; unlocks newer type defs |

---

## 3. Backend (Python) — `ingestion/pyproject.toml`

Already correct — all core dependencies have lower bounds as recommended. Only minor note:

- **Low severity**: `[project.optional-dependencies]` extras (`docx`, `pptx`, `xlsx`) redeclare packages already pulled by `markitdown[all]` (`mammoth`, `python-pptx`, `pandas`, `openpyxl`). This is API-surface redundancy, not a bug. Left as-is.

### Verified bounds present

`markitdown[all]>=0.1.6`, `defusedxml>=0.7.1`, `charset-normalizer>=3.3.0`, `markdownify>=0.14.0`, `requests>=2.31.0`, `beautifulsoup4>=4.15.0`, `feedparser>=6.0.12`

---

## 4. Backend (Rust) — `src-tauri/Cargo.toml` & `Cargo.lock`

No version discrepancies found. Cargo.lock pins all transitive dependencies. Cargo.toml uses standard Rust semver specifiers. Dependabot is configured to open PRs for minor/patch updates grouped by ecosystem.

---

## 5. CI/CD — `.github/workflows/`

All GitHub Actions are pinned by commit SHA with human-readable tag comments. Node 24, pnpm v6 action, Python 3.12. No drift.

---

## Summary of Files Changed

| File | Changes |
|---|---|
| `package.json` | `plugin-math` `^7.21.2` → `^7.5.9`, `plugin-table` `^7.21.2` → `^5.3.1`, `vite` `^8.0.16` → `^8.1.0`, `svelte` `^5.56.2` → `^5.56.3`, `svelte-check` `^4.6.0` → `^4.7.0`, `@tauri-apps/cli` `^2.11.2` → `^2.11.3` |
| `tsconfig.json` | `target` `ES2021` → `ESNext`, `lib` `["ES2021",...]` → `["ESNext",...]` |
