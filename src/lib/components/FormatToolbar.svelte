<script lang="ts">
  interface FormatAction {
    label: string;
    icon: string;
    action: string;
    shortcut?: string;
    active?: () => boolean;
  }

  let {
    onFormat = (_action: string) => {},
    activeFormats = new Set<string>(),
  }: {
    onFormat?: (action: string) => void;
    activeFormats?: Set<string>;
  } = $props();

  const actions: FormatAction[] = [
    { label: 'Heading 1', icon: 'H1', action: 'h1', shortcut: '⌘1' },
    { label: 'Heading 2', icon: 'H2', action: 'h2', shortcut: '⌘2' },
    { label: 'Heading 3', icon: 'H3', action: 'h3', shortcut: '⌘3' },
    { label: 'Bold', icon: 'B', action: 'bold', shortcut: '⌘B' },
    { label: 'Italic', icon: 'I', action: 'italic', shortcut: '⌘I' },
    { label: 'Strikethrough', icon: 'S', action: 'strikethrough', shortcut: '⌘⇧S' },
    { label: 'Link', icon: '🔗', action: 'link', shortcut: '⌘K' },
    { label: 'Image', icon: '🖼', action: 'image' },
    { label: 'Code', icon: '<>', action: 'code', shortcut: '⌘⇧C' },
    { label: 'Math', icon: '∑', action: 'math', shortcut: '⌘⇧M' },
    { label: 'Table', icon: '⊞', action: 'table' },
    { label: 'Bullet List', icon: '•', action: 'bulletList' },
    { label: 'Ordered List', icon: '1.', action: 'orderedList' },
    { label: 'Task List', icon: '☑', action: 'taskList' },
    { label: 'Blockquote', icon: '"', action: 'blockquote', shortcut: '⌘⇧.' },
    { label: 'Horizontal Rule', icon: '—', action: 'hr' },
    { label: 'Diagram', icon: '◇', action: 'diagram' },
  ];

  function isActive(action: string): boolean {
    return activeFormats.has(action);
  }
</script>

<div class="toolbar" role="toolbar" aria-label="Formatting toolbar">
  {#each actions as act}
    <button
      class="toolbar-btn"
      class:active={isActive(act.action)}
      onclick={() => onFormat(act.action)}
      title={act.shortcut ? `${act.label} (${act.shortcut})` : act.label}
      aria-label={act.label}
      aria-pressed={isActive(act.action)}
    >
      <span class="toolbar-icon">{act.icon}</span>
    </button>
    {#if act.action === 'h3' || act.action === 'strikethrough' || act.action === 'image' || act.action === 'table' || act.action === 'blockquote'}
      <div class="separator" />
    {/if}
  {/each}
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 1px;
    padding: 4px 8px;
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    overflow-x: auto;
    flex-wrap: nowrap;
  }
  .toolbar-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    border-radius: var(--radius-sm);
    font-size: 12px;
    font-weight: 600;
    transition: all var(--transition-fast);
    flex-shrink: 0;
  }
  .toolbar-btn:hover {
    background: color-mix(in srgb, var(--color-accent) 10%, transparent);
  }
  .toolbar-btn.active {
    background: color-mix(in srgb, var(--color-accent) 20%, transparent);
    color: var(--color-accent);
  }
  .toolbar-icon {
    line-height: 1;
  }
  .separator {
    width: 1px;
    height: 20px;
    background: var(--color-border);
    margin: 0 4px;
    flex-shrink: 0;
  }
</style>
