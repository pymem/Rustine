<script>
  import { onMount, onDestroy } from 'svelte';
  import ScriptTabs from './ScriptTabs.svelte';
  import MonacoEditor from './MonacoEditor.svelte';
  import ConsoleDrawer from './ConsoleDrawer.svelte';
  import WorkspacePane from './WorkspacePane.svelte';
  import {
    editorState,
    appendLog,
    addScriptTab,
  } from '../../stores/editorState.svelte.js';
  import { appState } from '../../stores/appState.svelte.js';
  import { saveScriptToDisk } from '../../lib/tauri.js';

  let monacoComponent = null;
  let fileInputEl = null;
  let injecting = false;
  let pollId = null;

  function handleClear() {
    const activeId = editorState.activeId;
    if (!activeId) return;
    editorState.tabs = editorState.tabs.map((t) =>
      t.id === activeId ? { ...t, content: '' } : t
    );
    appendLog('editor cleared.');
  }

  function handleOpenFileClick() {
    if (fileInputEl) {
      fileInputEl.value = '';
      fileInputEl.click();
    }
  }

  function handleFileSelected(e) {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => {
      const content = event.target.result;
      addScriptTab(file.name, content);
      appendLog(`opened ${file.name}`);
    };
    reader.readAsText(file);
  }

  async function handleSave() {
    const active = editorState.tabs.find((t) => t.id === editorState.activeId);
    if (!active) return;
    const title = active.title || 'Script.lua';
    const finalName = title.endsWith('.lua') ? title : `${title}.lua`;
    await saveScriptToDisk(finalName, active.content);
    const exists = editorState.workspaceFiles.find((f) => f.name === finalName);
    if (exists) {
      editorState.workspaceFiles = editorState.workspaceFiles.map((f) =>
        f.name === finalName ? { ...f, content: active.content } : f
      );
    } else {
      editorState.workspaceFiles.push({
        id: `ws-${Date.now()}`,
        name: finalName,
        content: active.content,
      });
    }
    appendLog(`${finalName} saved to scripts/ folder.`);
  }

  function toggleConsole() {
    editorState.consoleOpen = !editorState.consoleOpen;
  }

  async function toggleInject() {
    if (injecting) return;
    injecting = true;
    try {
      appendLog('resolving DataModel...');
      const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
      if (!invoke) throw new Error('tauri not available');
      const line = await invoke('inject');
      appState.injected = true;
      appendLog(typeof line === 'string' && line ? line : 'DataModel verified');
    } catch (e) {
      appState.injected = false;
      appendLog(`inject failed: ${e}`);
    } finally {
      injecting = false;
    }
  }

  onMount(() => {
    pollId = setInterval(async () => {
      try {
        const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
        if (!invoke) return;
        const s = await invoke('get_status');
        if (s === 'not_found' && appState.injected) appState.injected = false;
      } catch {}
    }, 2000);
  });

  onDestroy(() => {
    if (pollId) clearInterval(pollId);
  });
</script>

<input
  bind:this={fileInputEl}
  type="file"
  accept=".lua,.txt"
  style="display: none;"
  onchange={handleFileSelected}
/>

<div class="editor-layout">
  <div class="editor-pane">
    <ScriptTabs />

    <div class="editor-canvas-wrapper">
      <MonacoEditor bind:this={monacoComponent} />
      <ConsoleDrawer />
    </div>

    <div class="editor-action-bar">
      <div class="action-group">
        <button class="action-btn" id="btn-clear" title="Clear Editor" onclick={handleClear}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M15 4V2H9v2H4v2h1v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V6h1V4h-5zm2 16H7V6h10v14zM9 8h2v10H9zm4 0h2v10h-2z"
            />
          </svg>
          <span>Clear</span>
        </button>

        <button class="action-btn" id="btn-open-file" title="Open File" onclick={handleOpenFileClick}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"
            />
          </svg>
          <span>Open</span>
        </button>

        <button class="action-btn" id="btn-save-file" title="Save File" onclick={handleSave}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"
            />
          </svg>
          <span>Save</span>
        </button>
      </div>

      <div class="action-group">
        <button
          class="action-btn icon-only"
          class:active={editorState.consoleOpen}
          id="btn-console-toggle"
          title="Toggle Console"
          onclick={toggleConsole}
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h16v12zM6 10l4 3-4 3v-2l1.33-1L6 12zm6 5h6v-2h-6z"
            />
          </svg>
        </button>

        <button
          class="action-btn inject-btn"
          class:injected={appState.injected}
          id="btn-inject"
          title="Verify DataModel"
          onclick={toggleInject}
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M4 6h7v2H4zm0 10h7v2H4zM2 8h2v8H2zm18-2h-7v2h7zm0 10h-7v2h7zm2-8h-2v8h2zM7 11h10v2H7z" />
          </svg>
          <span>{appState.injected ? 'Injected' : 'Inject'}</span>
        </button>
      </div>
    </div>
  </div>

  <WorkspacePane />
</div>

<style>
  .editor-layout {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    overflow: hidden;
    min-width: 0;
  }

  .editor-pane {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 0;
  }

  .editor-canvas-wrapper {
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

  .editor-action-bar {
    height: 30px;
    background-color: #1a1a1a;
    border-top: 1px solid #141414;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 4px;
    flex-shrink: 0;
    z-index: 11;
    min-width: 0;
    gap: 4px;
  }

  .action-group {
    display: flex;
    align-items: center;
    gap: 3px;
    flex-shrink: 0;
  }

  .action-btn {
    height: 22px;
    background: #242424;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #c2bebe;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    padding: 0 5px;
    font-family: 'ProggyClean', monospace;
    cursor: pointer;
    outline: none;
    border-radius: 0;
    transition: background 0.1s ease, color 0.1s ease;
    flex-shrink: 0;
  }

  .action-btn svg {
    width: 12px;
    height: 12px;
    fill: currentColor;
    display: block;
    flex-shrink: 0;
  }

  .action-btn span {
    font-family: 'ProggyClean', monospace;
    font-size: 15px;
    line-height: 15px;
    display: inline-flex;
    align-items: center;
    transform: translateY(0.5px);
  }

  .action-btn:hover {
    background-color: #2e2e2e;
    color: #ffffff;
  }

  .action-btn:hover svg {
    fill: #ffcec6;
  }

  .action-btn:active {
    border-top: 1px solid #141414;
    border-left: 1px solid #141414;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    padding-top: 1px;
    padding-left: 6px;
    padding-right: 4px;
  }

  .action-btn.icon-only {
    width: 22px;
    padding: 0;
  }

  .action-btn.icon-only:active {
    padding-top: 1px;
    padding-left: 1px;
    padding-right: 0;
  }

  .action-btn.icon-only.active {
    background: #191919;
    color: #ffcec6;
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #333333;
    border-bottom: 1px solid #333333;
  }

  .action-btn.icon-only.active svg {
    fill: #ffcec6;
  }

  .action-btn.inject-btn {
    color: #ffcec6;
    border-top: 1px solid #5a4545;
    border-left: 1px solid #5a4545;
    padding: 0 6px;
  }

  .action-btn.inject-btn:hover {
    background-color: #382929;
    color: #ffffff;
  }

  .action-btn.inject-btn.injected {
    color: #a8e6cf;
    border-top: 1px solid #3d5a45;
    border-left: 1px solid #3d5a45;
  }
</style>
