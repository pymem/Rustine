<script>
  import {
    editorState,
    addScriptTab,
    closeScriptTab,
    renameTab,
  } from '../../stores/editorState.svelte.js';
  import { openContextMenu } from '../../stores/appState.svelte.js';

  let renameInputEl = $state(null);

  function selectTab(id) {
    editorState.activeId = id;
  }

  function handleDoubleClick(id) {
    editorState.renamingTabId = id;
  }

  function handleRenameBlur(id, e) {
    renameTab(id, e.target.value);
  }

  function handleRenameKeydown(id, e) {
    if (e.key === 'Enter') {
      renameTab(id, e.target.value);
    } else if (e.key === 'Escape') {
      editorState.renamingTabId = null;
    }
  }

  $effect(() => {
    if (editorState.renamingTabId && renameInputEl) {
      renameInputEl.focus();
      renameInputEl.select();
    }
  });
</script>

<div class="script-tabs-bar">
  <div class="script-tabs-scroll-area" id="script-tabs-list">
    {#each editorState.tabs as tab (tab.id)}
      <div
        role="tab"
        tabindex="0"
        class="script-tab"
        class:active={editorState.activeId === tab.id}
        onclick={() => selectTab(tab.id)}
        ondblclick={() => handleDoubleClick(tab.id)}
        oncontextmenu={(e) => openContextMenu(e, tab.id)}
        onkeydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') selectTab(tab.id);
        }}
      >
        {#if editorState.renamingTabId === tab.id}
          <input
            bind:this={renameInputEl}
            class="script-tab-rename-input"
            type="text"
            value={tab.title}
            onblur={(e) => handleRenameBlur(tab.id, e)}
            onkeydown={(e) => handleRenameKeydown(tab.id, e)}
            onclick={(e) => e.stopPropagation()}
          />
        {:else}
          <span class="script-tab-title">{tab.title}</span>
        {/if}

        <button
          type="button"
          class="script-tab-close"
          title="Close tab"
          onclick={(e) => {
            e.stopPropagation();
            closeScriptTab(tab.id);
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

  <button
    class="script-tab-add"
    id="btn-add-tab"
    title="New Script Tab"
    onclick={() => addScriptTab()}
  >
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
      <path d="M11 4h2v7h7v2h-7v7h-2v-7H4v-2h7z" />
    </svg>
  </button>
</div>

<style>
  .script-tabs-bar {
    height: 25px;
    background-color: #1a1a1a;
    border-bottom: 1px solid #141414;
    display: flex;
    align-items: flex-end;
    padding: 2px 3px 0 3px;
    gap: 2px;
    overflow: hidden;
    flex-shrink: 0;
    width: 100%;
    min-width: 0;
  }

  .script-tabs-scroll-area {
    flex: 1;
    display: flex;
    align-items: flex-end;
    gap: 2px;
    overflow-x: auto;
    scrollbar-width: none;
    -ms-overflow-style: none;
    min-width: 0;
  }

  .script-tabs-scroll-area::-webkit-scrollbar {
    display: none;
  }

  .script-tab {
    height: 22px;
    background-color: #1e1e1e;
    border-top: 1px solid #383838;
    border-left: 1px solid #383838;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #888888;
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 0 5px 0 7px;
    cursor: pointer;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    white-space: nowrap;
    flex-shrink: 0;
    position: relative;
    transition: background 0.1s ease, color 0.1s ease;
  }

  .script-tab:hover {
    background-color: #262626;
    color: #c0c0c0;
  }

  .script-tab.active {
    height: 23px;
    background-color: #212121;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #212121;
    color: #ffcec6;
    z-index: 2;
  }

  .script-tab-rename-input {
    background: #141414;
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    color: #ffcec6;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    height: 17px;
    padding: 0 3px;
    outline: 1px solid #ffcec6;
    min-width: 60px;
    max-width: 120px;
    width: auto;
  }

  .script-tab-close {
    width: 12px;
    height: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: #777777;
    cursor: pointer;
    border-radius: 0;
    padding: 0;
  }

  .script-tab-close:hover {
    color: #ff7b72;
    background-color: rgba(255, 123, 114, 0.2);
  }

  .script-tab-close svg {
    width: 10px;
    height: 10px;
    fill: currentColor;
  }

  .script-tab-add {
    width: 19px;
    height: 19px;
    background-color: #212121;
    border-top: 1px solid #383838;
    border-left: 1px solid #383838;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #888888;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    outline: none;
    margin-bottom: 1px;
    flex-shrink: 0;
  }

  .script-tab-add:hover {
    background-color: #2a2a2a;
    color: #ffcec6;
  }

  .script-tab-add svg {
    width: 12px;
    height: 12px;
    fill: currentColor;
  }
</style>
