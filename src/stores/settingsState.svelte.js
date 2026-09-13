export const retroThemes = [
  {
    id: 'rustine-dark',
    name: 'Rustine Dark (Default)',
    desc: 'Dark charcoal with dusty rose coral accent',
    preview: {
      bg: '#212121',
      window: '#262626',
      accent: '#ffcec6',
      title: '#1c1c1c',
      text: '#dedede',
    },
  },
  {
    id: 'win95-classic',
    name: 'Win95',
    desc: 'Authentic battleship gray and navy title',
    preview: {
      bg: '#c0c0c0',
      window: '#ffffff',
      accent: '#000080',
      title: '#000080',
      text: '#000000',
    },
  },
];

const defaultSettings = {
  theme: 'rustine-dark',
  fontSize: 16,
  tabSize: 4,
  lineNumbers: true,
  minimap: false,
  wordWrap: false,
  renderWhitespace: 'none',
  cursorStyle: 'line',
  cursorBlinking: 'solid',
  smoothScrolling: true,
};

function loadSettings() {
  if (typeof localStorage !== 'undefined') {
    try {
      const saved = localStorage.getItem('rustine_editor_settings');
      if (saved) {
        return { ...defaultSettings, ...JSON.parse(saved) };
      }
    } catch (e) {}
  }
  return defaultSettings;
}

export const settingsState = $state(loadSettings());

export function updateSetting(key, value) {
  settingsState[key] = value;
  if (typeof localStorage !== 'undefined') {
    try {
      localStorage.setItem('rustine_editor_settings', JSON.stringify(settingsState));
    } catch (e) {}
  }
}
