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

let isLoading = $state(false);
let error = $state<string | null>(null);

async function loadAll(): Promise<void> {
  isLoading = true;
  error = null;

  try {
    const [settings, vault, prov, sandbox, sync, pub, img, kb, pl] = await Promise.all([
      invoke<EditorSettings>('get_editor_settings').catch(() => editorSettings),
      invoke<VaultConfig>('get_config').catch(() => vaultConfig),
      invoke<ProviderConfig[]>('get_providers').catch(() => providers),
      invoke<SandboxConfig>('get_sandbox_config').catch(() => sandboxConfig),
      invoke<SyncConfig>('get_sync_config').catch(() => syncConfig),
      invoke<PublishTarget[]>('get_publish_targets').catch(() => publishTargets),
      invoke<ImageHost | null>('get_image_host').catch(() => imageHost),
      invoke<KnowledgeBaseInfo[]>('kb_list').catch(() => knowledgeBases),
      invoke<PluginConfig[]>('plugin_list').catch(() => plugins),
    ]);

    editorSettings = settings;
    vaultConfig = vault;
    providers = prov;
    sandboxConfig = sandbox;
    syncConfig = sync;
    publishTargets = pub;
    imageHost = img;
    knowledgeBases = kb;
    plugins = pl;
  } catch (err) {
    error = String(err);
  } finally {
    isLoading = false;
  }
}

async function saveEditorSettings(): Promise<void> {
  try {
    await invoke('save_editor_settings', { settings: editorSettings });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

async function saveVaultConfig(): Promise<void> {
  try {
    await invoke('save_config', { config: vaultConfig });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

async function saveProviders(): Promise<void> {
  try {
    await invoke('save_providers', { providers });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

async function saveSandboxConfig(): Promise<void> {
  try {
    await invoke('save_sandbox_config', { config: sandboxConfig });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

async function saveSyncConfig(): Promise<void> {
  try {
    await invoke('save_sync_config', { config: syncConfig });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

async function savePublishTargets(): Promise<void> {
  try {
    await invoke('save_publish_targets', { targets: publishTargets });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

async function saveImageHost(): Promise<void> {
  try {
    await invoke('save_image_host', { host: imageHost });
  } catch (err) {
    error = String(err);
    throw err;
  }
}

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
    saveEditorSettings,
    saveVaultConfig,
    saveProviders,
    saveSandboxConfig,
    saveSyncConfig,
    savePublishTargets,
    saveImageHost,
  };
}
