import type { FileEntry } from '../types';
import * as editorCommands from '../commands/editor';

let activeFilePath = $state<string>('');
let content = $state<string>('');
let previousContent = $state<string>('');
let editorMode = $state<'wysiwyg' | 'source' | 'split'>('wysiwyg');
let cursorPosition = $state<{ line: number; col: number }>({ line: 1, col: 1 });
let isLoading = $state(false);
let error = $state<string | null>(null);

const isDirty = $derived(content !== previousContent);

function openFile(path: string): void {
  isLoading = true;
  error = null;

  editorCommands.readFile(path)
    .then((fileContent) => {
      content = fileContent;
      previousContent = fileContent;
      activeFilePath = path;
      isLoading = false;
    })
    .catch((err) => {
      error = String(err);
      isLoading = false;
    });
}

function saveFile(): Promise<void> {
  if (!activeFilePath) return Promise.resolve();
  error = null;

  return editorCommands.saveFile(activeFilePath, content)
    .then(() => {
      previousContent = content;
    })
    .catch((err) => {
      error = String(err);
      throw err;
    });
}

function setMode(mode: 'wysiwyg' | 'source' | 'split'): void {
  editorMode = mode;
}

function updateContent(newContent: string): void {
  content = newContent;
}

function setCursorPosition(line: number, col: number): void {
  cursorPosition = { line, col };
}

function closeFile(): void {
  activeFilePath = '';
  content = '';
  previousContent = '';
}

export function getEditorStore() {
  return {
    get activeFilePath() { return activeFilePath; },
    get content() { return content; },
    get editorMode() { return editorMode; },
    get cursorPosition() { return cursorPosition; },
    get isDirty() { return isDirty; },
    get isLoading() { return isLoading; },
    get error() { return error; },
    openFile,
    saveFile,
    setMode,
    updateContent,
    setCursorPosition,
    closeFile,
  };
}
