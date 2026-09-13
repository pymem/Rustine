<script>
  import {
    settingsState,
    updateSetting,
  } from '../stores/settingsState.svelte.js';
  import { appendLog } from '../stores/editorState.svelte.js';

  let activeSubTab = $state('Editor');
  const subTabs = ['Editor', 'Appearance'];

  function handleCheckbox(key, e) {
    updateSetting(key, e.target.checked);
    appendLog(`setting '${key}' set to ${e.target.checked}`);
  }

  function handleSelect(key, e) {
    const val = isNaN(e.target.value) ? e.target.value : Number(e.target.value);
    updateSetting(key, val);
    appendLog(`setting '${key}' set to ${val}`);
  }

  function selectTheme(themeId) {
    updateSetting('theme', themeId);
    appendLog(`theme changed to ${themeId}`);
  }
</script>

<div class="settings-dialog">
  <div class="win95-property-sheet">
    <div class="win95-tab-strip">
      {#each subTabs as tab (tab)}
        <button
          type="button"
          class="win95-tab"
          class:active={activeSubTab === tab}
          onclick={() => (activeSubTab = tab)}
        >
          <span>{tab}</span>
        </button>
      {/each}
    </div>

    <div class="win95-sheet-body">
      {#if activeSubTab === 'Editor'}
        <div class="sheet-content editor-settings-grid">
          <fieldset class="win95-groupbox">
            <legend>Display & Layout</legend>
            <div class="groupbox-content">
              <label class="win95-checkbox">
                <input
                  type="checkbox"
                  checked={settingsState.lineNumbers}
                  onchange={(e) => handleCheckbox('lineNumbers', e)}
                />
                <span class="win95-check-box"></span>
                <span class="win95-check-label">Show line numbers</span>
              </label>

              <label class="win95-checkbox">
                <input
                  type="checkbox"
                  checked={settingsState.minimap}
                  onchange={(e) => handleCheckbox('minimap', e)}
                />
                <span class="win95-check-box"></span>
                <span class="win95-check-label">Minimap</span>
              </label>

              <label class="win95-checkbox">
                <input
                  type="checkbox"
                  checked={settingsState.wordWrap}
                  onchange={(e) => handleCheckbox('wordWrap', e)}
                />
                <span class="win95-check-box"></span>
                <span class="win95-check-label">Enable word wrap</span>
              </label>

              <label class="win95-checkbox">
                <input
                  type="checkbox"
                  checked={settingsState.smoothScrolling}
                  onchange={(e) => handleCheckbox('smoothScrolling', e)}
                />
                <span class="win95-check-box"></span>
                <span class="win95-check-label">Smooth scrolling</span>
              </label>
            </div>
          </fieldset>

          <fieldset class="win95-groupbox">
            <legend>Font & Formatting</legend>
            <div class="groupbox-content rows">
              <div class="setting-row">
                <label for="setting-font-size">Font Size:</label>
                <select
                  id="setting-font-size"
                  class="win95-select"
                  value={settingsState.fontSize}
                  onchange={(e) => handleSelect('fontSize', e)}
                >
                  <option value={12}>12 px</option>
                  <option value={14}>14 px</option>
                  <option value={16}>16 px (Default)</option>
                  <option value={18}>18 px</option>
                  <option value={20}>20 px</option>
                  <option value={22}>22 px</option>
                </select>
              </div>
            </div>
          </fieldset>
        </div>
      {:else if activeSubTab === 'Appearance'}
        <div class="sheet-content appearance-settings-grid">
          <fieldset class="win95-groupbox">
            <legend>Color Scheme</legend>
            <div class="theme-preview-buttons-grid">
              <button
                type="button"
                class="theme-preview-btn"
                class:active={settingsState.theme === 'rustine-dark'}
                onclick={() => selectTheme('rustine-dark')}
              >
                <div class="mini-window rustine-dark-theme">
                  <div class="mini-titlebar">
                    <span class="mini-title">Rustine</span>
                    <div class="mini-controls">
                      <span>_</span>
                      <span>✕</span>
                    </div>
                  </div>
                  <div class="mini-body">
                    <div class="mini-sidebar">
                      <div class="mini-tab-icon active">E</div>
                      <div class="mini-tab-icon">S</div>
                    </div>
                    <div class="mini-main">
                      <div class="mini-editor-tabs">
                        <div class="mini-tab active">Script1.lua</div>
                      </div>
                      <div class="mini-code">
                        <span class="kw">print</span>(<span class="str">"rustine"</span>)
                      </div>
                    </div>
                  </div>
                </div>
                <div class="theme-label">
                  <span class="radio-indicator"></span>
                  <span class="theme-name">Rustine Dark</span>
                </div>
              </button>

              <button
                type="button"
                class="theme-preview-btn"
                class:active={settingsState.theme === 'win95-classic'}
                onclick={() => selectTheme('win95-classic')}
              >
                <div class="mini-window win95-classic-theme">
                  <div class="mini-titlebar">
                    <span class="mini-title">Rustine</span>
                    <div class="mini-controls">
                      <span>_</span>
                      <span>✕</span>
                    </div>
                  </div>
                  <div class="mini-body">
                    <div class="mini-sidebar">
                      <div class="mini-tab-icon active">E</div>
                      <div class="mini-tab-icon">S</div>
                    </div>
                    <div class="mini-main">
                      <div class="mini-editor-tabs">
                        <div class="mini-tab active">Script1.lua</div>
                      </div>
                      <div class="mini-code">
                        <span class="kw">print</span>(<span class="str">"rustine"</span>)
                      </div>
                    </div>
                  </div>
                </div>
                <div class="theme-label">
                  <span class="radio-indicator"></span>
                  <span class="theme-name">Win95</span>
                </div>
              </button>
            </div>
          </fieldset>
        </div>
      {/if}
    </div>
  </div>

  <div class="win95-dialog-footer">
    <button class="win95-btn" onclick={() => appendLog('settings applied.')}>OK</button>
    <button class="win95-btn">Cancel</button>
  </div>
</div>

<style>
  .settings-dialog {
    width: 100%;
    height: 100%;
    background-color: #212121;
    display: flex;
    flex-direction: column;
    padding: 8px 10px 8px 10px;
    gap: 8px;
    box-sizing: border-box;
    font-family: 'ProggyClean', monospace;
    overflow: hidden;
  }

  .win95-property-sheet {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .win95-tab-strip {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    padding-left: 4px;
    position: relative;
    z-index: 2;
  }

  .win95-tab {
    height: 22px;
    padding: 0 10px;
    background-color: #1e1e1e;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #888888;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 14px;
    line-height: 14px;
    user-select: none;
    transition: background 0.1s ease, color 0.1s ease;
  }

  .win95-tab:hover {
    background-color: #262626;
    color: #c0c0c0;
  }

  .win95-tab.active {
    height: 24px;
    background-color: #262626;
    border-top: 1px solid #555555;
    border-left: 1px solid #555555;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #262626;
    color: #ffcec6;
    margin-bottom: -1px;
    z-index: 3;
  }

  .win95-tab span {
    transform: translateY(0.5px);
  }

  .win95-sheet-body {
    flex: 1;
    background-color: #262626;
    border-top: 1px solid #555555;
    border-left: 1px solid #555555;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    padding: 10px;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    position: relative;
    z-index: 1;
  }

  .win95-sheet-body::-webkit-scrollbar {
    width: 6px;
  }

  .win95-sheet-body::-webkit-scrollbar-track {
    background: #191919;
    border-left: 1px solid #141414;
  }

  .win95-sheet-body::-webkit-scrollbar-thumb {
    background: #2b2b2b;
    border-top: 1px solid #3d3d3d;
    border-left: 1px solid #3d3d3d;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
  }

  .win95-sheet-body::-webkit-scrollbar-thumb:hover {
    background: #363636;
  }

  .sheet-content {
    flex: 1;
    width: 100%;
    height: 100%;
  }

  .editor-settings-grid,
  .appearance-settings-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .win95-groupbox {
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    padding: 8px 10px 10px 10px;
    margin: 0;
  }

  .win95-groupbox legend {
    color: #ffcec6;
    font-size: 14px;
    line-height: 14px;
    padding: 0 4px;
  }

  .groupbox-content {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 2px;
  }

  .groupbox-content.rows {
    gap: 8px;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    max-width: 320px;
  }

  .setting-row label {
    font-size: 14px;
    color: #cccccc;
    user-select: none;
  }

  .win95-checkbox {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    user-select: none;
    font-size: 14px;
    color: #cccccc;
  }

  .win95-checkbox input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .win95-check-box {
    width: 13px;
    height: 13px;
    background-color: #181818;
    border-top: 1px solid #0d0d0d;
    border-left: 1px solid #0d0d0d;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    flex-shrink: 0;
  }

  .win95-checkbox input:checked + .win95-check-box::after {
    content: '✓';
    color: #ffcec6;
    font-size: 12px;
    line-height: 12px;
    font-weight: bold;
    transform: translateY(-0.5px);
  }

  .win95-check-label {
    transform: translateY(0.5px);
  }

  .win95-select {
    height: 22px;
    background-color: #141414;
    border-top: 1px solid #0d0d0d;
    border-left: 1px solid #0d0d0d;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    color: #ffcec6 !important;
    font-family: 'ProggyClean', monospace;
    font-size: 15px;
    line-height: 20px;
    padding: 0 6px;
    outline: none;
    cursor: pointer;
    min-width: 130px;
  }

  .win95-select option {
    background-color: #212121;
    color: #ffcec6;
    font-family: 'ProggyClean', monospace;
    font-size: 15px;
    padding: 2px 4px;
  }

  .theme-preview-buttons-grid {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    padding: 4px 2px;
  }

  .theme-preview-btn {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 6px;
    background: #1e1e1e;
    border-top: 1px solid #424242;
    border-left: 1px solid #424242;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    cursor: pointer;
    user-select: none;
    transition: all 0.1s ease;
    width: 145px;
  }

  .theme-preview-btn:hover {
    background: #252525;
    border-top: 1px solid #5a5a5a;
    border-left: 1px solid #5a5a5a;
  }

  .theme-preview-btn.active {
    background: #282222;
    border-top: 1px solid #8c4a4a;
    border-left: 1px solid #8c4a4a;
    border-right: 1px solid #2b1414;
    border-bottom: 1px solid #2b1414;
    outline: 1px dotted #ffcec6;
  }

  .mini-window {
    width: 100%;
    height: 76px;
    border-radius: 0;
    display: flex;
    flex-direction: column;
    padding: 1px;
    overflow: hidden;
  }

  .mini-window.rustine-dark-theme {
    background-color: #262626;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #101010;
    border-bottom: 1px solid #101010;
  }

  .mini-window.rustine-dark-theme .mini-titlebar {
    background-color: #1c1c1c;
    color: #ffcec6;
  }

  .mini-window.rustine-dark-theme .mini-sidebar {
    background-color: #212121;
    border-right: 1px solid #141414;
  }

  .mini-window.rustine-dark-theme .mini-tab-icon.active {
    background-color: #191919;
    color: #ffcec6;
  }

  .mini-window.rustine-dark-theme .mini-main {
    background-color: #212121;
  }

  .mini-window.rustine-dark-theme .mini-editor-tabs {
    background-color: #1a1a1a;
  }

  .mini-window.rustine-dark-theme .mini-tab.active {
    background-color: #212121;
    color: #ffcec6;
  }

  .mini-window.rustine-dark-theme .mini-code {
    color: #dedede;
  }

  .mini-window.rustine-dark-theme .kw {
    color: #ffcec6;
  }

  .mini-window.rustine-dark-theme .str {
    color: #d4b8b4;
  }

  .mini-window.win95-classic-theme {
    background-color: #c0c0c0;
    border-top: 1px solid #ffffff;
    border-left: 1px solid #ffffff;
    border-right: 1px solid #000000;
    border-bottom: 1px solid #000000;
  }

  .mini-window.win95-classic-theme .mini-titlebar {
    background-color: #000080;
    color: #ffffff;
  }

  .mini-window.win95-classic-theme .mini-sidebar {
    background-color: #c0c0c0;
    border-right: 1px solid #808080;
  }

  .mini-window.win95-classic-theme .mini-tab-icon {
    color: #000000;
  }

  .mini-window.win95-classic-theme .mini-tab-icon.active {
    background-color: #000080;
    color: #ffffff;
  }

  .mini-window.win95-classic-theme .mini-main {
    background-color: #ffffff;
  }

  .mini-window.win95-classic-theme .mini-editor-tabs {
    background-color: #c0c0c0;
  }

  .mini-window.win95-classic-theme .mini-tab.active {
    background-color: #ffffff;
    color: #000080;
  }

  .mini-window.win95-classic-theme .mini-code {
    color: #000000;
  }

  .mini-window.win95-classic-theme .kw {
    color: #0000ff;
  }

  .mini-window.win95-classic-theme .str {
    color: #a31515;
  }

  .mini-titlebar {
    height: 12px;
    padding: 0 3px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 9px;
    line-height: 9px;
    font-weight: bold;
  }

  .mini-controls {
    display: flex;
    gap: 2px;
    font-size: 7px;
  }

  .mini-body {
    flex: 1;
    display: flex;
    gap: 1px;
    padding: 1px 0 0 0;
    min-height: 0;
  }

  .mini-sidebar {
    width: 22px;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1px;
    padding: 1px;
  }

  .mini-tab-icon {
    font-size: 8px;
    text-align: center;
    line-height: 10px;
    height: 10px;
  }

  .mini-main {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .mini-editor-tabs {
    height: 11px;
    display: flex;
    align-items: flex-end;
    padding: 1px 2px 0 2px;
  }

  .mini-tab {
    font-size: 8px;
    line-height: 9px;
    padding: 0 3px;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
  }

  .mini-code {
    flex: 1;
    padding: 2px 3px;
    font-size: 8px;
    line-height: 9px;
    overflow: hidden;
    white-space: nowrap;
  }

  .theme-label {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 13px;
    line-height: 13px;
    color: #cccccc;
    padding-top: 2px;
  }

  .theme-preview-btn.active .theme-name {
    color: #ffcec6;
    font-weight: bold;
  }

  .radio-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #141414;
    border: 1px solid #4a4a4a;
    display: inline-block;
  }

  .theme-preview-btn.active .radio-indicator {
    background-color: #ffcec6;
    border-color: #ffcec6;
    box-shadow: 0 0 2px #ffcec6;
  }

  .win95-dialog-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 6px;
    padding-top: 2px;
    flex-shrink: 0;
  }

  .win95-btn {
    min-width: 62px;
    height: 22px;
    padding: 0 8px;
    background: #262626;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #dedede;
    font-family: 'ProggyClean', monospace;
    font-size: 14px;
    line-height: 14px;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    outline: none;
    user-select: none;
    transition: background 0.1s ease;
  }

  .win95-btn:hover {
    background-color: #2e2e2e;
    color: #ffffff;
  }

  .win95-btn:active {
    border-top: 1px solid #141414;
    border-left: 1px solid #141414;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    padding-top: 1px;
    padding-left: 9px;
    padding-right: 7px;
  }
</style>
