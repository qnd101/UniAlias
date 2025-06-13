import { readTextFile, writeTextFile, BaseDirectory  } from '@tauri-apps/plugin-fs';
import { Window } from '@tauri-apps/api/window';
import { listen, emit } from '@tauri-apps/api/event';

const settingsWindow = new Window('settings');
const hotkeyInput = document.getElementById('hotkey');
const saveBtn = document.getElementById('saveBtn');
const cancelBtn = document.getElementById('cancelBtn');
const themeSelect = document.getElementById('theme');

// Default settings
const defaultSettings = {
    hotkey: 'alt+shift+u'
};

// Load settings from file
async function loadSettings() {
    try {
        let contents = await readTextFile('settings.json',  { baseDir: BaseDirectory.AppData })
        let settings = JSON.parse(contents);

        const theme = localStorage.getItem('color-theme') || 'light';
        document.documentElement.setAttribute('color-theme', theme);

        hotkeyInput.value = settings.hotkey || defaultSettings.hotkey;
    } catch (error) {
        console.error('Error loading settings:', error);
        hotkeyInput.value = defaultSettings.hotkey;
    }
}

// Save settings to file
async function saveSettings() {
    try {
        const settings = {
            hotkey: hotkeyInput.value
        };
        await writeTextFile('settings.json', JSON.stringify(settings, null, 2),  { baseDir: BaseDirectory.AppData });
        localStorage.setItem('color-theme', themeSelect.value);
        settingsWindow.hide();
    } catch (error) {
        console.error('Failed to save settings:', error);
    }
}

// Event listeners
saveBtn.addEventListener('click', saveSettings);
cancelBtn.addEventListener('click', () => {
    settingsWindow.hide();
});

window.addEventListener('keydown', (e) => {
  if (e.key === "Escape") {
    e.preventDefault();
    // Use either close or hide
    // await appWindow.close(); // Quits the window
    settingsWindow.hide(); //hide the window
    return;
  }}
)

// Load settings when window opens
loadSettings();

themeSelect.value = localStorage.getItem('color-theme') || 'light';
themeSelect.addEventListener('change', (e) => {
    const theme = e.target.value;
    document.documentElement.setAttribute('color-theme', theme);
    emit('theme-changed', theme);
});