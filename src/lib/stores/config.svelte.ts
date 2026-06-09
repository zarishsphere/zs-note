import type {
  EditorSettings,
  VaultConfig,
  ProviderConfig,
  SandboxConfig,
  SyncConfig,
  PublishTarget,
  KnowledgeBaseInfo,
  PluginConfig,
  ImageHost,
} from '../types';
import { invoke } from '@tauri-apps/api/core';

type BackendAIConfig = {
  provider: string;
  model: string;
  api_key?: string | null;
  base_url?: string | null;
  max_tokens: number;
  temperature: number;
};

type BackendThemeConfig = {
  name: string;
  background: string;
  foreground: string;
  accent: string;
  font_family: string;
  font_size: number;
};

type BackendToolConfig = {
  name: string;
  wasmPath: string;
  permissions: string[];
  memoryLimit: number;
  timeout: number;
};

type BackendSandboxConfig = {
  enabled: boolean;
  default_memory_limit: number;
  default_timeout: number;
  allowed_domains: string[];
  max_module_size: number;
  tools: BackendToolConfig[];
};

type BackendRemoteConfig = {
  url: string;
  branch: string;
  sshKey?: string | null;
  autoSync: boolean;
  syncInterval: number;
};

type BackendGitConfig = {
  auto_commit: boolean;
  commit_style: string;
  remote?: BackendRemoteConfig | null;
};

type BackendSyncConfig = {
  autoCommit: boolean;
  commitStyle: string;
  remote?: BackendRemoteConfig | null;
};

type BackendKeyBindings = {
  save: string;
  search: string;
  command_palette: string;
  toggle_sidebar: string;
  toggle_preview: string;
};

type BackendPluginsConfig = {
  enabled: string[];
  settings: Record<string, unknown>;
};

type BackendVoiceConfig = {
  enabled: boolean;
  language: string;
  model: string;
};

type BackendKnowledgeBase = {
  name: string;
  path: string;
  formats: string[];
  indexOnStart: boolean;
};

type BackendPublishTarget = {
  name: string;
  type: string;
  repo?: string | null;
  endpoint?: string | null;
  branch?: string | null;
  keyId?: string | null;
};

type BackendVaultConfig = {
  name: string;
  path: string;
  vault_type: string;
  created_at: string;
  version: string;
};

type BackendEditorSettings = {
  theme: string;
  fontSize: number;
  fontFamily: string;
  lineHeight: number;
  proseWidth: number;
  vimMode: boolean;
  spellCheck: boolean;
  autoSave: boolean;
};

type BackendConfig = {
  version: string;
  vault: BackendVaultConfig;
  editor: BackendEditorSettings;
  ai: BackendAIConfig;
  theme: BackendThemeConfig;
  sandbox: BackendSandboxConfig;
  git: BackendGitConfig;
  sync: BackendSyncConfig;
  keys: BackendKeyBindings;
  plugins: BackendPluginsConfig;
  voice: BackendVoiceConfig;
  knowledge: BackendKnowledgeBase[];
  publish: BackendPublishTarget[];
  imageHost?: ImageHost | null;
  features: string[];
};

let editorSettings = $state<EditorSettings>({
  fontSize: 16,
  fontFamily: 'system-ui',
  lineHeight: 1.7,
  tabSize: 2,
  wordWrap: true,
  lineNumbers: true,
  vimMode: false,
  spellCheck: false,
  autoSave: true,
  autoSaveDelay: 3000,
  defaultMode: 'wysiwyg',
  theme: 'system',
  milkdownTheme: 'default',
});

let vaultConfig = $state<VaultConfig>({
  path: '',
  name: 'My Vault',
  autoIndex: true,
  watchForChanges: true,
  ignorePatterns: ['node_modules', '.git', '.obsidian'],
  maxFileSize: 10485760,
});

let providers = $state<ProviderConfig[]>([]);
let sandboxConfig = $state<SandboxConfig>({
  enabled: false,
  timeout_ms: 30000,
  memory_mb: 128,
  allowed_fs_paths: [],
  allowed_network: false,
  wasm_optimization: 'balanced',
});

let syncConfig = $state<SyncConfig>({
  enabled: false,
  type: 'git',
  intervalMinutes: 30,
  autoSync: false,
  conflictResolution: 'manual',
});

let publishTargets = $state<PublishTarget[]>([]);
let imageHost = $state<ImageHost | null>(null);

let knowledgeBases = $state<KnowledgeBaseInfo[]>([]);
let plugins = $state<PluginConfig[]>([]);

let currentConfig = $state<BackendConfig>(createDefaultBackendConfig());
let isLoading = $state(false);
let error = $state<string | null>(null);

/**
 * Create a BackendConfig populated with sensible default values for a new installation.
 *
 * @returns A BackendConfig object initialized with defaults for vault, editor, AI/provider, theme, sandbox, git/sync, keybindings, plugins, voice, knowledge bases, publish targets, optional image hosting, and enabled feature flags.
 */
function createDefaultBackendConfig(): BackendConfig {
  const now = new Date().toISOString();

  return {
    version: '1',
    vault: {
      name: 'My Vault',
      path: '.',
      vault_type: 'local',
      created_at: now,
      version: '1',
    },
    editor: {
      theme: 'zarish-light',
      fontSize: 16,
      fontFamily: 'Inter, sans-serif',
      lineHeight: 1.7,
      proseWidth: 720,
      vimMode: false,
      spellCheck: true,
      autoSave: true,
    },
    ai: {
      provider: 'ollama',
      model: 'llama3.2',
      api_key: null,
      base_url: 'http://localhost:11434',
      max_tokens: 4096,
      temperature: 0.7,
    },
    theme: {
      name: 'zarish-light',
      background: '#ffffff',
      foreground: '#1a1a2e',
      accent: '#7c3aed',
      font_family: 'Inter, sans-serif',
      font_size: 16,
    },
    sandbox: {
      enabled: true,
      default_memory_limit: 64 * 1024 * 1024,
      default_timeout: 30000,
      allowed_domains: ['*.example.com'],
      max_module_size: 10 * 1024 * 1024,
      tools: [],
    },
    git: {
      auto_commit: true,
      commit_style: 'conventional',
      remote: null,
    },
    sync: {
      autoCommit: true,
      commitStyle: 'conventional',
      remote: null,
    },
    keys: {
      save: 'ctrl+s',
      search: 'ctrl+p',
      command_palette: 'ctrl+shift+p',
      toggle_sidebar: 'ctrl+b',
      toggle_preview: 'ctrl+shift+v',
    },
    plugins: {
      enabled: [],
      settings: {},
    },
    voice: {
      enabled: false,
      language: 'en-US',
      model: 'base',
    },
    knowledge: [],
    publish: [],
    imageHost: null,
    features: ['sandbox', 'ai', 'git', 'search'],
  };
}

/**
 * Normalize an arbitrary value into a filesystem path string.
 *
 * @param path - The value to convert; if already a string it is returned unchanged.
 * @returns The input string when `path` is a string, an empty string when `path` is `null` or `undefined`, otherwise `String(path)`.
 */
function toPathString(path: unknown): string {
  if (typeof path === 'string') return path;
  return path == null ? '' : String(path);
}

/**
 * Map a backend theme identifier to the editor theme name.
 *
 * @param theme - Backend-provided theme string (e.g., "light", "zarish-dark")
 * @returns The normalized editor theme: `'light'` for recognized light variants, `'dark'` for recognized dark variants, `'system'` otherwise.
 */
function toEditorTheme(theme: string): EditorSettings['theme'] {
  if (theme === 'light' || theme === 'zarish-light') return 'light';
  if (theme === 'dark' || theme === 'zarish-dark') return 'dark';
  return 'system';
}

/**
 * Apply a BackendConfig to update all in-memory reactive configuration stores.
 *
 * Updates the module's active backend snapshot and maps the provided backend configuration into the public stores:
 * vaultConfig, editorSettings, providers, sandboxConfig, syncConfig, publishTargets, imageHost, and plugins.
 *
 * @param config - The backend configuration object to apply to the in-memory stores
 */
function applyBackendConfig(config: BackendConfig): void {
  currentConfig = config;

  vaultConfig = {
    ...vaultConfig,
    name: config.vault.name,
    path: toPathString(config.vault.path),
  };

  editorSettings = {
    ...editorSettings,
    fontSize: config.editor.fontSize,
    fontFamily: config.editor.fontFamily,
    lineHeight: config.editor.lineHeight,
    vimMode: config.editor.vimMode,
    spellCheck: config.editor.spellCheck,
    autoSave: config.editor.autoSave,
    theme: toEditorTheme(config.editor.theme),
  };

  providers = [{
    id: config.ai.provider,
    name: config.ai.provider,
    provider_type: config.ai.provider,
    api_key: config.ai.api_key ?? '',
    base_url: config.ai.base_url ?? '',
    models: config.ai.model ? [config.ai.model] : [],
    default_model: config.ai.model,
    enabled: true,
    temperature: config.ai.temperature,
    max_tokens: config.ai.max_tokens,
  }];

  sandboxConfig = {
    ...sandboxConfig,
    enabled: config.sandbox.enabled,
    timeout_ms: config.sandbox.default_timeout,
    memory_mb: Math.max(1, Math.round(config.sandbox.default_memory_limit / (1024 * 1024))),
    allowed_network: config.sandbox.allowed_domains.length > 0,
  };

  syncConfig = {
    ...syncConfig,
    enabled: config.sync.autoCommit,
    type: 'git',
    autoSync: config.sync.remote?.autoSync ?? false,
    intervalMinutes: Math.max(1, Math.round((config.sync.remote?.syncInterval ?? 1_800_000) / 60000)),
    remoteUrl: config.sync.remote?.url,
    branch: config.sync.remote?.branch,
  };

  publishTargets = config.publish.map((target, index) => ({
    id: `${target.name || 'target'}-${index}`,
    name: target.name,
    type: target.type as PublishTarget['type'],
    url: target.endpoint ?? undefined,
    branch: target.branch ?? undefined,
    remoteName: target.repo ?? undefined,
    endpoint: target.endpoint ?? undefined,
    keyId: target.keyId ?? undefined,
    uploadImages: true,
    convertWikilinks: true,
    stripPrivate: false,
    generateRss: false,
  }));

  imageHost = config.imageHost ?? null;
  plugins = config.plugins.enabled.map((id) => ({
    id,
    name: id,
    version: 'unknown',
    enabled: true,
    source: 'local',
    config: config.plugins.settings[id] as Record<string, unknown> | undefined,
  }));
}

/**
 * Builds a BackendConfig snapshot from the current in-memory config stores for persistence.
 *
 * The returned config merges the existing `currentConfig` with values taken from the UI-facing
 * stores (vaultConfig, editorSettings, providers, sandboxConfig, syncConfig, publishTargets,
 * imageHost, and plugins), producing a backend-ready configuration object that reflects the
 * current application state.
 *
 * @returns The assembled BackendConfig ready to be persisted to the backend.
 */
function buildConfigForSave(): BackendConfig {
  const provider = providers.find((item) => item.enabled) ?? providers[0];
  const remote = syncConfig.remoteUrl || syncConfig.branch || syncConfig.autoSync
    ? {
        url: syncConfig.remoteUrl ?? currentConfig.sync.remote?.url ?? '',
        branch: syncConfig.branch ?? currentConfig.sync.remote?.branch ?? 'main',
        sshKey: currentConfig.sync.remote?.sshKey ?? null,
        autoSync: syncConfig.autoSync,
        syncInterval: syncConfig.intervalMinutes * 60_000,
      }
    : null;

  return {
    ...currentConfig,
    vault: {
      ...currentConfig.vault,
      name: vaultConfig.name,
      path: vaultConfig.path || '.',
    },
    editor: {
      ...currentConfig.editor,
      theme: editorSettings.theme,
      fontSize: editorSettings.fontSize,
      fontFamily: editorSettings.fontFamily,
      lineHeight: editorSettings.lineHeight,
      vimMode: editorSettings.vimMode,
      spellCheck: editorSettings.spellCheck,
      autoSave: editorSettings.autoSave,
    },
    ai: provider
      ? {
          ...currentConfig.ai,
          provider: provider.provider_type,
          model: provider.default_model || provider.models[0] || currentConfig.ai.model,
          api_key: provider.api_key || null,
          base_url: provider.base_url || null,
          max_tokens: provider.max_tokens ?? currentConfig.ai.max_tokens,
          temperature: provider.temperature ?? currentConfig.ai.temperature,
        }
      : currentConfig.ai,
    sandbox: {
      ...currentConfig.sandbox,
      enabled: sandboxConfig.enabled,
      default_timeout: sandboxConfig.timeout_ms,
      default_memory_limit: sandboxConfig.memory_mb * 1024 * 1024,
      allowed_domains: sandboxConfig.allowed_network ? currentConfig.sandbox.allowed_domains : [],
    },
    sync: {
      ...currentConfig.sync,
      autoCommit: syncConfig.enabled,
      remote,
    },
    publish: publishTargets.map((target) => ({
      name: target.name,
      type: target.type,
      repo: target.remoteName ?? null,
      endpoint: target.endpoint ?? target.url ?? null,
      branch: target.branch ?? null,
      keyId: target.keyId ?? null,
    })),
    imageHost,
    plugins: {
      ...currentConfig.plugins,
      enabled: plugins.filter((plugin) => plugin.enabled).map((plugin) => plugin.id),
      settings: plugins.reduce<Record<string, unknown>>((settings, plugin) => {
        if (plugin.config) settings[plugin.id] = plugin.config;
        return settings;
      }, { ...currentConfig.plugins.settings }),
    },
  };
}

/**
 * Loads the persisted backend configuration and applies it to the in-memory stores.
 *
 * While loading this updates the module's loading and error state and invokes the backend
 * `get_config` command; on success the retrieved configuration is mapped into the reactive stores.
 *
 * @throws The original error thrown by the backend call when loading fails; the module `error` state
 *         is set to `Failed to load settings: <err>` before the error is re-thrown.
 */
async function loadAll(): Promise<void> {
  isLoading = true;
  error = null;

  try {
    const config = await invoke<BackendConfig>('get_config');
    applyBackendConfig(config);
  } catch (err) {
    error = `Failed to load settings: ${String(err)}`;
    throw err;
  } finally {
    isLoading = false;
  }
}

/**
 * Persists the current in-memory backend configuration to the backend and updates local state.
 *
 * On success, replaces `currentConfig` with the saved snapshot and clears `error`. On failure,
 * sets `error` to a human-readable message and rethrows the caught error.
 */
async function saveAll(): Promise<void> {
  try {
    const nextConfig = buildConfigForSave();
    await invoke('update_config', { newConfig: nextConfig });
    currentConfig = nextConfig;
    error = null;
  } catch (err) {
    error = `Failed to save settings: ${String(err)}`;
    throw err;
  }
}

/**
 * Persist current editor settings to the backend by saving the application's entire configuration.
 *
 * This triggers a full configuration save so changes to editor settings are included in the single backend update.
 */
async function saveEditorSettings(): Promise<void> {
  await saveAll();
}

/**
 * Persist the current vault configuration to the backend.
 */
async function saveVaultConfig(): Promise<void> {
  await saveAll();
}

/**
 * Persist current provider configuration to the backend.
 *
 * Saves any changes to provider settings as part of the application's backend configuration.
 */
async function saveProviders(): Promise<void> {
  await saveAll();
}

/**
 * Persist the current sandbox configuration as part of the full configuration snapshot to the backend.
 *
 * This saves sandbox settings along with related configuration sections so they are stored persistently.
 */
async function saveSandboxConfig(): Promise<void> {
  await saveAll();
}

/**
 * Persist the current sync configuration to the backend.
 *
 * Triggers a full configuration save using the unified backend update flow.
 */
async function saveSyncConfig(): Promise<void> {
  await saveAll();
}

/**
 * Persist the current publish targets to the backend configuration.
 */
async function savePublishTargets(): Promise<void> {
  await saveAll();
}

/**
 * Persists the current image host configuration to the backend as part of the full configuration snapshot.
 */
async function saveImageHost(): Promise<void> {
  await saveAll();
}

/**
 * Provides a programmatic interface to the in-memory configuration stores and related persistence actions.
 *
 * @returns An object exposing getters and setters for each reactive store slice: `editorSettings`, `vaultConfig`, `providers`, `sandboxConfig`, `syncConfig`, `publishTargets`, `imageHost`, `knowledgeBases`, and `plugins`; read-only status fields `isLoading` and `error`; and persistence methods `loadAll`, `saveAll`, and sectioned save wrappers `saveEditorSettings`, `saveVaultConfig`, `saveProviders`, `saveSandboxConfig`, `saveSyncConfig`, `savePublishTargets`, and `saveImageHost`.
 */
export function getConfigStore() {
  return {
    get editorSettings() { return editorSettings; },
    set editorSettings(v: EditorSettings) { editorSettings = v; },
    get vaultConfig() { return vaultConfig; },
    set vaultConfig(v: VaultConfig) { vaultConfig = v; },
    get providers() { return providers; },
    set providers(v: ProviderConfig[]) { providers = v; },
    get sandboxConfig() { return sandboxConfig; },
    set sandboxConfig(v: SandboxConfig) { sandboxConfig = v; },
    get syncConfig() { return syncConfig; },
    set syncConfig(v: SyncConfig) { syncConfig = v; },
    get publishTargets() { return publishTargets; },
    set publishTargets(v: PublishTarget[]) { publishTargets = v; },
    get imageHost() { return imageHost; },
    set imageHost(v: ImageHost | null) { imageHost = v; },
    get knowledgeBases() { return knowledgeBases; },
    set knowledgeBases(v: KnowledgeBaseInfo[]) { knowledgeBases = v; },
    get plugins() { return plugins; },
    set plugins(v: PluginConfig[]) { plugins = v; },
    get isLoading() { return isLoading; },
    get error() { return error; },
    loadAll,
    saveAll,
    saveEditorSettings,
    saveVaultConfig,
    saveProviders,
    saveSandboxConfig,
    saveSyncConfig,
    savePublishTargets,
    saveImageHost,
  };
}
