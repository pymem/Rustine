<script>
  import { onMount } from 'svelte';
  import {
    filterOptions,
    scripthubState,
  } from '../../stores/scripthubState.svelte.js';
  import { appendLog } from '../../stores/editorState.svelte.js';

  let isOpen = $state(false);

  function toggleDropdown(e) {
    e.stopPropagation();
    isOpen = !isOpen;
  }

  function selectFilter(opt, e) {
    e.stopPropagation();
    scripthubState.filter = opt.id;
    isOpen = false;
    appendLog(`filter changed to ${opt.label}`);
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
    class="scripthub-btn filter-dropdown"
    class:open={isOpen}
    id="btn-scripthub-filter"
    onclick={toggleDropdown}
  >
    <svg class="filter-icon" viewBox="0 0 24 24">
      <path d="M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z" />
    </svg>
    <span>
      {filterOptions.find((f) => f.id === scripthubState.filter)?.label || 'All Scripts'}
    </span>
    <svg class="chevron-icon" viewBox="0 0 24 24">
      <path d="M7 10l5 5 5-5z" />
    </svg>
  </button>

  {#if isOpen}
    <div class="scripthub-dropdown-menu show" id="menu-scripthub-filter">
      {#each filterOptions as opt (opt.id)}
        <button
          type="button"
          class="dropdown-option"
          class:active={scripthubState.filter === opt.id}
          onclick={(e) => selectFilter(opt, e)}
        >
          {opt.label}
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

  .scripthub-btn.filter-dropdown {
    padding: 0 6px;
    gap: 4px;
  }

  .scripthub-btn.filter-dropdown .filter-icon {
    width: 11px;
    height: 11px;
    fill: currentColor;
  }

  .scripthub-btn.filter-dropdown .chevron-icon {
    width: 10px;
    height: 10px;
    fill: currentColor;
    margin-left: 2px;
  }

  .scripthub-btn.filter-dropdown span {
    transform: translateY(0.5px);
    white-space: nowrap;
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
</style>
