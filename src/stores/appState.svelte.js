export const appState = $state({
  activeTab: 'editor',
  sidebarCollapsed: false,
  injected: false,
  splashVisible: true,
  contextMenu: {
    visible: false,
    x: 0,
    y: 0,
    tabId: null,
  },
});

export function openContextMenu(e, tabId) {
  e.preventDefault();
  e.stopPropagation();
  appState.contextMenu = {
    visible: true,
    x: e.clientX,
    y: e.clientY,
    tabId,
  };
}

export function closeContextMenu() {
  appState.contextMenu.visible = false;
}
