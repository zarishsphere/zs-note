import { invoke } from '@tauri-apps/api/core';
import type { FileEntry, RecentFile, SearchResult, Template } from '../types';

export async function readFile(path: string): Promise<string> {
  return invoke('read_file', { path });
}

export async function saveFile(path: string, content: string): Promise<void> {
  return invoke('save_file', { path, content });
}

export async function listFiles(path: string): Promise<FileEntry[]> {
  return invoke('list_files', { path });
}

export async function createFile(path: string): Promise<void> {
  return invoke('create_file', { path });
}

export async function createFolder(path: string): Promise<void> {
  return invoke('create_folder', { path });
}

export async function renameFile(oldPath: string, newPath: string): Promise<void> {
  return invoke('rename_file', { oldPath, newPath });
}

export async function deleteFile(path: string): Promise<void> {
  return invoke('delete_file', { path });
}

export async function duplicateFile(path: string): Promise<void> {
  return invoke('duplicate_file', { path });
}

export async function getTags(): Promise<string[]> {
  return invoke('get_tags');
}

export async function getRecentFiles(): Promise<RecentFile[]> {
  return invoke('get_recent_files');
}

export async function searchFiles(query: string, path?: string): Promise<SearchResult[]> {
  return invoke('search_files', { query, path });
}

export async function getTemplates(): Promise<Template[]> {
  return invoke('get_templates');
}
