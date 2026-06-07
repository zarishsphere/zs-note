import type { FileEntry, SearchResult } from '../types';
import * as editorCommands from '../commands/editor';

let fileTree = $state<FileEntry[]>([]);
let tags = $state<string[]>([]);
let searchQuery = $state('');
let searchResults = $state<SearchResult[]>([]);
let recentFiles = $state<FileEntry[]>([]);
let isLoading = $state(false);
let error = $state<string | null>(null);
let selectedFilePath = $state<string | null>(null);

function loadFileTree(path?: string): void {
  isLoading = true;
  error = null;

  editorCommands.listFiles(path ?? '')
    .then((entries) => {
      fileTree = entries;
      isLoading = false;
    })
    .catch((err) => {
      error = String(err);
      isLoading = false;
    });
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

function searchFiles(query: string): void {
  searchQuery = query;
  if (!query.trim()) {
    searchResults = [];
    return;
  }

  editorCommands.searchFiles(query)
    .then((results) => { searchResults = results; })
    .catch(() => { searchResults = []; });
}

function createFile(path: string): Promise<void> {
  return editorCommands.createFile(path)
    .then(() => loadFileTree())
    .catch((err) => { error = String(err); throw err; });
}

function createFolder(path: string): Promise<void> {
  return editorCommands.createFolder(path)
    .then(() => loadFileTree())
    .catch((err) => { error = String(err); throw err; });
}

function deleteFileEntry(path: string): Promise<void> {
  return editorCommands.deleteFile(path)
    .then(() => loadFileTree())
    .catch((err) => { error = String(err); throw err; });
}

function renameFileEntry(oldPath: string, newPath: string): Promise<void> {
  return editorCommands.renameFile(oldPath, newPath)
    .then(() => loadFileTree())
    .catch((err) => { error = String(err); throw err; });
}

function duplicateFileEntry(path: string): Promise<void> {
  return editorCommands.duplicateFile(path)
    .then(() => loadFileTree())
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
