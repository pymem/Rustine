<script>
  import { onMount } from 'svelte';
  import {
    editorState,
    addScriptTab,
    addWorkspaceFile,
    deleteWorkspaceFile,
    loadWorkspaceScripts,
    appendLog,
  } from '../../stores/editorState.svelte.js';
  import { appState } from '../../stores/appState.svelte.js';

  onMount(() => {
    loadWorkspaceScripts();
  });

  async function handleRefresh() {
    await loadWorkspaceScripts();
    appendLog('workspace refreshed from scripts/ folder.');
  }

  $effect(() => {
    editorState.workspaceWidth = appState.sidebarCollapsed ? 155 : 110;
  });

  function handleAddWorkspaceFile() {
    const newTab = addScriptTab();
    addWorkspaceFile(newTab.title, newTab.content);
  }

  function handleSelectFile(file) {
    const existingTab = editorState.tabs.find((t) => t.title === file.name);
    if (existingTab) {
      editorState.activeId = existingTab.id;
    } else {
      addScriptTab(file.name, file.content);
    }
  }
</script>

<div
  class="workspace-pane"
  style="width: {editorState.workspaceWidth}px;"
>
  <div class="workspace-header">
    <span class="workspace-title">Workspace</span>
    <div class="workspace-controls">
      <button
        class="ws-btn"
        id="btn-add-workspace"
        title="Add Script to Workspace"
        onclick={handleAddWorkspaceFile}
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path d="M11 4h2v7h7v2h-7v7h-2v-7H4v-2h7z" />
        </svg>
      </button>
      <button
        class="ws-btn"
        id="btn-refresh-workspace"
        title="Refresh Workspace"
        onclick={handleRefresh}
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path d="M16 4h2v6h-2zm-2-2h2v2h-2zm0 2h2v8h-2zM4 8H2v5h2z" />
          <path d="M4 6h16v2H4zm4 14H6v-6h2zm2 2H8v-2h2zm0-2H8v-8h2zm10-4h2v-5h-2z" />
          <path d="M20 18H4v-2h16z" />
        </svg>
      </button>
    </div>
  </div>

  <div class="workspace-list">
    {#each editorState.workspaceFiles as file (file.id)}
      {@const isActive = editorState.tabs.some((t) => t.id === editorState.activeId && t.title === file.name)}
      <div
        role="button"
        tabindex="0"
        class="workspace-item"
        class:active={isActive}
        onclick={() => handleSelectFile(file)}
        onkeydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') handleSelectFile(file);
        }}
      >
        <span class="workspace-item-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M6 4H4v16h2zm10-2H6v2h10zm4 4h-2v14h2zm-2 14H6v2h12zM16 4h2v2h-2zm-4 0h2v6h-2z"
            />
            <path d="M12 8h6v2h-6z" />
          </svg>
        </span>
        <span class="workspace-item-label">{file.name}</span>
        <button
          type="button"
          class="workspace-item-delete"
          title="Delete file"
          onclick={(e) => {
            e.stopPropagation();
            deleteWorkspaceFile(file.id);
          }}
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M7 19H5V17H7V19ZM19 19H17V17H19V19ZM9 15V17H7V15H9ZM17 17H15V15H17V17ZM11 15H9V13H11V15ZM15 15H13V13H15V15ZM13 13H11V11H13V13ZM11 11H9V9H11V11ZM15 11H13V9H15V11ZM9 9H7V7H9V9ZM17 9H15V7H17V9ZM7 7H5V5H7V7ZM19 7H17V5H19V7Z"
            />
          </svg>
        </button>
      </div>
    {/each}
  </div>
</div>

<style>
  .workspace-pane {
    height: 100%;
    background-color: #1c1c1c;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    overflow: hidden;
    border-left: 1px solid #141414;
    transition: width 0.2s cubic-bezier(0.1, 0.9, 0.2, 1);
  }

  .workspace-header {
    height: 25px;
    background-color: #171717;
    border-bottom: 1px solid #141414;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 4px 0 6px;
    flex-shrink: 0;
  }

  .workspace-title {
    font-family: 'ProggyClean', monospace;
    font-size: 13px;
    line-height: 13px;
    color: #888888;
    transform: translateY(0.5px);
    white-space: nowrap;
    overflow: hidden;
  }

  .workspace-controls {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .ws-btn {
    width: 15px;
    height: 15px;
    background: transparent;
    border: 1px solid transparent;
    color: #777777;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border-radius: 0;
    outline: none;
  }

  .ws-btn svg {
    width: 11px;
    height: 11px;
    fill: currentColor;
  }

  .ws-btn:hover {
    background: #262626;
    color: #ffcec6;
    border: 1px solid #333333;
  }

  .ws-btn:active {
    background: #141414;
    border: 1px solid #0d0d0d;
  }

  .workspace-list {
    flex: 1;
    padding: 3px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .workspace-list::-webkit-scrollbar {
    width: 6px;
  }

  .workspace-list::-webkit-scrollbar-track {
    background: #181818;
  }

  .workspace-list::-webkit-scrollbar-thumb {
    background: #282828;
    border: 1px solid #141414;
  }

  .workspace-item {
    display: flex;
    align-items: center;
    gap: 4px;
    height: 22px;
    padding: 0 4px 0 6px;
    background: transparent;
    color: #9a9a9a;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    cursor: pointer;
    border: 1px solid transparent;
    user-select: none;
    transition: all 0.1s ease;
    position: relative;
    min-width: 0;
  }

  .workspace-item-icon {
    width: 12px;
    height: 12px;
    fill: currentColor;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .workspace-item-icon svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  .workspace-item-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transform: translateY(0.5px);
    min-width: 0;
  }

  .workspace-item-delete {
    width: 13px;
    height: 13px;
    display: none;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: #666666;
    cursor: pointer;
    padding: 0;
    border-radius: 0;
    flex-shrink: 0;
  }

  .workspace-item:hover .workspace-item-delete,
  .workspace-item.active .workspace-item-delete {
    display: flex;
  }

  .workspace-item-delete:hover {
    color: #ff7b72;
    background-color: rgba(255, 123, 114, 0.2);
  }

  .workspace-item-delete svg {
    width: 10px;
    height: 10px;
    fill: currentColor;
  }

  .workspace-item:hover {
    background: #242424;
    color: #dedede;
    border: 1px solid #333333;
  }

  .workspace-item.active {
    background: #282121;
    color: #ffcec6;
    border-top: 1px solid #141414;
    border-left: 1px solid #141414;
    border-right: 1px solid #383838;
    border-bottom: 1px solid #383838;
  }

  .workspace-item.active .workspace-item-icon svg {
    fill: #ffcec6;
  }
</style>
