export function getWin() {
  if (typeof window !== 'undefined' && window.__TAURI__) {
    if (window.__TAURI__.webviewWindow && window.__TAURI__.webviewWindow.getCurrentWebviewWindow) {
      return window.__TAURI__.webviewWindow.getCurrentWebviewWindow();
    }
    if (window.__TAURI__.window && window.__TAURI__.window.getCurrentWindow) {
      return window.__TAURI__.window.getCurrentWindow();
    }
    if (window.__TAURI__.getCurrentWindow) {
      return window.__TAURI__.getCurrentWindow();
    }
  }
  return null;
}

export async function startDragging() {
  const win = getWin();
  if (win && win.startDragging) {
    try {
      await win.startDragging();
    } catch (err) {
      console.warn('startDragging error:', err);
    }
  }
}

export function minimizeWindow() {
  const win = getWin();
  if (win && win.minimize) {
    win.minimize();
  }
}

export function closeWindow() {
  const win = getWin();
  if (win && win.close) {
    win.close();
  }
}

export async function apiGet(url, timeoutMs = 3500) {
  const timeoutPromise = new Promise((resolve) =>
    setTimeout(() => resolve(null), timeoutMs)
  );

  const fetchPromise = (async () => {
    try {
      if (typeof window !== 'undefined' && window.__TAURI__) {
        const invoke = window.__TAURI__.core?.invoke || window.__TAURI__.invoke;
        if (invoke) {
          const raw = await invoke('http_get', { url });
          if (typeof raw === 'string') {
            return JSON.parse(raw);
          }
          return raw;
        }
      }
    } catch (tauriErr) {
      console.warn('Tauri http_get error, falling back to fetch:', tauriErr);
    }

    try {
      const res = await fetch(url, { headers: { Accept: 'application/json' } });
      if (!res.ok) return null;
      return await res.json();
    } catch (fetchErr) {
      return null;
    }
  })();

  return Promise.race([fetchPromise, timeoutPromise]);
}

export async function openUrl(url) {
  try {
    if (typeof window !== 'undefined' && window.__TAURI__) {
      const invoke = window.__TAURI__.core?.invoke || window.__TAURI__.invoke;
      if (invoke) {
        await invoke('open_url', { url });
        return;
      }
    }
  } catch (err) {
    console.warn('open_url error:', err);
  }
  window.open(url, '_blank');
}

export async function listSavedScripts() {
  try {
    if (typeof window !== 'undefined' && window.__TAURI__) {
      const invoke = window.__TAURI__.core?.invoke || window.__TAURI__.invoke;
      if (invoke) {
        return await invoke('list_saved_scripts');
      }
    }
  } catch (err) {
    console.warn('list_saved_scripts error:', err);
  }
  return [];
}

export async function saveScriptToDisk(name, content) {
  try {
    if (typeof window !== 'undefined' && window.__TAURI__) {
      const invoke = window.__TAURI__.core?.invoke || window.__TAURI__.invoke;
      if (invoke) {
        await invoke('save_script', { name, content });
      }
    }
  } catch (err) {
    console.warn('save_script error:', err);
  }
}

export async function deleteScriptFromDisk(name) {
  try {
    if (typeof window !== 'undefined' && window.__TAURI__) {
      const invoke = window.__TAURI__.core?.invoke || window.__TAURI__.invoke;
      if (invoke) {
        await invoke('delete_script', { name });
      }
    }
  } catch (err) {
    console.warn('delete_script error:', err);
  }
}

export async function inject() {
  const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
  if (invoke) return await invoke('inject');
  return null;
}

export async function getGameId() {
  try {
    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
    if (!invoke) return null;
    const id = await invoke('get_game_id');
    const num = Number(id);
    if (!Number.isFinite(num) || num <= 0) return null;
    return Math.floor(num);
  } catch {
    return null;
  }
}

export async function isRobloxOpen() {
  try {
    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
    if (invoke) return await invoke('roblox_open');
  } catch {
    return false;
  }
  return false;
}

export async function getStatus() {
  try {
    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
    if (invoke) return await invoke('get_status');
  } catch {
    return 'not_found';
  }
  return 'not_found';
}

export async function getOffset(path) {
  try {
    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
    if (!invoke) return null;
    return await invoke('get_offset', { path });
  } catch {
    return null;
  }
}

export async function getFflag(name) {
  try {
    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
    if (!invoke) return null;
    return await invoke('get_fflag', { name });
  } catch {
    return null;
  }
}

export async function getOffsetsStatus() {
  try {
    const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
    if (invoke) return await invoke('get_offsets_status');
  } catch {
    return null;
  }
  return null;
}

export async function refreshOffsets() {
  const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;
  if (invoke) return await invoke('refresh_offsets');
  return null;
}
