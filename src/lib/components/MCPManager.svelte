<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { McpServerInfo } from '../types';

  let {
    servers = [] as McpServerInfo[],
    onUpdate = () => {},
  }: {
    servers?: McpServerInfo[];
    onUpdate?: () => void;
  } = $props();

  let showAddForm = $state(false);
  let newServer = $state<Partial<McpServerInfo>>({
    name: '',
    transport: 'stdio',
    command: '',
    args: [],
    url: '',
    env: {},
    enabled: true,
  });
  let testingId = $state<string | null>(null);
  let error = $state<string | null>(null);

  function handleToggle(id: string, enabled: boolean) {
    invoke('mcp_toggle_server', { id, enabled })
      .then(() => onUpdate())
      .catch((err) => { error = String(err); });
  }

  function handleTest(id: string) {
    testingId = id;
    invoke<boolean>('mcp_test_connection', { id })
      .then((ok) => {
        if (ok) {
          onUpdate();
        }
      })
      .catch((err) => { error = String(err); })
      .finally(() => { testingId = null; });
  }

  function handleRemove(id: string) {
    if (!confirm('Remove this MCP server?')) return;
    invoke('mcp_remove_server', { id })
      .then(() => onUpdate())
      .catch((err) => { error = String(err); });
  }

  function handleAddServer() {
    if (!newServer.name || (!newServer.command && !newServer.url)) return;

    invoke('mcp_add_server', { server: newServer as McpServerInfo })
      .then(() => {
        showAddForm = false;
        newServer = { name: '', transport: 'stdio', command: '', args: [], url: '', env: {}, enabled: true };
        onUpdate();
      })
      .catch((err) => { error = String(err); });
  }

  function addEnvVar() {
    if (!newServer.env) newServer.env = {};
    const key = prompt('Environment variable name:');
    if (!key) return;
    const value = prompt(`Value for ${key}:`);
    if (value !== null) {
      newServer.env = { ...newServer.env, [key]: value };
    }
  }

  function removeEnvVar(key: string) {
    if (!newServer.env) return;
    const next = { ...newServer.env };
    delete next[key];
    newServer.env = next;
  }

  function getStatusClass(status: string): string {
    switch (status) {
      case 'connected': return 'badge-success';
      case 'disconnected': return 'badge-warning';
      case 'error': return 'badge-error';
      default: return '';
    }
  }
</script>

<div class="mcp-manager">
  <div class="manager-header">
    <h3 class="text-sm font-bold">MCP Servers</h3>
    <button class="btn btn-primary btn-sm" onclick={() => { showAddForm = !showAddForm; }}>
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
        <path d="M8 3v10M3 8h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
      </svg>
      Add Server
    </button>
  </div>

  {#if error}
    <div class="error-banner">
      <span class="text-sm" style="color: var(--color-error)">{error}</span>
      <button class="btn btn-ghost btn-icon" onclick={() => { error = null; }}>×</button>
    </div>
  {/if}

  {#if showAddForm}
    <div class="add-form">
      <div class="input-group">
        <label>Name</label>
        <input type="text" bind:value={newServer.name} placeholder="My MCP Server" />
      </div>
      <div class="input-group">
        <label>Transport</label>
        <select bind:value={newServer.transport}>
          <option value="stdio">STDIO</option>
          <option value="sse">SSE (HTTP)</option>
        </select>
      </div>
      {#if newServer.transport === 'stdio'}
        <div class="input-group">
          <label>Command</label>
          <input type="text" bind:value={newServer.command} placeholder="npx, python, node..." />
        </div>
        <div class="input-group">
          <label>Arguments (comma-separated)</label>
          <input
            type="text"
            value={newServer.args?.join(', ') ?? ''}
            oninput={(e) => {
              newServer.args = (e.target as HTMLInputElement).value.split(',').map(s => s.trim()).filter(Boolean);
            }}
            placeholder="arg1, arg2"
          />
        </div>
      {:else}
        <div class="input-group">
          <label>URL</label>
          <input type="text" bind:value={newServer.url} placeholder="https://example.com/sse" />
        </div>
      {/if}

      <div class="input-group">
        <label>
          Environment Variables
          <button class="btn btn-ghost btn-xs" onclick={addEnvVar}>+ Add</button>
        </label>
        {#if newServer.env && Object.keys(newServer.env).length > 0}
          <div class="env-list">
            {#each Object.entries(newServer.env!) as [key, value]}
              <div class="env-item">
                <span class="text-sm truncate">{key}={value}</span>
                <button class="btn btn-ghost btn-icon" onclick={() => removeEnvVar(key)}>×</button>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="form-actions">
        <button class="btn btn-secondary" onclick={() => { showAddForm = false; }}>Cancel</button>
        <button class="btn btn-primary" onclick={handleAddServer}>Add Server</button>
      </div>
    </div>
  {/if}

  <div class="server-list">
    {#if servers.length === 0}
      <div class="empty-state text-muted text-sm">
        No MCP servers configured. Add one to extend AI capabilities.
      </div>
    {:else}
      {#each servers as server}
        <div class="server-card" class:disabled={!server.enabled}>
          <div class="server-info">
            <div class="server-name">{server.name}</div>
            <span class="badge {getStatusClass(server.status)}">{server.status}</span>
          </div>
          <div class="server-meta text-xs text-muted">
            {server.transport} · {server.tools?.length ?? 0} tools
          </div>
          {#if server.errorMessage}
            <div class="error-message text-xs">{server.errorMessage}</div>
          {/if}
          <div class="server-actions">
            <label class="toggle-label">
              <input
                type="checkbox"
                checked={server.enabled}
                onchange={(e) => handleToggle(server.id, (e.target as HTMLInputElement).checked)}
              />
              <span class="toggle-slider" />
            </label>
            <button
              class="btn btn-ghost btn-sm"
              onclick={() => handleTest(server.id)}
              disabled={testingId === server.id}
            >
              {testingId === server.id ? 'Testing...' : 'Test'}
            </button>
            <button class="btn btn-ghost btn-sm danger" onclick={() => handleRemove(server.id)}>Remove</button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .mcp-manager {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .manager-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .error-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 10px;
    background: var(--color-error-bg);
    border-radius: var(--radius-md);
  }
  .add-form {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
  }
  .btn-xs {
    font-size: 11px;
    padding: 2px 6px;
  }
  .env-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: 4px;
  }
  .env-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 8px;
    background: var(--color-surface);
    border-radius: var(--radius-sm);
    font-size: 12px;
    font-family: var(--font-code);
  }
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding-top: 8px;
    border-top: 1px solid var(--color-border);
  }
  .server-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .empty-state {
    text-align: center;
    padding: 24px;
  }
  .server-card {
    padding: 10px 12px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .server-card.disabled {
    opacity: 0.6;
  }
  .server-info {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .server-name {
    font-weight: 500;
    font-size: 13px;
  }
  .server-meta {
    font-family: var(--font-code);
  }
  .error-message {
    padding: 4px 8px;
    background: var(--color-error-bg);
    border-radius: var(--radius-sm);
    color: var(--color-error);
  }
  .server-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    padding-top: 6px;
    border-top: 1px solid var(--color-border);
  }
  .server-actions .danger {
    color: var(--color-error);
    margin-left: auto;
  }
  .toggle-label {
    position: relative;
    display: inline-block;
    width: 32px;
    height: 18px;
    cursor: pointer;
  }
  .toggle-label input {
    opacity: 0;
    width: 0;
    height: 0;
  }
  .toggle-slider {
    position: absolute;
    inset: 0;
    background: var(--color-border);
    border-radius: 18px;
    transition: background var(--transition-fast);
  }
  .toggle-slider::before {
    content: '';
    position: absolute;
    width: 14px;
    height: 14px;
    left: 2px;
    bottom: 2px;
    background: white;
    border-radius: 50%;
    transition: transform var(--transition-fast);
  }
  .toggle-label input:checked + .toggle-slider {
    background: var(--color-accent);
  }
  .toggle-label input:checked + .toggle-slider::before {
    transform: translateX(14px);
  }
</style>
