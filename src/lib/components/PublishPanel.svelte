<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { getEditorStore } from '../stores/editor';
  import { getConfigStore } from '../stores/config';
  import type { PublishTarget } from '../types';

  const editor = getEditorStore();
  const config = getConfigStore();

  let {
    onClose = () => {},
  }: {
    onClose?: () => void;
  } = $props();

  let selectedTargetId = $state('');
  let publishOptions = $state({
    uploadImages: true,
    convertWikilinks: true,
    stripPrivate: false,
    generateRss: false,
  });
  let preview = $state('');
  let isPublishing = $state(false);
  let isPreviewLoading = $state(false);
  let error = $state<string | null>(null);
  let success = $state<string | null>(null);

  let selectedTarget = $derived(
    config.publishTargets.find(t => t.id === selectedTargetId),
  );

  $effect(() => {
    if (selectedTarget) {
      publishOptions = {
        uploadImages: selectedTarget.uploadImages,
        convertWikilinks: selectedTarget.convertWikilinks,
        stripPrivate: selectedTarget.stripPrivate,
        generateRss: selectedTarget.generateRss,
      };
    }
  });

  function loadPreview() {
    if (!editor.activeFilePath) return;
    isPreviewLoading = true;
    error = null;

    invoke<string>('publish_preview', { filePath: editor.activeFilePath })
      .then((p) => { preview = p; })
      .catch((err) => { error = String(err); })
      .finally(() => { isPreviewLoading = false; });
  }

  function handlePublish() {
    if (!selectedTarget || !editor.activeFilePath) return;

    isPublishing = true;
    error = null;
    success = null;

    invoke('publish_now', {
      target: selectedTarget,
      filePath: editor.activeFilePath,
      options: publishOptions,
    })
      .then(() => {
        success = `Published to ${selectedTarget.name}`;
      })
      .catch((err) => { error = String(err); })
      .finally(() => { isPublishing = false; });
  }
</script>

<div class="publish-panel">
  <h3 class="panel-title">Publish</h3>

  <!-- File Info -->
  <div class="file-info">
    <span class="text-sm text-muted">File:</span>
    <span class="text-sm truncate">{editor.activeFilePath || 'No file open'}</span>
  </div>

  <!-- Target Selection -->
  <div class="input-group">
    <label>Target</label>
    <select bind:value={selectedTargetId}>
      <option value="">Select a publish target...</option>
      {#each config.publishTargets as target}
        <option value={target.id}>{target.name} ({target.type})</option>
      {/each}
    </select>
  </div>

  {#if selectedTarget}
    <!-- Options -->
    <div class="options-section">
      <h4 class="options-title text-sm font-bold">Options</h4>
      <label class="option-row">
        <input type="checkbox" bind:checked={publishOptions.uploadImages} />
        <span>Upload images</span>
      </label>
      <label class="option-row">
        <input type="checkbox" bind:checked={publishOptions.convertWikilinks} />
        <span>Convert [[wikilinks]]</span>
      </label>
      <label class="option-row">
        <input type="checkbox" bind:checked={publishOptions.stripPrivate} />
        <span>Strip private tags</span>
      </label>
      <label class="option-row">
        <input type="checkbox" bind:checked={publishOptions.generateRss} />
        <span>Generate RSS feed</span>
      </label>
    </div>
  {/if}

  <!-- Preview -->
  <div class="preview-section">
    <button class="btn btn-secondary btn-sm" onclick={loadPreview} disabled={!editor.activeFilePath}>
      {isPreviewLoading ? 'Loading...' : 'Preview Changes'}
    </button>
    {#if preview}
      <div class="preview-content">
        <pre class="preview-text">{preview}</pre>
      </div>
    {/if}
  </div>

  {#if error}
    <div class="message error-message">
      <span class="text-sm" style="color: var(--color-error)">{error}</span>
    </div>
  {/if}

  {#if success}
    <div class="message success-message">
      <span class="text-sm" style="color: var(--color-success)">{success}</span>
    </div>
  {/if}

  <!-- Actions -->
  <div class="actions">
    <button class="btn btn-secondary" onclick={onClose}>Cancel</button>
    <button
      class="btn btn-primary"
      onclick={handlePublish}
      disabled={!selectedTarget || !editor.activeFilePath || isPublishing}
    >
      {isPublishing ? 'Publishing...' : 'Publish Now'}
    </button>
  </div>
</div>

<style>
  .publish-panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .panel-title {
    font-size: 16px;
    font-weight: 600;
  }
  .file-info {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 8px;
    background: var(--color-surface);
    border-radius: var(--radius-md);
  }
  .options-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }
  .options-title {
    margin-bottom: 4px;
  }
  .option-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    cursor: pointer;
  }
  .option-row input[type="checkbox"] {
    width: 14px;
    height: 14px;
  }
  .preview-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .preview-content {
    max-height: 200px;
    overflow-y: auto;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }
  .preview-text {
    font-size: 12px;
    padding: 8px;
    white-space: pre-wrap;
    word-break: break-all;
  }
  .message {
    padding: 6px 10px;
    border-radius: var(--radius-md);
  }
  .error-message {
    background: var(--color-error-bg);
  }
  .success-message {
    background: var(--color-success-bg);
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding-top: 8px;
    border-top: 1px solid var(--color-border);
  }
</style>
