/**
 * Tests for src/lib/commands/editor.ts (PR changes).
 *
 * Key changes tested:
 * - toVaultRelativePath() calls were removed; paths are now forwarded as-is.
 * - moveFile() was removed from the module.
 * - getRecentFiles() return type changed from RecentFile[] to FileEntry[].
 * - getTags() return type changed (now proxied directly from invoke).
 */
import { describe, it, expect, vi, beforeEach } from 'vitest';

// ---------------------------------------------------------------------------
// Mock @tauri-apps/api/core
// ---------------------------------------------------------------------------
const invokeMock = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: invokeMock,
}));

import {
  readFile,
  saveFile,
  listFiles,
  createFile,
  createFolder,
  renameFile,
  deleteFile,
  duplicateFile,
  getTags,
  getRecentFiles,
  searchFiles,
} from './editor';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

beforeEach(() => {
  invokeMock.mockReset();
});

// ---------------------------------------------------------------------------
// readFile – path passed as-is (no vault-relative normalization)
// ---------------------------------------------------------------------------

describe('readFile', () => {
  it('passes the path directly to invoke without modification', async () => {
    invokeMock.mockResolvedValue('file content');

    await readFile('notes/daily/today.md');

    expect(invokeMock).toHaveBeenCalledWith('read_file', { path: 'notes/daily/today.md' });
  });

  it('does not strip leading slashes (backend handles normalization)', async () => {
    invokeMock.mockResolvedValue('');

    await readFile('/some/absolute/path.md');

    expect(invokeMock).toHaveBeenCalledWith('read_file', { path: '/some/absolute/path.md' });
  });

  it('returns the content from invoke', async () => {
    invokeMock.mockResolvedValue('# Hello');
    const content = await readFile('hello.md');
    expect(content).toBe('# Hello');
  });
});

// ---------------------------------------------------------------------------
// saveFile – path passed as-is
// ---------------------------------------------------------------------------

describe('saveFile', () => {
  it('passes path and content directly to invoke', async () => {
    invokeMock.mockResolvedValue(undefined);

    await saveFile('notes/note.md', '# Note');

    expect(invokeMock).toHaveBeenCalledWith('save_file', {
      path: 'notes/note.md',
      content: '# Note',
    });
  });
});

// ---------------------------------------------------------------------------
// listFiles – path passed as-is
// ---------------------------------------------------------------------------

describe('listFiles', () => {
  it('passes the path directly to invoke', async () => {
    invokeMock.mockResolvedValue([]);

    await listFiles('subfolder');

    expect(invokeMock).toHaveBeenCalledWith('list_files', { path: 'subfolder' });
  });

  it('returns the FileEntry array from invoke', async () => {
    const fakeEntries = [
      { name: 'a.md', path: 'a.md', is_dir: false },
      { name: 'notes', path: 'notes', is_dir: true },
    ];
    invokeMock.mockResolvedValue(fakeEntries);

    const result = await listFiles('');
    expect(result).toEqual(fakeEntries);
  });
});

// ---------------------------------------------------------------------------
// createFile – path passed as-is
// ---------------------------------------------------------------------------

describe('createFile', () => {
  it('passes the path directly to invoke', async () => {
    invokeMock.mockResolvedValue(undefined);

    await createFile('new-note.md');

    expect(invokeMock).toHaveBeenCalledWith('create_file', { path: 'new-note.md' });
  });
});

// ---------------------------------------------------------------------------
// createFolder – path passed as-is
// ---------------------------------------------------------------------------

describe('createFolder', () => {
  it('passes the path directly to invoke', async () => {
    invokeMock.mockResolvedValue(undefined);

    await createFolder('archive/2024');

    expect(invokeMock).toHaveBeenCalledWith('create_folder', { path: 'archive/2024' });
  });
});

// ---------------------------------------------------------------------------
// renameFile – paths passed as-is (old toVaultRelativePath calls removed)
// ---------------------------------------------------------------------------

describe('renameFile', () => {
  it('passes oldPath and newPath directly to invoke without normalization', async () => {
    invokeMock.mockResolvedValue(undefined);

    await renameFile('old-name.md', 'new-name.md');

    expect(invokeMock).toHaveBeenCalledWith('rename_file', {
      oldPath: 'old-name.md',
      newPath: 'new-name.md',
    });
  });

  it('does not transform paths with backslashes (no normalization)', async () => {
    invokeMock.mockResolvedValue(undefined);

    // Windows-style paths are now forwarded verbatim.
    await renameFile('notes\\old.md', 'notes\\new.md');

    expect(invokeMock).toHaveBeenCalledWith('rename_file', {
      oldPath: 'notes\\old.md',
      newPath: 'notes\\new.md',
    });
  });
});

// ---------------------------------------------------------------------------
// deleteFile – path passed as-is
// ---------------------------------------------------------------------------

describe('deleteFile', () => {
  it('passes the path directly to invoke', async () => {
    invokeMock.mockResolvedValue(undefined);

    await deleteFile('old-note.md');

    expect(invokeMock).toHaveBeenCalledWith('delete_file', { path: 'old-note.md' });
  });
});

// ---------------------------------------------------------------------------
// duplicateFile – path passed as-is
// ---------------------------------------------------------------------------

describe('duplicateFile', () => {
  it('passes the path directly to invoke', async () => {
    invokeMock.mockResolvedValue(undefined);

    await duplicateFile('template.md');

    expect(invokeMock).toHaveBeenCalledWith('duplicate_file', { path: 'template.md' });
  });
});

// ---------------------------------------------------------------------------
// getTags – now returns whatever invoke returns (was previously Vec<String>,
// backend now returns Vec<(String, u32)>; frontend proxies the raw value)
// ---------------------------------------------------------------------------

describe('getTags', () => {
  it('calls invoke with get_tags (no args)', async () => {
    invokeMock.mockResolvedValue([['rust', 5], ['svelte', 3]]);

    await getTags();

    expect(invokeMock).toHaveBeenCalledWith('get_tags');
  });

  it('returns the raw value from invoke', async () => {
    invokeMock.mockResolvedValue([['rust', 5]]);

    const result = await getTags();
    expect(result).toEqual([['rust', 5]]);
  });

  it('returns an empty array when there are no tags', async () => {
    invokeMock.mockResolvedValue([]);
    const result = await getTags();
    expect(result).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// getRecentFiles – return type changed from RecentFile[] to FileEntry[]
// ---------------------------------------------------------------------------

describe('getRecentFiles', () => {
  it('calls invoke with get_recent_files (no args)', async () => {
    invokeMock.mockResolvedValue([]);

    await getRecentFiles();

    expect(invokeMock).toHaveBeenCalledWith('get_recent_files');
  });

  it('returns FileEntry objects (path string format from backend)', async () => {
    // The backend now returns Vec<String> (absolute paths); the frontend
    // type annotation is FileEntry[] but the actual values are plain paths.
    const backendPaths = [
      '/home/user/vault/notes/today.md',
      '/home/user/vault/notes/yesterday.md',
    ];
    invokeMock.mockResolvedValue(backendPaths);

    const result = await getRecentFiles();
    expect(result).toEqual(backendPaths);
  });

  it('returns an empty array when there are no recent files', async () => {
    invokeMock.mockResolvedValue([]);
    const result = await getRecentFiles();
    expect(result).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// moveFile – must NOT be exported (function was removed in this PR)
// ---------------------------------------------------------------------------

describe('moveFile removal', () => {
  it('moveFile is no longer exported from the module', () => {
    // Dynamically check the module exports at runtime.
    const mod = { readFile, saveFile, listFiles, createFile, createFolder,
                  renameFile, deleteFile, duplicateFile, getTags, getRecentFiles, searchFiles };
    expect((mod as Record<string, unknown>)['moveFile']).toBeUndefined();
  });
});

// ---------------------------------------------------------------------------
// searchFiles – path passed as-is (no toVaultRelativePath)
// ---------------------------------------------------------------------------

describe('searchFiles', () => {
  it('passes query and optional path directly to invoke', async () => {
    invokeMock.mockResolvedValue([]);

    await searchFiles('rust programming', 'notes/tech');

    expect(invokeMock).toHaveBeenCalledWith('search_files', {
      query: 'rust programming',
      path: 'notes/tech',
    });
  });

  it('passes undefined for path when not provided', async () => {
    invokeMock.mockResolvedValue([]);

    await searchFiles('svelte');

    expect(invokeMock).toHaveBeenCalledWith('search_files', {
      query: 'svelte',
      path: undefined,
    });
  });
});