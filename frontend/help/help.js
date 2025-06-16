import {listen} from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";

const helpWindow = new Window('help');

const theme = localStorage.getItem('color-theme');
document.documentElement.setAttribute('color-theme', theme || 'light');

listen('theme-changed', (event) => {
    document.documentElement.setAttribute('color-theme', event.payload);
})

window.addEventListener('keydown', (e) => {
  if (e.key === "Escape") {
    e.preventDefault();
    helpWindow.close()
    return;
  }}
)
