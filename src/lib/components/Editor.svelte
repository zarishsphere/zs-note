<script lang="ts">
  import { getEditorStore } from '../stores/editor';
  import MilkdownEditor from './MilkdownEditor.svelte';
  import SourceEditor from './SourceEditor.svelte';

  const editor = getEditorStore();

  let {
    content = '',
    filePath = '',
    mode = 'wysiwyg' as 'wysiwyg' | 'source' | 'split',
    onContentChange = (_content: string) => {},
    onSave = () => {},
  }: {
    content?: string;
    filePath?: string;
    mode?: 'wysiwyg' | 'source' | 'split';
    onContentChange?: (content: string) => void;
    onSave?: () => void;
  } = $props();

  let showNoFile = $derived(!filePath && !content);
  let splitRatio = $state(50);

  function handleContentChange(newContent: string) {
    editor.updateContent(newContent);
    onContentChange(newContent);
  }

  function handleSave() {
    editor.saveFile().then(() => onSave());
  }

  function handleSplitDrag(e: MouseEvent) {
    e.preventDefault();
    const container = (e.target as HTMLElement).parentElement;
    if (!container) return;
    const rect = container.getBoundingClientRect();

    function onMove(ev: MouseEvent) {
      const pct = ((ev.clientX - rect.left) / rect.width) * 100;
      splitRatio = Math.max(20, Math.min(80, pct));
    }

    function onUp() {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    }

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  }
</script>

<div class="editor-viewport">
  {#if showNoFile}
    <div class="empty-editor">
      <div class="empty-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <line x1="12" y1="18" x2="12" y2="12" />
          <line x1="9" y1="15" x2="15" y2="15" />
        </svg>
      </div>
      <h2 class="empty-title">No file open</h2>
      <p class="empty-description text-muted">
        Select a file from the sidebar or create a new note to get started.
      </p>
    </div>
  {:else if mode === 'wysiwyg'}
    <MilkdownEditor
      content={editor.content}
      onChange={handleContentChange}
      onSave={handleSave}
    />
  {:else if mode === 'source'}
    <SourceEditor
      onContentChange={handleContentChange}
      onSave={handleSave}
    />
  {:else if mode === 'split'}
    <div class="split-container">
      <div class="split-pane source-pane" style="width: {splitRatio}%">
        <div class="pane-label">Source</div>
        <SourceEditor
          onContentChange={handleContentChange}
          onSave={handleSave}
        />
      </div>
      <div
        class="split-divider"
        role="separator"
        tabindex="0"
        onmousedown={handleSplitDrag}
        aria-label="Resize editor panes"
      >
        <div class="divider-handle" />
      </div>
      <div class="split-pane preview-pane" style="flex: 1">
        <div class="pane-label">Preview</div>
        <MilkdownEditor
          content={editor.content}
          readOnly={true}
          onChange={() => {}}
          onSave={() => {}}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  .editor-viewport {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-bg);
  }
  .empty-editor {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 32px;
  }
  .empty-icon {
    color: var(--color-text-muted);
    opacity: 0.4;
  }
  .empty-title {
    font-size: 18px;
    font-weight: 600;
  }
  .empty-description {
    font-size: 14px;
    text-align: center;
    max-width: 320px;
  }
  .split-container {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  .split-pane {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }
  .source-pane {
    border-right: 1px solid var(--color-border);
  }
  .pane-label {
    position: absolute;
    top: 4px;
    right: 8px;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
    z-index: 1;
    padding: 2px 6px;
    background: var(--color-surface);
    border-radius: var(--radius-sm);
    opacity: 0.7;
    pointer-events: none;
  }
  .split-divider {
    width: 4px;
    cursor: col-resize;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    position: relative;
    z-index: 2;
  }
  .split-divider:hover, .split-divider:focus-visible {
    background: color-mix(in srgb, var(--color-accent) 20%, transparent);
  }
  .divider-handle {
    width: 2px;
    height: 32px;
    border-radius: 1px;
    background: var(--color-border);
  }
  .split-divider:hover .divider-handle {
    background: var(--color-accent);
  }
</style>
