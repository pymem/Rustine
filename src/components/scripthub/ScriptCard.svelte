<script>
  import { addScriptTab, appendLog } from '../../stores/editorState.svelte.js';
  import { appState } from '../../stores/appState.svelte.js';

  let { script } = $props();

  function copyScript() {
    const code = script.code || `print("${script.name}")`;
    navigator.clipboard.writeText(code);
    appendLog(`copied ${script.name} to clipboard.`);
  }

  function loadToEditor() {
    const cleanTitle = `${script.name.replace(/[^a-zA-Z0-9_-]/g, '_').substring(0, 16)}.lua`;
    const code = script.code || `-- ${script.name}\nprint("${script.name} loaded")`;
    addScriptTab(cleanTitle, code);
    appState.activeTab = 'editor';
    appendLog(`loaded ${script.name} into editor.`);
  }

  function runScript() {
    appendLog(`executing ${script.name}...`);
  }
</script>

<div class="script-card">
  <div class="script-card-title-row">
    <span class="script-card-title" title={script.name}>{script.name}</span>
  </div>

  <div class="script-card-meta-row">
    {#if script.game}
      <span class="script-game-badge" title={script.game}>{script.game}</span>
    {/if}
    {#if script.verified}
      <span class="script-status-badge verified">Verified</span>
    {/if}
    {#if script.key === false}
      <span class="script-status-badge keyless">Keyless</span>
    {:else if script.key === true}
      <span class="script-status-badge key">Key</span>
    {/if}
    {#if script.isPaid}
      <span class="script-status-badge paid">Paid</span>
    {/if}
  </div>

  <div class="script-card-author-row">
    <span class="script-card-author" title={script.author}>{script.author}</span>
    <span class="views-text">{script.views} views</span>
  </div>

  <div class="script-card-footer">
    <button
      class="script-card-btn icon-only"
      title="Copy Script Code"
      onclick={copyScript}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path
          d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"
        />
      </svg>
    </button>
    <button class="script-card-btn" onclick={loadToEditor}>Load</button>
    <button class="script-card-btn primary" onclick={runScript}>Run</button>
  </div>
</div>

<style>
  .script-card {
    background: #252525;
    border-top: 1px solid #444444;
    border-left: 1px solid #444444;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
    padding: 7px 9px;
    display: flex;
    flex-direction: column;
    gap: 5px;
    cursor: default;
    transition: background 0.1s ease, border-color 0.1s ease;
  }

  .script-card:hover {
    background: #2a2a2a;
    border-top: 1px solid #555555;
    border-left: 1px solid #555555;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
  }

  .script-card-title-row {
    width: 100%;
    overflow: hidden;
  }

  .script-card-title {
    font-family: 'ProggyClean', monospace;
    font-size: 15px;
    line-height: 16px;
    font-weight: bold;
    color: #ffffff;
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    letter-spacing: 0.2px;
  }

  .script-card-meta-row {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-wrap: wrap;
  }

  .script-game-badge {
    font-family: 'ProggyClean', monospace;
    font-size: 12px;
    line-height: 12px;
    color: #ffcec6;
    background: #161616;
    padding: 1px 4px;
    border-top: 1px solid #0d0d0d;
    border-left: 1px solid #0d0d0d;
    border-right: 1px solid #333333;
    border-bottom: 1px solid #333333;
    max-width: 110px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .script-status-badge {
    font-family: 'ProggyClean', monospace;
    font-size: 11px;
    line-height: 11px;
    padding: 1px 4px;
    border: 1px solid;
    border-radius: 0;
    white-space: nowrap;
  }

  .script-status-badge.verified {
    color: #7ee787;
    border-color: #238636;
    background: #0f2316;
  }

  .script-status-badge.keyless {
    color: #79c0ff;
    border-color: #1f6feb;
    background: #0d1e38;
  }

  .script-status-badge.key {
    color: #e3b341;
    border-color: #9e6a03;
    background: #2b1f06;
  }

  .script-status-badge.paid {
    color: #ff7b72;
    border-color: #da3633;
    background: #2b1012;
  }

  .script-card-author-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: #999999;
    font-size: 12px;
    line-height: 12px;
    margin-top: 1px;
  }

  .script-card-author {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: #a6a6a6;
    max-width: 110px;
  }

  .views-text {
    font-size: 12px;
    color: #888888;
    flex-shrink: 0;
  }

  .script-card-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 5px;
    margin-top: 2px;
    padding-top: 5px;
    border-top: 1px solid #1c1c1c;
  }

  .script-card-btn {
    height: 19px;
    padding: 0 8px;
    background: #222222;
    border-top: 1px solid #484848;
    border-left: 1px solid #484848;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
    color: #d4d4d4;
    font-family: 'ProggyClean', monospace;
    font-size: 12px;
    line-height: 12px;
    cursor: pointer;
    transition: all 0.1s ease;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .script-card-btn:hover {
    background: #2c2c2c;
    color: #ffffff;
  }

  .script-card-btn:active {
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #484848;
    border-bottom: 1px solid #484848;
    background: #191919;
  }

  .script-card-btn.primary {
    background: #2d1d20;
    border-top: 1px solid #6b3f42;
    border-left: 1px solid #6b3f42;
    border-right: 1px solid #14090a;
    border-bottom: 1px solid #14090a;
    color: #ffcec6;
    font-weight: bold;
  }

  .script-card-btn.primary:hover {
    background: #3e2629;
    color: #ffffff;
  }

  .script-card-btn.icon-only {
    width: 20px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .script-card-btn.icon-only svg {
    width: 11px;
    height: 11px;
    fill: currentColor;
  }
</style>
