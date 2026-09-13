<script>
  import { editorState, clearLogs } from '../../stores/editorState.svelte.js';

  let outputContainer = $state(null);

  $effect(() => {
    if (editorState.logs.length && outputContainer) {
      setTimeout(() => {
        outputContainer.scrollTop = outputContainer.scrollHeight;
      }, 10);
    }
  });

  function handleClose() {
    editorState.consoleOpen = false;
  }
</script>

<div class="console-drawer" class:open={editorState.consoleOpen}>
  <div class="console-header">
    <div class="console-title">
      <span class="console-title-accent">&gt;</span> Console Output
    </div>
    <div class="console-actions">
      <button class="console-btn" id="btn-console-clear" title="Clear Console" onclick={clearLogs}>Clear</button>
      <button class="console-btn" id="btn-console-close" title="Close Drawer" onclick={handleClose}>✕</button>
    </div>
  </div>

  <div class="console-output" bind:this={outputContainer}>
    {#each editorState.logs as log, i (i)}
      <div class="log-line">
        <span class="log-tag">[rustine] </span>
        <span class="log-text">{log}</span>
      </div>
    {/each}
  </div>
</div>

<style>
  .console-drawer {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 130px;
    background-color: #1e1e1e;
    border-top: 1px solid #4a4a4a;
    display: flex;
    flex-direction: column;
    transform: translateY(100%);
    transition: transform 0.2s cubic-bezier(0.1, 0.9, 0.2, 1);
    z-index: 10;
    box-shadow: 0 -3px 8px rgba(0, 0, 0, 0.45);
  }

  .console-drawer.open {
    transform: translateY(0);
  }

  .console-header {
    height: 20px;
    background: #171717;
    border-bottom: 1px solid #101010;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 4px 0 6px;
    flex-shrink: 0;
  }

  .console-title {
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    color: #999999;
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .console-title-accent {
    color: #ffcec6;
  }

  .console-actions {
    display: flex;
    align-items: center;
    gap: 3px;
  }

  .console-btn {
    height: 16px;
    padding: 0 4px;
    background: #242424;
    border-top: 1px solid #424242;
    border-left: 1px solid #424242;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
    color: #aaaaaa;
    font-family: 'ProggyClean', monospace;
    font-size: 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .console-btn:hover {
    background: #2e2e2e;
    color: #ffffff;
  }

  .console-btn:active {
    border-top: 1px solid #101010;
    border-left: 1px solid #101010;
    border-right: 1px solid #424242;
    border-bottom: 1px solid #424242;
  }

  .console-output {
    flex: 1;
    background-color: #141414;
    border-top: 1px solid #0d0d0d;
    border-left: 1px solid #0d0d0d;
    border-right: 1px solid #292929;
    border-bottom: 1px solid #292929;
    margin: 2px 3px 3px 3px;
    padding: 4px 6px;
    overflow-y: auto;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 16px;
    user-select: text;
    -webkit-user-select: text;
  }

  .console-output::-webkit-scrollbar {
    width: 10px;
  }

  .console-output::-webkit-scrollbar-track {
    background: #191919;
    border-left: 1px solid #0d0d0d;
  }

  .console-output::-webkit-scrollbar-thumb {
    background: #2b2b2b;
    border-top: 1px solid #3d3d3d;
    border-left: 1px solid #3d3d3d;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
  }

  .log-line {
    white-space: pre-wrap;
    word-break: break-all;
    margin-bottom: 1px;
  }

  .log-tag {
    color: #ffcec6;
    font-weight: bold;
  }

  .log-text {
    color: #dedede;
  }
</style>
