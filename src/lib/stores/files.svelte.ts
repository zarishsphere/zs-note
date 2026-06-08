import type { FileEntry, SearchResult } from '../types';
import * as editorCommands from '../commands/editor';

/* ── File tree state ── */
let fileTree = $state<FileEntry[]>([]);
let tags = $state<string[]>([]);
let searchQuery = $state('');
let searchResults = $state<SearchResult[]>([]);
let recentFiles = $state<FileEntry[]>([]);
let isLoading = $state(false);
let error = $state<string | null>(null);
let selectedFilePath = $state<string | null>(null);

/* ── Large vault performance state ── */
let fileTreeProgress = $state<{ loaded: number; total: number }>({ loaded: 0, total: 0 });
let isBatchUpdating = $state(false);

/* ── LRU cache for recently accessed files (max 100 entries) ── */
const LRU_MAX = 100;
const lruCache = new Map<string, { content: string; timestamp: number }>();

function lruGet(key: string): { content: string; timestamp: number } | undefined {
  if (!lruCache.has(key)) return undefined;
  const entry = lruCache.get(key)!;
  // Move to end (most recently used)
  lruCache.delete(key);
  lruCache.set(key, entry);
  return entry;
}

function lruSet(key: string, value: { content: string; timestamp: number }): void {
  if (lruCache.has(key)) {
    lruCache.delete(key);
  } else if (lruCache.size >= LRU_MAX) {
    // Delete least recently used (first item)
    const firstKey = lruCache.keys().next().value;
    if (firstKey !== undefined) {
      lruCache.delete(firstKey);
    }
  }
  lruCache.set(key, value);
}

export function getFileFromCache(path: string): string | undefined {
  return lruGet(path)?.content;
}

export function cacheFileContent(path: string, content: string): void {
  lruSet(path, { content, timestamp: Date.now() });
}

/* ── Debounced file watcher coalescing ── */
let watchTimer: ReturnType<typeof setTimeout> | undefined;
let pendingWatchPaths = new Set<string>();

function coalescedWatch(path: string): void {
  pendingWatchPaths.add(path);
  if (watchTimer) clearTimeout(watchTimer);
  watchTimer = setTimeout(() => {
    pendingWatchPaths.clear();
    loadFileTree();
  }, 500);
}

/* ── Batch file tree updates ── */
let batchTimer: ReturnType<typeof setTimeout> | undefined;
let pendingTreeUpdate: (() => void) | undefined;

function scheduleTreeUpdate(updateFn: () => void): void {
  pendingTreeUpdate = updateFn;
  isBatchUpdating = true;
  if (batchTimer) clearTimeout(batchTimer);
  batchTimer = setTimeout(() => {
    if (pendingTreeUpdate) {
      pendingTreeUpdate();
      pendingTreeUpdate = undefined;
    }
    isBatchUpdating = false;
  }, 200);
}

/* ── Core operations ── */

function loadFileTree(path?: string): void {
  isLoading = true;
  error = null;

  editorCommands.listFiles(path ?? '')
    .then((entries) => {
      scheduleTreeUpdate(() => {
        fileTree = entries;
        fileTreeProgress = { loaded: countEntries(entries), total: countEntries(entries) };
        isLoading = false;
      });
    })
    .catch((err) => {
      error = String(err);
      isLoading = false;
    });
}

/** Count total entries recursively */
function countEntries(entries: FileEntry[]): number {
  let count = 0;
  for (const entry of entries) {
    count++;
    if (entry.children) {
      count += countEntries(entry.children);
    }
  }
  return count;
}

function loadTags(): void {
  editorCommands.getTags()
    .then((t) => { tags = t; })
    .catch(() => {});
}

function loadRecentFiles(): void {
  editorCommands.getRecentFiles()
    .then((files) => { recentFiles = files; })
    .catch(() => {});
}

/** Debounced search (300ms) */
let searchTimer: ReturnType<typeof setTimeout> | undefined;

function searchFiles(query: string): void {
  searchQuery = query;
  if (searchTimer) clearTimeout(searchTimer);

  if (!query.trim()) {
    searchResults = [];
    return;
  }

  searchTimer = setTimeout(() => {
    editorCommands.searchFiles(query)
      .then((results) => { searchResults = results; })
      .catch(() => { searchResults = []; });
  }, 300);
}

function createFile(path: string): Promise<void> {
  return editorCommands.createFile(path)
    .then(() => { coalescedWatch(path); })
    .catch((err) => { error = String(err); throw err; });
}

function createFolder(path: string): Promise<void> {
  return editorCommands.createFolder(path)
    .then(() => { coalescedWatch(path); })
    .catch((err) => { error = String(err); throw err; });
}

function deleteFileEntry(path: string): Promise<void> {
  return editorCommands.deleteFile(path)
    .then(() => { coalescedWatch(path); })
    .catch((err) => { error = String(err); throw err; });
}

function renameFileEntry(oldPath: string, newPath: string): Promise<void> {
  return editorCommands.renameFile(oldPath, newPath)
    .then(() => { coalescedWatch(newPath); })
    .catch((err) => { error = String(err); throw err; });
}

function duplicateFileEntry(path: string): Promise<void> {
  return editorCommands.duplicateFile(path)
    .then(() => { coalescedWatch(path); })
    .catch((err) => { error = String(err); throw err; });
}

function selectFile(path: string): void {
  selectedFilePath = path;
}

export function getFilesStore() {
  return {
    get fileTree() { return fileTree; },
    get tags() { return tags; },
    get searchQuery() { return searchQuery; },
    get searchResults() { return searchResults; },
    get recentFiles() { return recentFiles; },
    get isLoading() { return isLoading; },
    get error() { return error; },
    get selectedFilePath() { return selectedFilePath; },
    get fileTreeProgress() { return fileTreeProgress; },
    get isBatchUpdating() { return isBatchUpdating; },
    loadFileTree,
    loadTags,
    loadRecentFiles,
    searchFiles,
    createFile,
    createFolder,
    deleteFile: deleteFileEntry,
    renameFile: renameFileEntry,
    duplicateFile: duplicateFileEntry,
    selectFile,
  };
}
