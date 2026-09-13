<script>
  import { appState } from '../../stores/appState.svelte.js';
  import { appendLog } from '../../stores/editorState.svelte.js';
  import { openUrl } from '../../lib/tauri.js';

  function setTab(tab) {
    appState.activeTab = tab;
  }

  function toggleCollapse() {
    appState.sidebarCollapsed = !appState.sidebarCollapsed;
  }

  async function handleDiscordClick() {
    appendLog('opening discord.gg/rustine...');
    await openUrl('https://discord.gg/rustine');
  }
</script>

<div class="sidebar" class:collapsed={appState.sidebarCollapsed} id="sidebar">
  <div class="sidebar-top">
    <button
      class="sidebar-tab"
      class:active={appState.activeTab === 'editor'}
      title="Editor"
      onclick={() => setTab('editor')}
    >
      <span class="sidebar-tab-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path
            d="M11 18H9v-4h2v4Zm-4-1H5v-2h2v2Zm12-2v2h-2v-2h2ZM5 15H3v-2h2v2Zm16 0h-2v-2h2v2Zm-8-1h-2v-4h2v4ZM3 13H1v-2h2v2Zm20 0h-2v-2h2v2ZM5 11H3V9h2v2Zm16 0h-2V9h2v2Zm-6-1h-2V6h2v4ZM7 9H5V7h2v2Zm12 0h-2V7h2v2Z"
          />
        </svg>
      </span>
      <span class="sidebar-tab-label">Editor</span>
    </button>

    <button
      class="sidebar-tab"
      class:active={appState.activeTab === 'scripthub'}
      title="Scripthub"
      onclick={() => setTab('scripthub')}
    >
      <span class="sidebar-tab-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path
            d="M16 19h2v2H4v-2h10v-2h2v2ZM6 15h8v2H4v2H2v-4h2V5h2v10ZM20 5h2v6h-2v8h-2V5H6V3h14v2Z"
          />
        </svg>
      </span>
      <span class="sidebar-tab-label">Scripthub</span>
    </button>

    <button
      class="sidebar-tab"
      class:active={appState.activeTab === 'settings'}
      title="Settings"
      onclick={() => setTab('settings')}
    >
      <span class="sidebar-tab-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path
            d="M17 18h5v2h-5v2h-2v-6h2v2Zm-4 2H2v-2h11v2Zm-4-5H7v-2H2v-2h5V9h2v6Zm13-2H11v-2h11v2Zm-7-9h7v2h-7v2h-2V2h2v2Zm-4 2H2V4h9v2Z"
          />
        </svg>
      </span>
      <span class="sidebar-tab-label">Settings</span>
    </button>
  </div>

  <div class="sidebar-bottom">
    <button
      class="sidebar-tab discord-btn"
      id="btn-discord"
      title="Discord Community"
      onclick={handleDiscordClick}
    >
      <span class="sidebar-tab-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path
            d="M9 21H5v-2h4v2Zm10 0h-4v-2h4v2ZM5 19H3v-2h2v2Zm12-2h-2v2h-2v-2h-2v2H9v-2H7v-2h10v2Zm4 2h-2v-2h2v2ZM3 17H1V7h2v10Zm20 0h-2V7h2v10Zm-12-4H8v-3h3v3Zm5 0h-3v-3h3v3ZM5 7H3V5h2v2Zm10 0H9V5h6v2Zm6 0h-2V5h2v2ZM9 5H5V3h4v2Zm10 0h-4V3h4v2Z"
          />
        </svg>
      </span>
      <span class="sidebar-tab-label">Discord</span>
    </button>

    <button
      class="sidebar-tab collapse-btn"
      id="btn-collapse-sidebar"
      title="Toggle Sidebar"
      onclick={toggleCollapse}
    >
      <span class="sidebar-tab-icon" id="collapse-icon">
        {#if appState.sidebarCollapsed}
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M16 13v-2h-2v2h2Zm-2-2V9h-2v2h2Zm0 4v-2h-2v2h2Zm-2-6V7h-2v2h2Zm0 8v-2h-2v2h2ZM10 7V5H8v2h2Zm0 12v-2H8v2h2Z" />
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M8 13v-2h2v2H8Zm2-2V9h2v2h-2Zm0 4v-2h2v2h-2Zm2-6V7h2v2h-2Zm0 8v-2h2v2h-2Zm2-10V5h2v2h-2Zm0 12v-2h2v2h-2Z" />
          </svg>
        {/if}
      </span>
      <span class="sidebar-tab-label">Collapse</span>
    </button>
  </div>
</div>

<style>
  .sidebar {
    width: 136px;
    height: 100%;
    background-color: #212121;
    border-top: 1px solid #141414;
    border-left: 1px solid #141414;
    border-right: 1px solid #383838;
    border-bottom: 1px solid #383838;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 4px;
    overflow: hidden;
    transition: width 0.2s cubic-bezier(0.1, 0.9, 0.2, 1), padding 0.2s ease;
  }

  .sidebar.collapsed {
    width: 36px;
    padding: 4px 2px;
  }

  .sidebar-top {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
  }

  .sidebar-bottom {
    display: flex;
    flex-direction: column;
    gap: 3px;
    width: 100%;
    padding-top: 4px;
    border-top: 1px solid #181818;
  }

  .sidebar-tab {
    display: flex;
    align-items: center;
    gap: 7px;
    height: 28px;
    padding: 0 8px;
    background-color: transparent;
    border: 1px solid transparent;
    color: #9a9a9a;
    cursor: pointer;
    font-family: 'ProggyClean', monospace;
    font-size: 16px;
    line-height: 16px;
    transition: background 0.1s ease, color 0.1s ease, padding 0.2s ease;
    outline: none;
    text-align: left;
    width: 100%;
  }

  .sidebar.collapsed .sidebar-tab {
    padding: 0;
    justify-content: center;
  }

  .sidebar-tab-icon {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .sidebar-tab-icon svg {
    width: 14px;
    height: 14px;
    fill: currentColor;
  }

  .sidebar-tab-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transform: translateY(0.5px);
    transition: opacity 0.15s ease;
  }

  .sidebar.collapsed .sidebar-tab-label {
    display: none;
  }

  .sidebar-tab:hover {
    background-color: #292929;
    color: #d0d0d0;
  }

  .sidebar-tab.active {
    background-color: #191919;
    color: #ffcec6;
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #333333;
    border-bottom: 1px solid #333333;
  }

  .sidebar-tab.active .sidebar-tab-icon svg {
    fill: #ffcec6;
  }

  .discord-btn {
    color: #8a8fa3;
  }

  .discord-btn:hover {
    background-color: #272a38;
    color: #aeb5d6;
  }

  .discord-btn:hover .sidebar-tab-icon svg {
    fill: #aeb5d6;
  }

  .collapse-btn {
    color: #777777;
  }

  .collapse-btn:hover {
    background-color: #282828;
    color: #ffcec6;
  }

  .collapse-btn:hover .sidebar-tab-icon svg {
    fill: #ffcec6;
  }
</style>
