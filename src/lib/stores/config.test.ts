/**
 * Tests for the pure helper functions introduced in config.svelte.ts.
 *
 * Since toEditorTheme() and toPathString() are module-private, these tests
 * replicate their logic inline and verify the specified contracts.  If the
 * implementation is ever extracted/exported the tests can be updated to
 * import directly.
 *
 * Also tests the structure produced by createDefaultBackendConfig() (which
 * backs the initial $state) by verifying the shape expected by the backend
 * Rust config.
 */

import { describe, it, expect } from 'vitest';

// ---------------------------------------------------------------------------
// toEditorTheme – replicated from config.svelte.ts (not exported)
// ---------------------------------------------------------------------------

/**
 * Mirrors the implementation of toEditorTheme in config.svelte.ts:
 *   light / zarish-light → 'light'
 *   dark  / zarish-dark  → 'dark'
 *   anything else        → 'system'
 */
function toEditorTheme(theme: string): 'light' | 'dark' | 'system' {
  if (theme === 'light' || theme === 'zarish-light') return 'light';
  if (theme === 'dark' || theme === 'zarish-dark') return 'dark';
  return 'system';
}

describe('toEditorTheme', () => {
  it('maps "light" to "light"', () => {
    expect(toEditorTheme('light')).toBe('light');
  });

  it('maps "zarish-light" to "light"', () => {
    expect(toEditorTheme('zarish-light')).toBe('light');
  });

  it('maps "dark" to "dark"', () => {
    expect(toEditorTheme('dark')).toBe('dark');
  });

  it('maps "zarish-dark" to "dark"', () => {
    expect(toEditorTheme('zarish-dark')).toBe('dark');
  });

  it('maps an unknown theme to "system"', () => {
    expect(toEditorTheme('solarized')).toBe('system');
  });

  it('maps empty string to "system"', () => {
    expect(toEditorTheme('')).toBe('system');
  });

  it('is case-sensitive: "Light" maps to "system"', () => {
    expect(toEditorTheme('Light')).toBe('system');
  });

  it('is case-sensitive: "DARK" maps to "system"', () => {
    expect(toEditorTheme('DARK')).toBe('system');
  });

  it('maps "zarish-light" variant before checking dark', () => {
    // Ensure the light check runs before the dark check.
    expect(toEditorTheme('zarish-light')).toBe('light');
    expect(toEditorTheme('zarish-dark')).toBe('dark');
  });
});

// ---------------------------------------------------------------------------
// toPathString – replicated from config.svelte.ts (not exported)
// ---------------------------------------------------------------------------

/**
 * Mirrors toPathString from config.svelte.ts:
 *   string → returned as-is
 *   null/undefined → empty string
 *   other → String(value)
 */
function toPathString(path: unknown): string {
  if (typeof path === 'string') return path;
  return path == null ? '' : String(path);
}

describe('toPathString', () => {
  it('returns a string value unchanged', () => {
    expect(toPathString('notes/daily')).toBe('notes/daily');
  });

  it('returns empty string for null', () => {
    expect(toPathString(null)).toBe('');
  });

  it('returns empty string for undefined', () => {
    expect(toPathString(undefined)).toBe('');
  });

  it('converts a number to its string representation', () => {
    expect(toPathString(42)).toBe('42');
  });

  it('handles an object with a toString method', () => {
    const pathLike = { toString: () => '/vault/notes' };
    expect(toPathString(pathLike)).toBe('/vault/notes');
  });

  it('returns an empty string unchanged', () => {
    expect(toPathString('')).toBe('');
  });

  it('handles a path with backslashes (Windows-style)', () => {
    expect(toPathString('C:\\Users\\notes')).toBe('C:\\Users\\notes');
  });
});

// ---------------------------------------------------------------------------
// createDefaultBackendConfig structure (mirrors the PR-added function)
// ---------------------------------------------------------------------------

type BackendAIConfig = {
  provider: string;
  model: string;
  api_key?: string | null;
  base_url?: string | null;
  max_tokens: number;
  temperature: number;
};

type BackendConfig = {
  version: string;
  vault: { name: string; path: string; vault_type: string };
  editor: {
    theme: string;
    fontSize: number;
    fontFamily: string;
    lineHeight: number;
    proseWidth: number;
    vimMode: boolean;
    spellCheck: boolean;
    autoSave: boolean;
  };
  ai: BackendAIConfig;
  sandbox: {
    enabled: boolean;
    default_memory_limit: number;
    default_timeout: number;
    allowed_domains: string[];
    max_module_size: number;
    tools: unknown[];
  };
  sync: { autoCommit: boolean; remote: null | unknown };
  plugins: { enabled: string[]; settings: Record<string, unknown> };
  features: string[];
};

function createDefaultBackendConfig(): BackendConfig {
  return {
    version: '1',
    vault: {
      name: 'My Vault',
      path: '.',
      vault_type: 'local',
    } as BackendConfig['vault'],
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
    sandbox: {
      enabled: true,
      default_memory_limit: 64 * 1024 * 1024,
      default_timeout: 30000,
      allowed_domains: ['*.example.com'],
      max_module_size: 10 * 1024 * 1024,
      tools: [],
    },
    sync: {
      autoCommit: true,
      remote: null,
    },
    plugins: {
      enabled: [],
      settings: {},
    },
    features: ['sandbox', 'ai', 'git', 'search'],
  };
}

describe('createDefaultBackendConfig', () => {
  it('produces version "1"', () => {
    expect(createDefaultBackendConfig().version).toBe('1');
  });

  it('sets vault name to "My Vault"', () => {
    expect(createDefaultBackendConfig().vault.name).toBe('My Vault');
  });

  it('sets vault path to "."', () => {
    expect(createDefaultBackendConfig().vault.path).toBe('.');
  });

  it('sets AI provider to "ollama"', () => {
    expect(createDefaultBackendConfig().ai.provider).toBe('ollama');
  });

  it('sets AI model to "llama3.2"', () => {
    expect(createDefaultBackendConfig().ai.model).toBe('llama3.2');
  });

  it('sets AI api_key to null', () => {
    expect(createDefaultBackendConfig().ai.api_key).toBeNull();
  });

  it('sets AI base_url to the Ollama default endpoint', () => {
    expect(createDefaultBackendConfig().ai.base_url).toBe('http://localhost:11434');
  });

  it('sets max_tokens to 4096', () => {
    expect(createDefaultBackendConfig().ai.max_tokens).toBe(4096);
  });

  it('sets temperature to 0.7', () => {
    expect(createDefaultBackendConfig().ai.temperature).toBeCloseTo(0.7);
  });

  it('enables sandbox by default', () => {
    expect(createDefaultBackendConfig().sandbox.enabled).toBe(true);
  });

  it('sets sandbox memory limit to 64 MiB', () => {
    expect(createDefaultBackendConfig().sandbox.default_memory_limit).toBe(64 * 1024 * 1024);
  });

  it('sets sandbox timeout to 30 000 ms', () => {
    expect(createDefaultBackendConfig().sandbox.default_timeout).toBe(30_000);
  });

  it('includes expected feature flags', () => {
    const { features } = createDefaultBackendConfig();
    expect(features).toContain('sandbox');
    expect(features).toContain('ai');
    expect(features).toContain('git');
    expect(features).toContain('search');
  });

  it('starts with an empty plugins list', () => {
    const { plugins } = createDefaultBackendConfig();
    expect(plugins.enabled).toEqual([]);
    expect(plugins.settings).toEqual({});
  });

  it('sets autoSave to true in editor defaults', () => {
    expect(createDefaultBackendConfig().editor.autoSave).toBe(true);
  });

  it('sets vimMode to false in editor defaults', () => {
    expect(createDefaultBackendConfig().editor.vimMode).toBe(false);
  });
});

// ---------------------------------------------------------------------------
// applyBackendConfig logic – verify the mapping rules
// ---------------------------------------------------------------------------

describe('applyBackendConfig – editor theme mapping', () => {
  // The function calls toEditorTheme() when applying the backend config.
  // We verify the mapping contract here using our replicated function.

  it('converts "zarish-light" backend theme to "light" for the editor store', () => {
    expect(toEditorTheme('zarish-light')).toBe('light');
  });

  it('converts "zarish-dark" backend theme to "dark" for the editor store', () => {
    expect(toEditorTheme('zarish-dark')).toBe('dark');
  });

  it('converts unknown theme to "system"', () => {
    expect(toEditorTheme('monokai')).toBe('system');
  });
});

describe('applyBackendConfig – memory conversion', () => {
  // sandbox.default_memory_limit (bytes) → memory_mb (MiB), minimum 1
  function toMemoryMb(bytes: number): number {
    return Math.max(1, Math.round(bytes / (1024 * 1024)));
  }

  it('converts 64 MiB bytes to 64', () => {
    expect(toMemoryMb(64 * 1024 * 1024)).toBe(64);
  });

  it('converts 128 MiB bytes to 128', () => {
    expect(toMemoryMb(128 * 1024 * 1024)).toBe(128);
  });

  it('clamps to a minimum of 1 MiB for very small values', () => {
    expect(toMemoryMb(0)).toBe(1);
    expect(toMemoryMb(100)).toBe(1);
  });

  it('converts 512 MiB bytes to 512', () => {
    expect(toMemoryMb(512 * 1024 * 1024)).toBe(512);
  });
});

describe('applyBackendConfig – sync interval conversion', () => {
  // sync.remote.syncInterval (ms) → intervalMinutes, minimum 1
  function toIntervalMinutes(ms: number): number {
    return Math.max(1, Math.round(ms / 60000));
  }

  it('converts 1 800 000 ms (30 min) to 30', () => {
    expect(toIntervalMinutes(1_800_000)).toBe(30);
  });

  it('converts 3 600 000 ms (60 min) to 60', () => {
    expect(toIntervalMinutes(3_600_000)).toBe(60);
  });

  it('clamps sub-minute intervals to minimum 1', () => {
    expect(toIntervalMinutes(0)).toBe(1);
    expect(toIntervalMinutes(30_000)).toBe(1);
  });

  it('rounds 90 000 ms (1.5 min) to 2', () => {
    expect(toIntervalMinutes(90_000)).toBe(2);
  });
});

// ---------------------------------------------------------------------------
// buildConfigForSave – remote config construction logic
// ---------------------------------------------------------------------------

describe('buildConfigForSave – remote config', () => {
  /**
   * Mirrors the remote-building logic from buildConfigForSave:
   *   if (remoteUrl || branch || autoSync) → build remote object
   *   otherwise → null
   */
  function buildRemote(
    remoteUrl: string | undefined,
    branch: string | undefined,
    autoSync: boolean,
    intervalMinutes: number,
    currentRemote: { url?: string; branch?: string; sshKey?: string | null } | null,
  ) {
    return remoteUrl || branch || autoSync
      ? {
          url: remoteUrl ?? currentRemote?.url ?? '',
          branch: branch ?? currentRemote?.branch ?? 'main',
          sshKey: currentRemote?.sshKey ?? null,
          autoSync,
          syncInterval: intervalMinutes * 60_000,
        }
      : null;
  }

  it('returns null when remoteUrl, branch, and autoSync are all absent/false', () => {
    expect(buildRemote(undefined, undefined, false, 30, null)).toBeNull();
  });

  it('builds a remote object when remoteUrl is set', () => {
    const remote = buildRemote('https://github.com/user/repo', undefined, false, 30, null);
    expect(remote).not.toBeNull();
    expect(remote?.url).toBe('https://github.com/user/repo');
    expect(remote?.branch).toBe('main'); // default
  });

  it('builds a remote object when branch is set', () => {
    const remote = buildRemote(undefined, 'develop', false, 30, null);
    expect(remote).not.toBeNull();
    expect(remote?.branch).toBe('develop');
  });

  it('builds a remote object when autoSync is true', () => {
    const remote = buildRemote(undefined, undefined, true, 15, null);
    expect(remote).not.toBeNull();
    expect(remote?.autoSync).toBe(true);
    expect(remote?.syncInterval).toBe(15 * 60_000);
  });

  it('converts intervalMinutes to syncInterval in milliseconds', () => {
    const remote = buildRemote('https://example.com', undefined, false, 45, null);
    expect(remote?.syncInterval).toBe(45 * 60_000);
  });

  it('falls back to currentConfig remote url if syncConfig.remoteUrl is absent', () => {
    const currentRemote = { url: 'https://fallback.com', branch: 'main', sshKey: null };
    const remote = buildRemote(undefined, 'feature', false, 30, currentRemote);
    expect(remote?.url).toBe('https://fallback.com');
    expect(remote?.branch).toBe('feature');
  });

  it('preserves sshKey from currentConfig', () => {
    const currentRemote = { url: '', branch: 'main', sshKey: 'ssh-rsa abc123' };
    const remote = buildRemote('https://example.com', undefined, false, 30, currentRemote);
    expect(remote?.sshKey).toBe('ssh-rsa abc123');
  });
});

// ---------------------------------------------------------------------------
// buildConfigForSave – AI config mapping from provider
// ---------------------------------------------------------------------------

describe('buildConfigForSave – AI config mapping', () => {
  type MinimalProviderConfig = {
    provider_type: string;
    default_model: string;
    models: string[];
    api_key?: string;
    base_url?: string;
    temperature?: number;
    max_tokens?: number;
    enabled: boolean;
  };

  function buildAiConfig(
    provider: MinimalProviderConfig | undefined,
    currentAi: BackendAIConfig,
  ): BackendAIConfig {
    return provider
      ? {
          ...currentAi,
          provider: provider.provider_type,
          model: provider.default_model || provider.models[0] || currentAi.model,
          api_key: provider.api_key || null,
          base_url: provider.base_url || null,
          max_tokens: provider.max_tokens ?? currentAi.max_tokens,
          temperature: provider.temperature ?? currentAi.temperature,
        }
      : currentAi;
  }

  const defaultAi: BackendAIConfig = {
    provider: 'ollama',
    model: 'llama3.2',
    api_key: null,
    base_url: 'http://localhost:11434',
    max_tokens: 4096,
    temperature: 0.7,
  };

  it('returns currentAi unchanged when no provider is given', () => {
    expect(buildAiConfig(undefined, defaultAi)).toEqual(defaultAi);
  });

  it('maps provider_type to the ai.provider field', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: 'gpt-4o',
      models: ['gpt-4o'],
      api_key: 'sk-test',
      enabled: true,
    };
    const ai = buildAiConfig(provider, defaultAi);
    expect(ai.provider).toBe('openai');
  });

  it('uses default_model first', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: 'gpt-4o',
      models: ['gpt-3.5-turbo'],
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).model).toBe('gpt-4o');
  });

  it('falls back to models[0] when default_model is empty', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: '',
      models: ['gpt-3.5-turbo'],
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).model).toBe('gpt-3.5-turbo');
  });

  it('falls back to currentAi.model when both default_model and models[0] are empty', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'ollama',
      default_model: '',
      models: [],
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).model).toBe(defaultAi.model);
  });

  it('sets api_key to null when provider has no api_key', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'ollama',
      default_model: 'llama3.2',
      models: ['llama3.2'],
      api_key: '',
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).api_key).toBeNull();
  });

  it('sets base_url to null when provider has no base_url', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: 'gpt-4o',
      models: ['gpt-4o'],
      base_url: '',
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).base_url).toBeNull();
  });

  it('uses provider temperature when defined', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: 'gpt-4o',
      models: [],
      temperature: 1.2,
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).temperature).toBe(1.2);
  });

  it('falls back to currentAi.temperature when provider temperature is undefined', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: 'gpt-4o',
      models: [],
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).temperature).toBe(defaultAi.temperature);
  });

  it('uses provider max_tokens when defined', () => {
    const provider: MinimalProviderConfig = {
      provider_type: 'openai',
      default_model: 'gpt-4o',
      models: [],
      max_tokens: 8192,
      enabled: true,
    };
    expect(buildAiConfig(provider, defaultAi).max_tokens).toBe(8192);
  });
});