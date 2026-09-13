<script>
  import SplashScreen from './components/layout/SplashScreen.svelte';
  import Topbar from './components/layout/Topbar.svelte';
  import Sidebar from './components/layout/Sidebar.svelte';
  import TabContextMenu from './components/layout/TabContextMenu.svelte';
  import EditorView from './components/editor/EditorView.svelte';
  import ScripthubView from './components/scripthub/ScripthubView.svelte';
  import SettingsView from './views/SettingsView.svelte';
  import { appState } from './stores/appState.svelte.js';
  import { settingsState } from './stores/settingsState.svelte.js';
</script>

<div id="app" data-theme={settingsState.theme}>
  <SplashScreen />
  <Topbar />

  <div class="window-body">
    <Sidebar />

    <div class="main-view" id="main-content-view">
      <div class="tab-content" class:active={appState.activeTab === 'editor'}>
        <EditorView />
      </div>

      <div class="tab-content" class:active={appState.activeTab === 'scripthub'}>
        <ScripthubView />
      </div>

      <div class="tab-content" class:active={appState.activeTab === 'settings'}>
        <SettingsView />
      </div>
    </div>
  </div>

  <TabContextMenu />
</div>
