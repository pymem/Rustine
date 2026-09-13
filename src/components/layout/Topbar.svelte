<script>
  import { startDragging, minimizeWindow, closeWindow } from '../../lib/tauri.js';

  async function handleMouseDown(e) {
    if (e.target.closest('.win-btn') || e.target.closest('button')) return;
    if (e.button === 0) {
      await startDragging();
    }
  }
</script>

<div role="toolbar" tabindex="0" class="topbar" id="app-topbar" data-tauri-drag-region onmousedown={handleMouseDown}>
  <div class="topbar-left" data-tauri-drag-region>
    <div class="brand-icon" data-tauri-drag-region>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 115.4 136">
        <defs>
          <linearGradient id="topbar_grad" x1="57.61" x2="57.61" y1="132.3" y2="3.397" gradientUnits="userSpaceOnUse">
            <stop stop-color="#7C6363" offset="0" />
            <stop stop-color="#FFC9C2" offset="1" />
          </linearGradient>
        </defs>
        <path fill="url(#topbar_grad)"
          d="m7 9c3-3.5 7-5.4 15.6-5.6l59.1-0.1c4.8 0.1 10.2 1.3 12.3 2.2 3 1.3 6.7 3.9 8.5 6.2s3.4 6.3 3.8 9.7c0.3 1.7 0.8 5.2 0.7 10.1v9.5l-1.2 8.8-1.4 5.3c-2.3 5.6-5.7 9.9-9.2 14.2l-3.6 4.3-1.1 3.8c0.5 3.5 3.3 6.7 4.5 8.3 4.2 4.9 13.4 17.2 16.3 26.9 0.7 3.9 1.6 8.1 1.2 12.1l-1.2 3.6c-1.3 1.6-2.4 2.1-3.3 2.8-3.2 1-4.8 1.1-8.9 1.2h-18.6c-3.2 0-8.4-0.3-11.2-1-2.4-1.2-4-2.8-4.6-5.3-0.5-2.5-1-5.1-1-10l-0.2-11.4-0.8-4c-0.3-3.3-0.9-5.7-1.8-9.6-0.8-1.6-2.1-6.3-5.1-7.4-2.4-0.5-4.4-0.1-6.1 1.4l-1.1 3.1 1.6 6.8 4.3 15.1c0.8 5.6 2.3 11-1.6 17.2-1.1 1.7-2.5 2.7-3.9 3.8-3.4 1.1-10 1.2-14 1.3h-17c-2.7 0-5.9-0.5-8.9-1-2.5-1.3-5.1-3.5-6-7-0.9-5.3-1.2-9.4-0.5-14.9l0.8-13.7 0.7-6.6 0.4-9.1 0.9-10.6c0.4-7.8-1.1-13.8-1.1-20.8l-0.6-4.7-1.1-15.9c-0.1-4.2 0.3-7.1 0.5-11.1 0.2-2.6 2.6-6.4 3.9-7.9zm43.8 18.2c-1.7 4-1.7 8.8-1.7 15.3 0.2 2.1 1 4.3 2 6.1 1.6 1.1 3.6 1.8 7.1 1.5 2.7-1 4.1-2.2 4.8-5.2l0.5-1.4c0.3-1.7 0.1-6.5 0-9.1l-1.1-4.8c-1.1-2.4-3.1-4.9-6.3-4.9-2.5 0-3.7 0.8-5.3 2.5z" />
        <path fill="none" stroke="#3F3738" stroke-width="1.0992"
          d="m11.6 5.5c-5 3.1-7.6 8.1-8.2 10.6l-0.6 5.3v5.9l0.6 6.2c0 2.7 0.6 11 1.3 14.7 0.1 7.9 1.5 13.8 1.1 20.8l-0.9 11-0.2 7.7-0.9 8.4-0.7 12.9c-0.8 5.2-0.6 9.6 0.3 14.6 1.1 3.3 3.5 5.4 6 6.7 2.6 0.6 6.1 1.4 11.1 1.5h12.7c5.4 0 11-0.1 16.2-1.2" />
        <path opacity="0.3" fill="none" stroke="#BCA5A5" stroke-width="1.0992"
          d="m52.9 24.9c1.1-0.5 3.6-1.1 5.5-0.4 2.6 1.1 4.3 3.4 4.9 5.5 0.5 1.6 0.9 3.4 0.9 5.7s0.1 5.8-0.4 7.3l-1.1 3.1c-1.1 2.7-2.4 3.3-3.7 3.8" />
        <path fill="none" stroke="#3F3738" stroke-width="1.0992"
          d="m49.6 84.6c1.8-1.1 2.9-1.4 5.5-1.1 3.2 0.4 4.8 4 6 7.1l1.6 6.8 0.8 6.6 0.3 13.8c0.2 2.7 0.6 6.1 1.4 8.7 0.9 1.8 2.3 3.5 4.7 4.1 4.1 0.8 6.9 1.1 11 1.1l20-0.1c3 0 5.1-0.5 8.1-1" />
        <path opacity="0.3" fill="none" stroke="#BCA5A5" stroke-width="1.0992"
          d="m49.4 27.4c-0.9 2-1.5 6.1-1.4 8.5v7.7c0.3 2.3 1.3 4.1 1.9 5.2 2.6 2.9 5.2 2.4 8.5 2.2" />
        <path opacity="0.3" fill="none" stroke="#BCA5A5" stroke-width="1.0992"
          d="m105.1 50.6c-2.2 9.4-8.8 15.8-13.8 22.9-1.3 2.9-2.9 3.9 0.6 9.5l9 11.2c3.2 4.3 7.7 12.4 9.8 17.8 1.2 4.1 1.8 7 1.8 12.1-1 3.7-2 4.9-3.5 6.3" />
        <path opacity="0.3" fill="none" stroke="#BCA5A5" stroke-width="1.0992"
          d="m48.5 84.7c-1.8 4.2-0.2 5.9 2.3 15.9l2.1 6.4 1.7 7.6c1.2 6.1-0.5 11.3-4.7 15.8" />
      </svg>
    </div>
  </div>

  <div class="window-controls">
    <button class="win-btn" id="btn-minimize" title="Minimize" onclick={minimizeWindow}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path d="M4 11h16v2H4z" />
      </svg>
    </button>
    <button class="win-btn close" id="btn-close" title="Close" onclick={closeWindow}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path d="M7 19H5V17H7V19ZM19 19H17V17H19V19ZM9 15V17H7V15H9ZM17 17H15V15H17V17ZM11 15H9V13H11V15ZM15 15H13V13H15V15ZM13 13H11V11H13V13ZM11 11H9V9H11V11ZM15 11H13V9H15V11ZM9 9H7V7H9V9ZM17 9H15V7H17V9ZM7 7H5V5H7V7ZM19 7H17V5H19V7Z" />
      </svg>
    </button>
  </div>
</div>

<style>
  .topbar {
    height: 26px;
    width: 100%;
    background: #1c1c1c;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 3px 0 0;
    flex-shrink: 0;
    cursor: default;
    border-bottom: 1px solid #141414;
    -webkit-app-region: drag;
    user-select: none;
    -webkit-user-select: none;
  }

  .topbar-left {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    padding-left: 12px;
    height: 100%;
    flex-shrink: 0;
    -webkit-app-region: drag;
  }

  .brand-icon {
    width: 16px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    -webkit-app-region: drag;
  }

  .brand-icon svg {
    width: 100%;
    height: 100%;
    display: block;
    pointer-events: none;
  }

  .window-controls {
    display: flex;
    align-items: center;
    gap: 2px;
    -webkit-app-region: no-drag;
  }

  .win-btn {
    width: 20px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #262626;
    border-top: 1px solid #4a4a4a;
    border-left: 1px solid #4a4a4a;
    border-right: 1px solid #141414;
    border-bottom: 1px solid #141414;
    color: #b0b0b0;
    cursor: pointer;
    border-radius: 0;
    padding: 0;
    -webkit-app-region: no-drag;
  }

  .win-btn svg {
    width: 12px;
    height: 12px;
    fill: currentColor;
  }

  .win-btn:active {
    border-top: 1px solid #141414;
    border-left: 1px solid #141414;
    border-right: 1px solid #4a4a4a;
    border-bottom: 1px solid #4a4a4a;
    padding-top: 1px;
    padding-left: 1px;
  }

  .win-btn:hover {
    background-color: #2e2e2e;
    color: #ffffff;
  }

  .win-btn.close:hover {
    background-color: #8c3636;
    color: #ffffff;
  }
</style>
