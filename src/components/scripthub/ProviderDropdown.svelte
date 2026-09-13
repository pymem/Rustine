<script>
  import { onMount } from 'svelte';
  import {
    providers,
    scripthubState,
    fetchScripts,
  } from '../../stores/scripthubState.svelte.js';
  import { appendLog } from '../../stores/editorState.svelte.js';

  let isOpen = $state(false);

  function toggleDropdown(e) {
    e.stopPropagation();
    isOpen = !isOpen;
  }

  function selectProvider(prov, e) {
    e.stopPropagation();
    scripthubState.provider = prov;
    isOpen = false;
    appendLog(`source changed to ${prov.name}`);
    fetchScripts(false);
  }

  function handleWindowClick() {
    isOpen = false;
  }

  onMount(() => {
    window.addEventListener('click', handleWindowClick);
    return () => window.removeEventListener('click', handleWindowClick);
  });
</script>

<div class="dropdown-wrapper">
  <button
    class="scripthub-btn icon-only-dropdown"
    class:open={isOpen}
    id="btn-scripthub-provider"
    title="Source: {scripthubState.provider.name}"
    onclick={toggleDropdown}
  >
    <img
      src={scripthubState.provider.icon}
      alt={scripthubState.provider.name}
      class="provider-favicon"
    />
  </button>

  {#if isOpen}
    <div class="scripthub-dropdown-menu show" id="menu-scripthub-provider">
      {#each providers as prov (prov.id)}
        <button
          type="button"
          class="dropdown-option"
          class:active={scripthubState.provider.id === prov.id}
          onclick={(e) => selectProvider(prov, e)}
        >
          <div class="provider-item-row">
            <img
              src={prov.icon}
              alt={prov.name}
              class="provider-menu-icon"
            />
            <span>{prov.name}</span>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown-wrapper {
    position: relative;
    flex-shrink: 0;
  }

  .scripthub-btn {
    height: 22px;
    background: #242424;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #c2bebe;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 0;
    outline: none;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    user-select: none;
    transition: background 0.1s ease, color 0.1s ease;
  }

  .scripthub-btn:hover {
    background: #2e2e2e;
    color: #ffffff;
  }

  .scripthub-btn:active,
  .scripthub-btn.open {
    border-top: 1px solid #141414;
    border-left: 1px solid #141414;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    background: #1e1e1e;
    color: #ffcec6;
  }

  .scripthub-btn.icon-only-dropdown {
    width: 24px;
    padding: 0;
  }

  .provider-favicon {
    width: 14px;
    height: 14px;
    object-fit: contain;
    display: block;
    border-radius: 2px;
  }

  .scripthub-dropdown-menu {
    position: absolute;
    top: 24px;
    right: 0;
    background: #262626;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.6);
    padding: 2px;
    z-index: 1000;
    min-width: 110px;
    display: flex;
    flex-direction: column;
  }

  .dropdown-option {
    height: 20px;
    padding: 0 6px;
    display: flex;
    align-items: center;
    color: #c0c0c0;
    font-family: 'ProggyClean', monospace;
    font-size: 13px;
    line-height: 13px;
    cursor: pointer;
    border: 1px solid transparent;
    user-select: none;
    white-space: nowrap;
  }

  .dropdown-option:hover {
    background-color: #191919;
    color: #ffcec6;
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #333333;
    border-bottom: 1px solid #333333;
  }

  .dropdown-option.active {
    color: #ffcec6;
    font-weight: bold;
  }

  .provider-item-row {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
  }

  .provider-menu-icon {
    width: 13px;
    height: 13px;
    object-fit: contain;
    flex-shrink: 0;
    border-radius: 2px;
  }
</style>
