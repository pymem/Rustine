import {
  listSavedScripts,
  saveScriptToDisk,
  deleteScriptFromDisk,
} from '../lib/tauri.js';

export const defaultCode = `--[[
                _   _            
               | | (_)           
 _ __ _   _ ___| |_ _ _ __   ___ 
| '__| | | / __| __| | '_ \\ / _ \\
| |  | |_| \\__ \\ |_| | | | |  __/
|_|   \\__,_|___/\\__|_|_| |_|\\___|
                                 
                                 
--]]

print("discord.gg/rustine")`;

let tabCounter = 1;

export const editorState = $state({
  tabs: [
    {
      id: 'tab-1',
      title: 'Script1.lua',
      content: defaultCode,
    },
  ],
  activeId: 'tab-1',
  workspaceFiles: [],
  workspaceWidth: 110,
  consoleOpen: false,
  logs: ['rustine is ready.'],
  renamingTabId: null,
});

export function appendLog(msg) {
  editorState.logs.push(msg);
}

export function clearLogs() {
  editorState.logs = [];
}

export async function loadWorkspaceScripts() {
  const files = await listSavedScripts();
  if (files && files.length > 0) {
    editorState.workspaceFiles = files.map((f, i) => ({
      id: `ws-${i}-${Date.now()}`,
      name: f.name,
      content: f.content,
    }));
  }
}

export function addScriptTab(title = null, content = '') {
  tabCounter++;
  const newId = `tab-${tabCounter}`;
  const tabTitle = title || `Script${tabCounter}.lua`;
  const newTab = {
    id: newId,
    title: tabTitle,
    content: content || defaultCode,
  };
  editorState.tabs.push(newTab);
  editorState.activeId = newId;
  return newTab;
}

export function closeScriptTab(tabId) {
  const filtered = editorState.tabs.filter((t) => t.id !== tabId);
  if (filtered.length === 0) {
    tabCounter = 0;
    editorState.activeId = null;
  } else if (editorState.activeId === tabId) {
    editorState.activeId = filtered[filtered.length - 1].id;
  }
  editorState.tabs = filtered;
}

export function closeOtherTabs(keepId) {
  editorState.tabs = editorState.tabs.filter((t) => t.id === keepId);
  editorState.activeId = keepId;
}

export function closeAllTabs() {
  tabCounter = 0;
  editorState.tabs = [];
  editorState.activeId = null;
}

export function duplicateTab(tabId) {
  const source = editorState.tabs.find((t) => t.id === tabId);
  if (!source) return;
  const base = source.title.replace(/\.lua$/i, '');
  addScriptTab(`${base}_copy.lua`, source.content);
}

export function renameTab(tabId, newTitle) {
  if (!newTitle.trim()) return;
  const finalTitle = newTitle.endsWith('.lua') ? newTitle : `${newTitle}.lua`;
  const target = editorState.tabs.find((t) => t.id === tabId);
  const oldTitle = target ? target.title : null;

  editorState.tabs = editorState.tabs.map((t) =>
    t.id === tabId ? { ...t, title: finalTitle } : t
  );

  if (oldTitle) {
    editorState.workspaceFiles = editorState.workspaceFiles.map((f) =>
      f.name === oldTitle ? { ...f, name: finalTitle } : f
    );
    deleteScriptFromDisk(oldTitle);
    if (target) {
      saveScriptToDisk(finalTitle, target.content);
    }
  }
  editorState.renamingTabId = null;
}

export function addWorkspaceFile(name, content) {
  const id = `ws-${Date.now()}`;
  const finalName = name.endsWith('.lua') ? name : `${name}.lua`;
  const item = { id, name: finalName, content };
  editorState.workspaceFiles.push(item);
  saveScriptToDisk(finalName, content);
  appendLog(`${finalName} saved to scripts/ folder.`);
  return item;
}

export function deleteWorkspaceFile(id) {
  const target = editorState.workspaceFiles.find((f) => f.id === id);
  if (target) {
    editorState.workspaceFiles = editorState.workspaceFiles.filter((f) => f.id !== id);
    deleteScriptFromDisk(target.name);
    appendLog(`${target.name} deleted.`);
  }
}
