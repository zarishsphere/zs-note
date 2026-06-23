# ZarishNote — Fixes Applied (June 23, 2026)

This document lists every change made to the original zip file before repackaging.

## TypeScript Fixes (2)

| File | Change |
|---|---|
| `src/lib/commands/editor.ts` | Added missing `import { toVaultRelativePath } from '../utils/vaultPath'` |
| `src/lib/stores/config.svelte.ts` | Added `as ProviderType` cast on line 307 to resolve type error |

## New Backend Commands (9)

| Command | File | Notes |
|---|---|---|
| `get_templates` | `src-tauri/src/commands/ai.rs` | Returns 8 built-in AI prompt templates as JSON |
| `git_log` | `src-tauri/src/commands/git.rs` | Alias for `git_history` accepting optional `file_path` |
| `git_restore` | `src-tauri/src/commands/git.rs` | Restores file to past commit; creates new commit |
| `mcp_toggle_server` | `src-tauri/src/commands/mcp.rs` | Toggles MCP server enabled/disabled by `id` |
| `sandbox_exec` | `src-tauri/src/commands/sandbox.rs` | Ad-hoc WASM execution by file path |
| `sandbox_list_snapshots` | `src-tauri/src/commands/sandbox.rs` | Lists tool names as snapshots |
| `sandbox_create_snapshot` | `src-tauri/src/commands/sandbox.rs` | Stub — logs intent |
| `sandbox_restore_snapshot` | `src-tauri/src/commands/sandbox.rs` | Stub — logs intent |
| `sandbox_delete_snapshot` | `src-tauri/src/commands/sandbox.rs` | Stub — logs intent |

## Backend Registration (lib.rs)

All 9 new commands + 3 previously unregistered sandbox commands (`sandbox_execute`, `sandbox_get_tools`, `sandbox_test_tool`) added to `invoke_handler![]` in `src-tauri/src/lib.rs`.

## Parameter Fixes

| Command | Problem | Fix |
|---|---|---|
| `mcp_add_server` | Backend expected `config_data`, frontend sent `server` | Backend rewritten to accept `server: Value` |
| `mcp_remove_server` | Backend expected `name`, frontend sent `id` | Backend now matches by `id` OR `name` |
| `mcp_test_connection` | Backend expected `name`, frontend sent `id` | Backend param renamed to `id` |
| `git_diff` (HistoryBrowser) | Frontend sent `filePath/from/to`, backend expects `path/rev1/rev2` | HistoryBrowser.svelte updated to send correct keys |

## Type Fixes

| Struct | Change |
|---|---|
| `McpServerInfo` (types.rs) | Added `id`, `enabled`, `command`, `args`, `url`, `error_message` fields to match frontend interface |

## Bug Fix

| File | Bug | Fix |
|---|---|---|
| `src-tauri/src/mcp/transport.rs` | `StdioTransport::new` had no `kill_on_drop(true)` — child process leaked on exit | Added `.kill_on_drop(true)` |

## Removed Clutter

- `dist/index.html` — stale build artifact
- `zs_note_interface/` — design mockup directory (not part of app)

## Tests Status

- Python ingestion: **41/41 pass** ✅
- TypeScript typecheck: **0 errors** ✅
