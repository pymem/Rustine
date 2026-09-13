import { fetchScriptsFromProvider } from '../lib/api.js';
import { getGameId } from '../lib/tauri.js';

export const providers = [
  {
    id: 'scriptblox',
    name: 'ScriptBlox',
    icon: 'https://scriptblox.com/favicon.ico',
  },
  {
    id: 'rscripts',
    name: 'RScripts',
    icon: 'https://rscripts.net/favicon.ico',
  },
  { id: 'haxhell', name: 'HaxHell', icon: 'https://haxhell.com/favicon.ico' },
  {
    id: 'robloxscripts',
    name: 'RobloxScripts',
    icon: 'https://robloxscripts.com/favicon.ico',
  },
];

export const filterOptions = [
  { id: 'all', label: 'All Scripts' },
  { id: 'verified', label: 'Verified' },
  { id: 'keyless', label: 'Keyless' },
  { id: 'key', label: 'Key System' },
  { id: 'paid', label: 'Paid / Premium' },
  { id: 'universal', label: 'Universal' },
];

export const scripthubState = $state({
  provider: providers[0],
  filter: 'all',
  query: '',
  loading: false,
  loadingMore: false,
  page: 1,
  hasMore: true,
  scripts: [],
  gameId: null,
  gameIdStatus: 'unknown',
});

export async function refreshGameId() {
  scripthubState.gameIdStatus = 'loading';
  const id = await getGameId();
  if (id) {
    scripthubState.gameId = id;
    scripthubState.gameIdStatus = 'live';
  } else {
    scripthubState.gameId = null;
    scripthubState.gameIdStatus = 'unavailable';
  }
  return scripthubState.gameId;
}

export async function fetchScripts(append = false) {
  if (append) {
    scripthubState.loadingMore = true;
  } else {
    scripthubState.loading = true;
    scripthubState.page = 1;
    scripthubState.hasMore = true;
  }

  const provider = scripthubState.provider.id;
  const query = scripthubState.query.trim();
  const page = append ? scripthubState.page : 1;

  try {
    const res = await fetchScriptsFromProvider(provider, query, page, scripthubState.gameId);
    if (res.length === 0) {
      scripthubState.hasMore = false;
    }
    if (append) {
      const existingIds = new Set(scripthubState.scripts.map((s) => s.id));
      const unique = res.filter((s) => !existingIds.has(s.id));
      scripthubState.scripts = [...scripthubState.scripts, ...unique];
    } else {
      scripthubState.scripts = res;
    }
  } catch (e) {
    if (!append) {
      scripthubState.scripts = [];
    }
  } finally {
    scripthubState.loading = false;
    scripthubState.loadingMore = false;
  }
}
