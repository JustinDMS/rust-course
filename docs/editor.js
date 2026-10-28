// The exercise editor: CodeMirror 6, loaded from esm.sh through the import
// map in index.html. Exposes `window.Editor`:
//
//   Editor.ready   promise resolving to "codemirror" or "fallback"
//   Editor.create(parent, { doc, onChange(text), onRun() })
//                  -> { getValue, setValue, focus, destroy }, or null when
//                     CodeMirror could not be loaded (app.js then uses its
//                     own plain textarea editor).
//
// A module script, so it only runs over http(s); opened from the file
// system, app.js falls back on its own.

const api = { mode: 'loading', create: null, ready: null };
window.Editor = api;

api.ready = (async () => {
  try {
    const [stateMod, viewMod, cmMod, rustMod, darkMod, cmdMod, langMod] = await Promise.all([
      import('@codemirror/state'),
      import('@codemirror/view'),
      import('codemirror'),
      import('@codemirror/lang-rust'),
      import('@codemirror/theme-one-dark'),
      import('@codemirror/commands'),
      import('@codemirror/language'),
    ]);
    const { EditorState, Compartment } = stateMod;
    const { EditorView, keymap } = viewMod;
    const { basicSetup } = cmMod;
    const { rust } = rustMod;
    const { oneDark } = darkMod;
    const { indentWithTab } = cmdMod;
    const { indentUnit } = langMod;

    const dark = window.matchMedia('(prefers-color-scheme: dark)');
    const themeFor = () => (dark.matches ? oneDark : []);
    const base = EditorView.theme({
      '&': { fontSize: '13.5px' },
      '.cm-content, .cm-gutters': {
        fontFamily: 'ui-monospace, "Cascadia Code", "JetBrains Mono", Consolas, "SF Mono", Menlo, monospace',
      },
      '.cm-scroller': { lineHeight: '1.55' },
      '.cm-content': { padding: '10px 0' },
      '&.cm-focused': { outline: 'none' },
    });

    const editors = new Set();
    dark.addEventListener('change', () => {
      for (const e of editors) e.view.dispatch({ effects: e.theme.reconfigure(themeFor()) });
    });

    api.create = (parent, opts) => {
      const theme = new Compartment();
      const state = EditorState.create({
        doc: opts.doc || '',
        extensions: [
          keymap.of([
            { key: 'Mod-Enter', run: () => { if (opts.onRun) opts.onRun(); return true; } },
            indentWithTab,
          ]),
          basicSetup,
          rust(),
          indentUnit.of('    '),
          EditorState.tabSize.of(4),
          base,
          theme.of(themeFor()),
          EditorView.updateListener.of((u) => {
            if (u.docChanged && opts.onChange) opts.onChange(u.state.doc.toString());
          }),
        ],
      });
      const view = new EditorView({ state, parent });
      const handle = {
        view,
        theme,
        getValue: () => view.state.doc.toString(),
        setValue: (text) => view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: text } }),
        focus: () => view.focus(),
        destroy: () => { editors.delete(handle); view.destroy(); },
      };
      editors.add(handle);
      return handle;
    };
    api.mode = 'codemirror';
  } catch (e) {
    console.warn('CodeMirror could not be loaded; using the plain editor.', e);
    api.mode = 'fallback';
  }
  return api.mode;
})();
