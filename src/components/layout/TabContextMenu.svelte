<script>
  import { onMount } from 'svelte';
  import { appState, closeContextMenu } from '../../stores/appState.svelte.js';
  import {
    editorState,
    closeScriptTab,
    closeOtherTabs,
    closeAllTabs,
    duplicateTab,
  } from '../../stores/editorState.svelte.js';

  function handleRename() {
    if (appState.contextMenu.tabId) {
      editorState.renamingTabId = appState.contextMenu.tabId;
    }
    closeContextMenu();
  }

  function handleDuplicate() {
    if (appState.contextMenu.tabId) {
      duplicateTab(appState.contextMenu.tabId);
    }
    closeContextMenu();
  }

  function handleClose() {
    if (appState.contextMenu.tabId) {
      closeScriptTab(appState.contextMenu.tabId);
    }
    closeContextMenu();
  }

  function handleCloseOthers() {
    if (appState.contextMenu.tabId) {
      closeOtherTabs(appState.contextMenu.tabId);
    }
    closeContextMenu();
  }

  function handleCloseAll() {
    closeAllTabs();
    closeContextMenu();
  }

  function handleWindowClick(e) {
    if (appState.contextMenu.visible && !e.target.closest('.win-context-menu')) {
      closeContextMenu();
    }
  }

  onMount(() => {
    window.addEventListener('click', handleWindowClick);
    window.addEventListener('contextmenu', handleWindowClick);
    return () => {
      window.removeEventListener('click', handleWindowClick);
      window.removeEventListener('contextmenu', handleWindowClick);
    };
  });
</script>

{#if appState.contextMenu.visible}
  <div
    class="win-context-menu open"
    style="left: {appState.contextMenu.x}px; top: {appState.contextMenu.y}px;"
  >
    <button class="ctx-item" onclick={handleRename}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path d="M4 4h16v2H4zm0 14h16v2H4zM2 6h2v12H2zm18 0h2v12h-2zM8 9h2v6H8z" />
      </svg>
      <span>Rename</span>
    </button>

    <button class="ctx-item" onclick={handleDuplicate}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path
          d="M8 6h12v2H8zM4 2h12v2H4zm2 6h2v12H6zM2 4h2v12H2zm6 16h12v2H8zM20 8h2v12h-2zm-4-4h2v2h-2zM4 16h2v2H4z"
        />
      </svg>
      <span>Duplicate</span>
    </button>

    <div class="ctx-divider"></div>

    <button class="ctx-item" onclick={handleClose}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path
          d="M7 19H5V17H7V19ZM19 19H17V17H19V19ZM9 15V17H7V15H9ZM17 17H15V15H17V17ZM11 15H9V13H11V15ZM15 15H13V13H15V15ZM13 13H11V11H13V13ZM11 11H9V9H11V11ZM15 11H13V9H15V11ZM9 9H7V7H9V9ZM17 9H15V7H17V9ZM7 7H5V5H7V7ZM19 7H17V5H19V7Z"
        />
      </svg>
      <span>Close</span>
    </button>

    <button class="ctx-item" onclick={handleCloseOthers}>
      <span>Close Others</span>
    </button>

    <button class="ctx-item" onclick={handleCloseAll}>
      <span>Close All</span>
    </button>
  </div>
{/if}

<style>
  .win-context-menu {
    position: fixed;
    display: none;
    background-color: #262626;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.6);
    padding: 2px;
    z-index: 10000;
    min-width: 130px;
    flex-direction: column;
  }

  .win-context-menu.open {
    display: flex;
  }

  .ctx-item {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 22px;
    padding: 0 8px 0 6px;
    background: transparent;
    border: 1px solid transparent;
    color: #c0c0c0;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    cursor: pointer;
    user-select: none;
    text-align: left;
    width: 100%;
  }

  .ctx-item svg {
    width: 12px;
    height: 12px;
    fill: currentColor;
    flex-shrink: 0;
  }

  .ctx-item span {
    flex: 1;
    transform: translateY(0.5px);
  }

  .ctx-item:hover {
    background-color: #191919;
    color: #ffcec6;
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #333333;
    border-bottom: 1px solid #333333;
  }

  .ctx-item:hover svg {
    fill: #ffcec6;
  }

  .ctx-divider {
    height: 1px;
    background-color: #141414;
    border-bottom: 1px solid #383838;
    margin: 2px 1px;
  }
</style>
