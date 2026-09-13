<script>
  import { onMount, onDestroy } from 'svelte';
  import { editorState } from '../../stores/editorState.svelte.js';
  import { settingsState } from '../../stores/settingsState.svelte.js';

  let containerEl = $state(null);
  let editorInstance = null;
  let isUpdatingModel = false;
  let resizeObserver = null;

  export function getEditor() {
    return editorInstance;
  }

  function initMonaco() {
    if (typeof window === 'undefined' || !window.require) return;

    window.require.config({ paths: { vs: '/vs' } });
    window.require(['vs/editor/editor.main'], function () {
      if (!containerEl) return;

      window.monaco.editor.defineTheme('win95-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [
          { background: '212121' },
          { token: 'comment', foreground: '5c5252', fontStyle: 'italic' },
          { token: 'keyword', foreground: 'ffcec6', fontStyle: 'bold' },
          { token: 'string', foreground: 'd4b8b4' },
          { token: 'number', foreground: 'c4a29d' },
          { token: 'identifier', foreground: 'e8dcd9' },
          { token: 'delimiter', foreground: '9c8c89' },
          { token: 'delimiter.bracket', foreground: '9c8c89' },
          { token: 'delimiter.parenthesis', foreground: '9c8c89' },
          { token: 'delimiter.square', foreground: '9c8c89' },
          { token: 'delimiter.curly', foreground: '9c8c89' },
        ],
        colors: {
          'editor.background': '#212121',
          'editor.foreground': '#e8dcd9',
          'editorCursor.foreground': '#ffcec6',
          'editor.lineHighlightBackground': '#262626',
          'editorLineNumber.foreground': '#4f4747',
          'editorLineNumber.activeForeground': '#c4a29d',
          'editor.selectionBackground': '#3d2f2d',
          'editor.inactiveSelectionBackground': '#2b2221',
          'editorBracketHighlight.foreground1': '#9c8c89',
          'editorBracketHighlight.foreground2': '#9c8c89',
          'editorBracketHighlight.foreground3': '#9c8c89',
          'editorBracketHighlight.foreground4': '#9c8c89',
          'editorBracketHighlight.foreground5': '#9c8c89',
          'editorBracketHighlight.foreground6': '#9c8c89',
          'editorBracketHighlight.unexpectedBracket.foreground': '#9c8c89',
          'editorBracketMatch.background': '#3d2f2d',
          'editorBracketMatch.border': '#ffcec6',
        },
      });

      window.monaco.editor.defineTheme('win95-light', {
        base: 'vs',
        inherit: true,
        rules: [
          { background: 'ffffff' },
          { token: 'comment', foreground: '008000', fontStyle: 'italic' },
          { token: 'keyword', foreground: '0000ff', fontStyle: 'bold' },
          { token: 'string', foreground: 'a31515' },
          { token: 'number', foreground: '09885a' },
          { token: 'identifier', foreground: '000000' },
          { token: 'delimiter', foreground: '000000' },
          { token: 'delimiter.bracket', foreground: '000000' },
          { token: 'delimiter.parenthesis', foreground: '000000' },
          { token: 'delimiter.square', foreground: '000000' },
          { token: 'delimiter.curly', foreground: '000000' },
        ],
        colors: {
          'editor.background': '#ffffff',
          'editor.foreground': '#000000',
          'editorCursor.foreground': '#000000',
          'editor.lineHighlightBackground': '#f0f0f0',
          'editorLineNumber.foreground': '#808080',
          'editorLineNumber.activeForeground': '#000080',
          'editor.selectionBackground': '#b5d5ff',
          'editor.inactiveSelectionBackground': '#d0e0ff',
          'editorBracketHighlight.foreground1': '#000000',
          'editorBracketHighlight.foreground2': '#000000',
          'editorBracketHighlight.foreground3': '#000000',
          'editorBracketHighlight.foreground4': '#000000',
          'editorBracketHighlight.foreground5': '#000000',
          'editorBracketHighlight.foreground6': '#000000',
          'editorBracketHighlight.unexpectedBracket.foreground': '#000000',
          'editorBracketMatch.background': '#d0e0ff',
          'editorBracketMatch.border': '#000080',
        },
      });

      const initialTab = editorState.tabs.find((t) => t.id === editorState.activeId);

      editorInstance = window.monaco.editor.create(containerEl, {
        value: initialTab ? initialTab.content : '',
        language: 'lua',
        theme: settingsState.theme === 'win95-classic' ? 'win95-light' : 'win95-dark',
        fontFamily: "'ProggyClean', monospace",
        fontSize: settingsState.fontSize || 16,
        lineHeight: 18,
        letterSpacing: 0,
        roundedSelection: false,
        scrollBeyondLastLine: false,
        automaticLayout: true,
        minimap: { enabled: settingsState.minimap },
        lineNumbers: settingsState.lineNumbers ? 'on' : 'off',
        wordWrap: settingsState.wordWrap ? 'on' : 'off',
        renderWhitespace: settingsState.renderWhitespace || 'none',
        cursorStyle: settingsState.cursorStyle || 'line',
        cursorBlinking: settingsState.cursorBlinking || 'solid',
        smoothScrolling: settingsState.smoothScrolling,
        tabSize: settingsState.tabSize || 4,
        renderLineHighlight: 'all',
        overviewRulerBorder: false,
        hideCursorInOverviewRuler: true,
        bracketPairColorization: { enabled: false },
        matchBrackets: 'never',
        scrollbar: {
          verticalScrollbarSize: 10,
          horizontalScrollbarSize: 10,
          useShadows: false,
        },
        insertSpaces: true,
        cursorWidth: 2,
      });

      if (window.ResizeObserver && containerEl) {
        resizeObserver = new ResizeObserver(() => {
          if (editorInstance) {
            editorInstance.layout();
          }
        });
        resizeObserver.observe(containerEl);
      }

      editorInstance.onDidChangeModelContent(() => {
        if (isUpdatingModel) return;
        const currentActive = editorState.activeId;
        const currentVal = editorInstance.getValue();
        editorState.tabs = editorState.tabs.map((t) =>
          t.id === currentActive ? { ...t, content: currentVal } : t
        );
      });

      window.editorInstance = editorInstance;
    });
  }

  onMount(() => {
    if (window.require) {
      initMonaco();
    } else {
      const checkTimer = setInterval(() => {
        if (window.require) {
          clearInterval(checkTimer);
          initMonaco();
        }
      }, 50);
      return () => clearInterval(checkTimer);
    }
  });

  onDestroy(() => {
    if (resizeObserver) {
      resizeObserver.disconnect();
      resizeObserver = null;
    }
    if (editorInstance) {
      editorInstance.dispose();
      editorInstance = null;
    }
  });

  $effect(() => {
    const activeTabObj = editorState.tabs.find((t) => t.id === editorState.activeId);

    if (editorInstance && activeTabObj) {
      const currentVal = editorInstance.getValue();
      if (currentVal !== activeTabObj.content) {
        isUpdatingModel = true;
        editorInstance.setValue(activeTabObj.content);
        isUpdatingModel = false;
        editorInstance.layout();
      }
    }
  });

  $effect(() => {
    const theme = settingsState.theme;
    const fontSize = settingsState.fontSize;
    const tabSize = settingsState.tabSize;
    const lineNumbers = settingsState.lineNumbers;
    const minimap = settingsState.minimap;
    const wordWrap = settingsState.wordWrap;
    const renderWhitespace = settingsState.renderWhitespace;
    const cursorStyle = settingsState.cursorStyle;
    const cursorBlinking = settingsState.cursorBlinking;
    const smoothScrolling = settingsState.smoothScrolling;

    if (editorInstance && window.monaco) {
      window.monaco.editor.setTheme(
        theme === 'win95-classic' ? 'win95-light' : 'win95-dark'
      );
      editorInstance.updateOptions({
        fontSize,
        tabSize,
        lineNumbers: lineNumbers ? 'on' : 'off',
        minimap: { enabled: minimap },
        wordWrap: wordWrap ? 'on' : 'off',
        renderWhitespace,
        cursorStyle,
        cursorBlinking,
        smoothScrolling,
      });
      editorInstance.layout();
    }
  });
</script>

<div class="monaco-wrapper">
  {#if editorState.tabs.length === 0}
    <div class="editor-empty-state">
      Click + to create a new script
    </div>
  {/if}
  <div
    class="monaco-container"
    bind:this={containerEl}
    style="display: {editorState.tabs.length > 0 ? 'block' : 'none'};"
  ></div>
</div>

<style>
  .monaco-wrapper {
    flex: 1;
    width: 100%;
    height: 100%;
    position: relative;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
  }

  .monaco-container {
    flex: 1;
    width: 100%;
    height: 100%;
    user-select: text;
    -webkit-user-select: text;
    min-width: 0;
    min-height: 0;
  }

  .editor-empty-state {
    flex: 1;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #4a4a4a;
    font-family: 'ProggyClean', monospace;
    font-size: 16px;
    letter-spacing: 0.5px;
    background-color: #212121;
  }

  :global(.monaco-editor .margin) {
    background-color: #212121 !important;
  }

  :global(.monaco-editor .line-numbers) {
    font-family: 'ProggyClean', monospace !important;
    font-size: 16px !important;
    color: #4f4747 !important;
    text-align: right !important;
    padding-right: 10px !important;
  }

  :global(.monaco-editor .active-line-number) {
    color: #c4a29d !important;
  }
</style>
