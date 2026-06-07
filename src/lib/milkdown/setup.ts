import { Editor, rootCtx, defaultValueCtx, editorViewCtx } from '@milkdown/core';
import { commonmark } from '@milkdown/preset-commonmark';
import { gfm } from '@milkdown/preset-gfm';
import { history } from '@milkdown/plugin-history';
import { tooltip } from '@milkdown/plugin-tooltip';
import { listener, listenerCtx } from '@milkdown/plugin-listener';
import { clipboard } from '@milkdown/plugin-clipboard';
import { cursor } from '@milkdown/plugin-cursor';

declare module '@milkdown/core' {
  interface Common {
    zarishNoteOnChange?: (content: string) => void;
    zarishNoteOnSave?: () => void;
  }
}

export interface MilkdownSetupOptions {
  content: string;
  readOnly?: boolean;
  onChange?: (content: string) => void;
  onSave?: () => void;
  root: HTMLElement;
}

export function createMilkdownEditor(options: MilkdownSetupOptions) {
  const { content, readOnly, onChange, onSave, root } = options;

  const editor = Editor.make()
    .config((ctx) => {
      ctx.set(rootCtx, root);
      ctx.set(defaultValueCtx, content);
    })
    .use(commonmark)
    .use(gfm)
    .use(history)
    .use(tooltip)
    .use(clipboard)
    .use(cursor)
    .use(listener)
    .use(listener, (ctx) => {
      const listener = ctx.get(listenerCtx);

      listener.markdownUpdated((ctx, markdown) => {
        onChange?.(markdown);
      });

      listener.mounted(() => {
        if (readOnly) {
          const view = ctx.get(editorViewCtx);
          view.dom.setAttribute('contenteditable', 'false');
        }
      });
    })
    .config((ctx) => {
      const listener = ctx.get(listenerCtx);

      document.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 's') {
          e.preventDefault();
          onSave?.();
        }
      });
    });

  return {
    editor,
    async destroy() {
      await editor.destroy();
    },
    async getContent(): Promise<string> {
      try {
        const { getMarkdown } = await import('@milkdown/utils');
        return editor.action(getMarkdown());
      } catch {
        return '';
      }
    },
    async setContent(markdown: string) {
      const { setMarkdown } = await import('@milkdown/utils');
      editor.action((ctx) => {
        ctx.set(defaultValueCtx, markdown);
      });
    },
  };
}
