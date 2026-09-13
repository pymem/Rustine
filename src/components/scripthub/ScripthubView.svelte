<script>
  import { onMount } from 'svelte';
  import ProviderDropdown from './ProviderDropdown.svelte';
  import FilterDropdown from './FilterDropdown.svelte';
  import ScriptCard from './ScriptCard.svelte';
  import {
    scripthubState,
    fetchScripts,
    refreshGameId,
  } from '../../stores/scripthubState.svelte.js';

  let searchTimer = null;
  let contentEl = $state(null);

  let filteredScripts = $derived(
    scripthubState.scripts.filter((s) => {
      const f = scripthubState.filter;
      if (f === 'verified') return s.verified;
      if (f === 'keyless') return s.key === false;
      if (f === 'key') return s.key === true;
      if (f === 'paid') return s.isPaid;
      if (f === 'universal') {
        const g = (s.game || '').toLowerCase();
        const n = (s.name || '').toLowerCase();
        return g.includes('universal') || n.includes('universal');
      }
      return true;
    })
  );

  function handleInput(e) {
    scripthubState.query = e.target.value;
    clearTimeout(searchTimer);
    searchTimer = setTimeout(() => {
      fetchScripts(false);
    }, 300);
  }

  function handleClear() {
    scripthubState.query = '';
    fetchScripts(false);
  }

  function handleScroll() {
    if (!contentEl || scripthubState.loading || scripthubState.loadingMore || !scripthubState.hasMore) return;
    if (contentEl.scrollTop + contentEl.clientHeight >= contentEl.scrollHeight - 60) {
      scripthubState.page++;
      fetchScripts(true);
    }
  }

  onMount(async () => {
    await refreshGameId();
    if (scripthubState.scripts.length === 0 && !scripthubState.loading) {
      fetchScripts(false);
    }
  });

  async function handleRefreshGame() {
    await refreshGameId();
    fetchScripts(false);
  }
</script>

<div class="scripthub-layout">
  <div class="scripthub-toolbar">
    <div class="scripthub-search-box">
      <div class="scripthub-search-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <path
            d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"
          />
        </svg>
      </div>
      <input
        type="text"
        placeholder="Search..."
        value={scripthubState.query}
        oninput={handleInput}
      />
      {#if scripthubState.query.length > 0}
        <button
          id="scripthub-search-clear"
          title="Clear search"
          onclick={handleClear}
        >
          ✕
        </button>
      {/if}
    </div>

    <ProviderDropdown />
    <FilterDropdown />
    <button
      id="scripthub-game-badge"
      title="Refresh current GameId from DataModel"
      onclick={handleRefreshGame}
    >
      {#if scripthubState.gameIdStatus === 'live' && scripthubState.gameId}
        Game: {scripthubState.gameId}
      {:else if scripthubState.gameIdStatus === 'loading'}
        Game: ...
      {:else}
        Universal
      {/if}
    </button>
  </div>

  <div
    class="scripthub-content"
    bind:this={contentEl}
    onscroll={handleScroll}
  >
    {#if scripthubState.loading}
      {#each Array(6) as _, i (i)}
        <div class="script-card skeleton">
          <div class="skeleton-line title"></div>
          <div class="skeleton-line sub"></div>
          <div
            style="display: flex; justify-content: space-between; align-items: center; margin-top: 6px; padding-top: 4px; border-top: 1px solid #1c1c1c;"
          >
            <div class="skeleton-line tiny"></div>
            <div class="skeleton-line btn"></div>
          </div>
        </div>
      {/each}
    {:else if filteredScripts.length === 0}
      <div
        style="grid-column: 1 / -1; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 160px; color: #777777; gap: 8px;"
      >
        <span style="font-size: 16px;">No scripts found</span>
      </div>
    {:else}
      {#each filteredScripts as script (script.id)}
        <ScriptCard {script} />
      {/each}

      {#if scripthubState.loadingMore}
        {#each Array(3) as _, i (i)}
          <div class="script-card skeleton">
            <div class="skeleton-line title"></div>
            <div class="skeleton-line sub"></div>
          </div>
        {/each}
      {/if}
    {/if}
  </div>
</div>

<style>
  .scripthub-layout {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: #212121;
    overflow: hidden;
  }

  .scripthub-toolbar {
    height: 32px;
    background-color: #1a1a1a;
    border-bottom: 1px solid #141414;
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0 5px;
    flex-shrink: 0;
    position: relative;
    z-index: 100;
  }

  .scripthub-search-box {
    flex: 1;
    height: 22px;
    background: #141414;
    border-top: 1px solid #0d0d0d;
    border-left: 1px solid #0d0d0d;
    border-right: 1px solid #383838;
    border-bottom: 1px solid #383838;
    display: flex;
    align-items: center;
    padding: 0 5px;
    gap: 5px;
    min-width: 0;
  }

  .scripthub-search-icon {
    width: 12px;
    height: 12px;
    color: #777777;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .scripthub-search-icon svg {
    width: 100%;
    height: 100%;
    fill: currentColor;
  }

  .scripthub-search-box input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: #dedede;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    min-width: 0;
  }

  .scripthub-search-box input::placeholder {
    color: #666666;
  }

  #scripthub-search-clear {
    background: transparent;
    border: none;
    color: #777777;
    cursor: pointer;
    font-size: 10px;
    padding: 0 2px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  #scripthub-search-clear:hover {
    color: #ff7b72;
  }

  #scripthub-game-badge {
    height: 22px;
    background: #242424;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #c2bebe;
    font-family: 'ProggyClean', monospace;
    font-size: 13px;
    padding: 0 6px;
    cursor: pointer;
    white-space: nowrap;
  }

  #scripthub-game-badge:hover {
    background-color: #2e2e2e;
    color: #ffffff;
  }

  .scripthub-content {
    flex: 1;
    padding: 8px;
    overflow-y: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 7px;
    align-content: start;
  }

  .scripthub-content::-webkit-scrollbar {
    width: 8px;
  }

  .scripthub-content::-webkit-scrollbar-track {
    background: #181818;
  }

  .scripthub-content::-webkit-scrollbar-thumb {
    background: #282828;
    border: 1px solid #141414;
  }

  .script-card.skeleton {
    pointer-events: none;
    opacity: 0.55;
    background: #252525;
    border-top: 1px solid #444444;
    border-left: 1px solid #444444;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
    padding: 7px 9px;
  }

  .skeleton-line {
    background: #333333;
    height: 9px;
    margin-bottom: 5px;
    animation: skeletonPulse 1.2s infinite ease-in-out;
  }

  .skeleton-line.title {
    width: 75%;
    height: 12px;
  }

  .skeleton-line.sub {
    width: 50%;
  }

  .skeleton-line.tiny {
    width: 30%;
    margin-bottom: 0;
  }

  .skeleton-line.btn {
    width: 45px;
    height: 14px;
    margin-bottom: 0;
  }

  @keyframes skeletonPulse {
    0%,
    100% {
      opacity: 0.35;
    }
    50% {
      opacity: 0.75;
    }
  }
</style>
