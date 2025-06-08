import { readTextFile, writeTextFile, BaseDirectory  } from '@tauri-apps/plugin-fs';
import { Window } from '@tauri-apps/api/window';

const settingsWindow = new Window('settings');
const hotkeyInput = document.getElementById('hotkey');
const saveBtn = document.getElementById('saveBtn');
const cancelBtn = document.getElementById('cancelBtn');

// Default settings
const defaultSettings = {
    hotkey: 'alt+shift+u'
};

// Load settings from file
async function loadSettings() {
    try {
        let contents = await readTextFile('settings.json',  { baseDir: BaseDirectory.AppData })
        const settings = JSON.parse(contents);
        hotkeyInput.value = settings.hotkey;
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