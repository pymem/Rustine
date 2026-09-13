import { apiGet } from './tauri.js';

export function formatViews(n) {
  if (!n) return '0';
  if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
  if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
  return String(n);
}

export function cleanGameName(name) {
  if (!name) return 'Universal';
  return (
    name
      .replace(/[\u{1F300}-\u{1F9FF}]/gu, '')
      .replace(/d\?\?\?/g, '')
      .trim() || 'Universal'
  );
}

export function extractAuthorName(s) {
  if (s.owner && s.owner.username) return `by ${s.owner.username}`;
  if (s.author && s.author.username) return `by ${s.author.username}`;
  if (s.user && s.user.username) return `by ${s.user.username}`;

  const byMatch = s.title && s.title.match(/(?:by|made by|dev:?)\s+([a-zA-Z0-9_-]+)/i);
  if (byMatch && byMatch[1]) {
    return `by ${byMatch[1]}`;
  }
  return '';
}

export function isValidGameId(gameId) {
  const n = Number(gameId);
  return Number.isFinite(n) && n > 0 && n < Number.MAX_SAFE_INTEGER;
}

function extractGameId(s) {
  const candidates = [
    s.gameId,
    s.game_id,
    s.universeId,
    s.universe_id,
    s.game && s.game.gameId,
    s.game && s.game.game_id,
    s.game && s.game.id,
  ];
  for (const c of candidates) {
    const n = Number(c);
    if (Number.isFinite(n) && n > 0) return Math.floor(n);
  }
  return null;
}

function withGameId(list, gameId) {
  if (!isValidGameId(gameId)) return list;
  const target = Number(gameId);
  const scoped = list.filter((s) => {
    const id = extractGameId(s);
    return id !== null && id === target;
  });
  if (scoped.length > 0) return scoped;
  const hasAnyId = list.some((s) => extractGameId(s) !== null);
  if (hasAnyId) return scoped;
  return list;
}

export async function fetchScriptsFromProvider(providerId, term, targetPage, gameId = null) {
  let list = [];
  let newScripts = [];
  const scopedGame = isValidGameId(gameId) ? Math.floor(Number(gameId)) : null;

  if (providerId === 'scriptblox') {
    if (!term) {
      const data = await apiGet('https://scriptblox.com/api/script/trending', 2200);
      list = data && data.result && data.result.scripts ? data.result.scripts : [];
    } else {
      const data = await apiGet(
        `https://scriptblox.com/api/script/search?q=${encodeURIComponent(term)}&page=${targetPage}`,
        2200
      );
      list = data && data.result && data.result.scripts ? data.result.scripts : [];
    }

    if (list.length === 0) {
      const backupData = await apiGet(
        `https://rscripts.net/api/v2/scripts?${term ? `q=${encodeURIComponent(term)}&` : ''}page=${targetPage}${scopedGame ? `&gameId=${scopedGame}` : ''}`,
        2200
      );
      const backupList = backupData && backupData.scripts ? backupData.scripts : [];
      if (backupList.length > 0) {
        list = backupList.map((s) => ({
          _id: s._id,
          slug: s.slug,
          title: s.title,
          game: { name: s.game && (s.game.title || s.gameName) },
          gameId: s.gameId ?? s.game_id ?? s.game?.gameId ?? null,
          owner: { username: s.user && s.user.username },
          views: s.views,
          verified: s.user && s.user.verified,
          key: s.key,
          paid: s.paid || s.isPaid,
          script: s.rawScript ? `loadstring(game:HttpGet("${s.rawScript}"))()` : '',
        }));
      }
    }

    list = withGameId(list, scopedGame);
    newScripts = list.map((s, i) => {
      const titleLower = (s.title || '').toLowerCase();
      const hasKey = s.key === true || titleLower.includes('[key]') || titleLower.includes('with key');
      const isKeyless = s.key === false || titleLower.includes('keyless') || titleLower.includes('[keyless]');
      const isPaid = s.paid || s.isPaid || titleLower.includes('paid') || titleLower.includes('premium') || titleLower.includes('$');
      const isVerified = s.verified || (s.owner && s.owner.verified) || (s.user && s.user.verified);

      return {
        id: s._id || s.slug || `sb-${targetPage}-${i}`,
        name: s.title || 'Untitled Script',
        game: cleanGameName((s.game && s.game.name) || (s.isUniversal ? 'Universal' : 'Roblox')),
        gameId: extractGameId(s),
        author: extractAuthorName(s),
        verified: Boolean(isVerified),
        key: isKeyless ? false : hasKey ? true : undefined,
        isPaid: Boolean(isPaid),
        views: formatViews(s.views || 0),
        code: s.script || `loadstring(game:HttpGet("https://rawscripts.net/raw/${s.slug}"))()`,
      };
    });
  } else if (providerId === 'rscripts') {
    const base = term
      ? `https://rscripts.net/api/v2/scripts?q=${encodeURIComponent(term)}&page=${targetPage}`
      : `https://rscripts.net/api/v2/scripts?page=${targetPage}`;
    const url = scopedGame ? `${base}&gameId=${scopedGame}` : base;

    const data = await apiGet(url, 2200);
    list = data && data.scripts ? data.scripts : [];
    newScripts = list.map((s, i) => {
      const titleLower = (s.title || '').toLowerCase();
      const hasKey = s.key === true || titleLower.includes('[key]');
      const isKeyless = s.key === false || titleLower.includes('keyless') || titleLower.includes('[keyless]');
      const isPaid = s.paid || s.isPaid || titleLower.includes('paid') || titleLower.includes('$');
      const isVerified = Boolean(s.user && s.user.verified);

      return {
        id: s._id || s.slug || `rs-${targetPage}-${i}`,
        name: s.title || 'Untitled Script',
        game: cleanGameName((s.game && s.game.title) || s.gameName || 'Roblox'),
        gameId: extractGameId(s) ?? scopedGame,
        author: extractAuthorName(s),
        verified: isVerified,
        key: isKeyless ? false : hasKey ? true : undefined,
        isPaid: Boolean(isPaid),
        views: formatViews(s.views || 0),
        code: s.rawScript
          ? `loadstring(game:HttpGet("${s.rawScript}"))()`
          : s.slug
            ? `loadstring(game:HttpGet("https://rscripts.net/raw/${s.slug}"))()`
            : '',
      };
    });
  } else if (providerId === 'haxhell') {
    const url = term
      ? `https://api.haxhell.com/api/v1/search/scripts?q=${encodeURIComponent(term)}&page=${targetPage}&limit=18`
      : `https://api.haxhell.com/api/v1/scripts?page=${targetPage}&limit=18`;

    let data = await apiGet(url, 3500);
    list = data && data.data ? data.data : [];
    if (list.length === 0) {
      const altUrl = term
        ? `https://haxhell.com/api/v1/search/scripts?q=${encodeURIComponent(term)}&page=${targetPage}&limit=18`
        : `https://haxhell.com/api/v1/scripts?page=${targetPage}&limit=18`;
      const altData = await apiGet(altUrl, 3500);
      list = altData && altData.data ? altData.data : [];
    }

    list = withGameId(list, scopedGame);
    newScripts = list.map((s, i) => {
      const titleLower = (s.title || '').toLowerCase();
      const hasKey = Boolean(s.flags && s.flags.keySystem);
      const isKeyless = titleLower.includes('keyless') || (s.flags && s.flags.keySystem === false);
      const isPaid = Boolean(s.flags && s.flags.isPaid) || titleLower.includes('paid') || titleLower.includes('$');
      const isVerified = Boolean(s.flags && s.flags.verified);

      return {
        id: s.id || s.slug || `hh-${targetPage}-${i}`,
        name: s.title || 'Untitled Script',
        game: cleanGameName((s.game && s.game.name) || (s.type === 'universal' ? 'Universal' : 'Roblox')),
        gameId: extractGameId(s),
        author: extractAuthorName(s),
        verified: isVerified,
        key: isKeyless ? false : hasKey ? true : undefined,
        isPaid: isPaid,
        views: formatViews((s.stats && s.stats.views) || 0),
        code: `loadstring(game:HttpGet("${(s.links && s.links.raw) || `https://haxhell.com/api/raw/${s.slug}` }"))()`,
      };
    });
  } else if (providerId === 'robloxscripts') {
    const q = term || 'roblox';
    const scoped = scopedGame ? `&gameId=${scopedGame}` : '';
    let data = await apiGet(`https://rscripts.net/api/v2/scripts?q=${encodeURIComponent(q)}&page=${targetPage}${scoped}`, 2200);
    list = data && data.scripts ? data.scripts : [];
    if (list.length === 0) {
      const fallback = await apiGet(
        `https://scriptblox.com/api/script/search?q=${encodeURIComponent(q)}&page=${targetPage}&max=18`,
        2200
      );
      list = fallback && fallback.result && fallback.result.scripts ? fallback.result.scripts : [];
      list = withGameId(list, scopedGame);
    }

    newScripts = list.map((s, i) => {
      const titleLower = (s.title || s.name || '').toLowerCase();
      const isKeyless = titleLower.includes('keyless') || s.key === false;
      const hasKey = s.key === true || titleLower.includes('key');
      const isPaid = s.paid || titleLower.includes('paid') || titleLower.includes('$');
      const isVerified = Boolean(s.verified || (s.user && s.user.verified));

      return {
        id: s._id || s.slug || `rbs-${targetPage}-${i}`,
        name: s.title || s.name || 'Untitled Script',
        game: cleanGameName((s.game && (s.game.title || s.game.name)) || 'Roblox'),
        gameId: extractGameId(s) ?? scopedGame,
        author: extractAuthorName(s),
        verified: isVerified,
        key: isKeyless ? false : hasKey ? true : undefined,
        isPaid: Boolean(isPaid),
        views: formatViews(s.views || 0),
        code: s.rawScript
          ? `loadstring(game:HttpGet("${s.rawScript}"))()`
          : s.script || `loadstring(game:HttpGet("https://rawscripts.net/raw/${s.slug}"))()`,
      };
    });
  }

  return newScripts;
}
