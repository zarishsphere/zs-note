/**
 * Tests for src/lib/commands/ai.ts (PR changes).
 *
 * Key change tested: testProviderConnection now accepts a full ProviderConfig
 * object and passes it directly to invoke(), instead of accepting a
 * ProviderConfig | string and extracting the id.
 */
import { describe, it, expect, vi, beforeEach } from 'vitest';

// ---------------------------------------------------------------------------
// Mock @tauri-apps/api/core so we can inspect invoke() calls
// ---------------------------------------------------------------------------
const invokeMock = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: invokeMock,
}));

// Mock the event listener used by aiChat/aiTemplate
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

// Import after mocking so the module picks up the mock
import {
  testProviderConnection,
  aiListModels,
} from './ai';
import type { ProviderConfig } from '../types';

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

function makeProvider(overrides?: Partial<ProviderConfig>): ProviderConfig {
  return {
    id: 'my-provider',
    name: 'My Provider',
    provider_type: 'ollama',
    api_key: 'key123',
    base_url: 'http://localhost:11434',
    models: ['llama3.2'],
    default_model: 'llama3.2',
    enabled: true,
    temperature: 0.7,
    max_tokens: 4096,
    ...overrides,
  };
}

// ---------------------------------------------------------------------------
// testProviderConnection
// ---------------------------------------------------------------------------

describe('testProviderConnection', () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it('passes the full ProviderConfig object to invoke', async () => {
    invokeMock.mockResolvedValue(true);

    const provider = makeProvider();
    await testProviderConnection(provider);

    expect(invokeMock).toHaveBeenCalledOnce();
    expect(invokeMock).toHaveBeenCalledWith('test_provider_connection', { provider });
  });

  it('passes the whole provider object (not just the id)', async () => {
    invokeMock.mockResolvedValue(true);

    const provider = makeProvider({ id: 'openai-prod', provider_type: 'openai' });
    await testProviderConnection(provider);

    const [, args] = invokeMock.mock.calls[0];
    // The full object must be passed, not only the id string.
    expect(args.provider).toEqual(provider);
    expect(typeof args.provider).toBe('object');
  });

  it('returns the boolean result from invoke', async () => {
    invokeMock.mockResolvedValue(true);
    const result = await testProviderConnection(makeProvider());
    expect(result).toBe(true);
  });

  it('returns false when invoke resolves with false', async () => {
    invokeMock.mockResolvedValue(false);
    const result = await testProviderConnection(makeProvider());
    expect(result).toBe(false);
  });

  it('propagates errors from invoke', async () => {
    invokeMock.mockRejectedValue(new Error('connection refused'));
    await expect(testProviderConnection(makeProvider())).rejects.toThrow('connection refused');
  });

  it('works with a provider that has no api_key', async () => {
    invokeMock.mockResolvedValue(true);

    const provider = makeProvider({ api_key: undefined });
    await testProviderConnection(provider);

    const [, args] = invokeMock.mock.calls[0];
    expect(args.provider.api_key).toBeUndefined();
  });

  it('works with a provider that has no base_url', async () => {
    invokeMock.mockResolvedValue(true);

    const provider = makeProvider({ base_url: undefined });
    await testProviderConnection(provider);

    const [, args] = invokeMock.mock.calls[0];
    expect(args.provider.base_url).toBeUndefined();
  });
});

// ---------------------------------------------------------------------------
// aiListModels
// ---------------------------------------------------------------------------

describe('aiListModels', () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it('calls invoke with ai_list_models and the provider string', async () => {
    invokeMock.mockResolvedValue(['gpt-4o', 'gpt-3.5-turbo']);

    const result = await aiListModels('openai');

    expect(invokeMock).toHaveBeenCalledWith('ai_list_models', { provider: 'openai' });
    expect(result).toEqual(['gpt-4o', 'gpt-3.5-turbo']);
  });

  it('returns an empty array when no models are available', async () => {
    invokeMock.mockResolvedValue([]);
    const result = await aiListModels('ollama');
    expect(result).toEqual([]);
  });
});