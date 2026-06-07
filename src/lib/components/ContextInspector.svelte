<script lang="ts">
  import { getAIStore } from '../stores/ai.svelte';
  import Modal from './Modal.svelte';

  const ai = getAIStore();

  let {
    show = false,
    onClose = () => {},
  }: {
    show?: boolean;
    onClose?: () => void;
  } = $props();

  interface ContextItem {
    id: string;
    label: string;
    type: 'file' | 'selection' | 'tag' | 'template' | 'system';
    tokenCount: number;
    pinned: boolean;
    excluded: boolean;
  }

  let contextItems = $state<ContextItem[]>([
    { id: '1', label: 'Active Document', type: 'file', tokenCount: 1240, pinned: true, excluded: false },
    { id: '2', label: 'Selected Text', type: 'selection', tokenCount: 340, pinned: false, excluded: false },
    { id: '3', label: 'Recent Files (3)', type: 'file', tokenCount: 2800, pinned: false, excluded: false },
    { id: '4', label: 'Tags', type: 'tag', tokenCount: 85, pinned: true, excluded: false },
    { id: '5', label: 'System Prompt', type: 'system', tokenCount: 420, pinned: true, excluded: false },
  ]);

  let totalTokens = $derived(
    contextItems
      .filter(i => !i.excluded)
      .reduce((sum, i) => sum + i.tokenCount, 0),
  );

  function toggleExclude(id: string) {
    contextItems = contextItems.map(i =>
      i.id === id ? { ...i, excluded: !i.excluded } : i,
    );
  }

  function togglePin(id: string) {
    contextItems = contextItems.map(i =>
      i.id === id ? { ...i, pinned: !i.pinned } : i,
    );
  }

  function getTypeIcon(type: ContextItem['type']): string {
    switch (type) {
      case 'file': return '📄';
      case 'selection': return '📝';
      case 'tag': return '🏷️';
      case 'template': return '📋';
      case 'system': return '⚙️';
    }
  }
</script>

<Modal
  title="Context Inspector"
  bind:show
  size="md"
  {onClose}
>
  <div class="context-inspector">
    <div class="summary">
      <span class="text-sm text-muted">Total context: <strong>{totalTokens}</strong> tokens</span>
      <span class="text-sm text-muted">
        ({contextItems.filter(i => !i.excluded).length} items)
      </span>
    </div>

    <div class="context-list">
      {#each contextItems as item}
        <div class="context-item" class:excluded={item.excluded}>
          <div class="context-info">
            <span class="context-icon">{getTypeIcon(item.type)}</span>
            <div class="context-details">
              <span class="context-label">{item.label}</span>
              <span class="context-meta text-xs text-muted">
                {item.type} · {item.tokenCount} tokens
              </span>
            </div>
          </div>
          <div class="context-actions">
            <span class="token-badge">{item.tokenCount}</span>
            <button
              class="btn-icon btn-action"
              class:pinned={item.pinned}
              onclick={() => togglePin(item.id)}
              title={item.pinned ? 'Unpin' : 'Pin'}
            >
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
                <path d="M10 2l4 4-6 6-4-4 6-6z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" />
                <path d="M5 11l-3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
              </svg>
            </button>
            <button
              class="btn-icon btn-action"
              class:excluded={item.excluded}
              onclick={() => toggleExclude(item.id)}
              title={item.excluded ? 'Include' : 'Exclude'}
            >
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
                <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" />
                <path d="M5 8h6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
              </svg>
            </button>
          </div>
        </div>
      {/each}
    </div>

    {#if contextItems.every(i => i.excluded)}
      <div class="empty-state">
        <p class="text-muted text-sm">All items excluded. The AI will have no context.</p>
      </div>
    {/if}
  </div>
</Modal>

<style>
  .context-inspector {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .summary {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 8px 12px;
    background: var(--color-surface);
    border-radius: var(--radius-md);
  }
  .context-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .context-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    transition: background var(--transition-fast);
  }
  .context-item:hover {
    background: var(--color-surface);
  }
  .context-item.excluded {
    opacity: 0.4;
  }
  .context-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .context-icon {
    font-size: 16px;
  }
  .context-details {
    display: flex;
    flex-direction: column;
  }
  .context-label {
    font-size: 13px;
    font-weight: 500;
  }
  .context-meta {
    text-transform: capitalize;
  }
  .context-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .token-badge {
    font-size: 11px;
    font-weight: 500;
    padding: 2px 6px;
    background: var(--color-surface);
    border-radius: 999px;
    color: var(--color-text-muted);
  }
  .btn-action {
    width: 24px;
    height: 24px;
    color: var(--color-text-muted);
  }
  .btn-action:hover {
    color: var(--color-text);
    background: var(--color-surface);
  }
  .btn-action.pinned {
    color: var(--color-accent);
  }
  .btn-action.excluded {
    color: var(--color-error);
  }
  .empty-state {
    text-align: center;
    padding: 24px;
  }
</style>
