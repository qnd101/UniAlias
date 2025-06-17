import { Window } from '@tauri-apps/api/window';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { listen } from '@tauri-apps/api/event';
import { error } from '@tauri-apps/plugin-log';

const { invoke } = window.__TAURI__.core;

const window_config = {
  "help": {
    "title": "UniAlias Help",
    "url": "/help/index.html",
    "width": 500,
    "height": 500
  },
  "settings": {
    "title": "UniAlias Settings",
    "url": "settings/index.html",
    "width": 500,
    "height": 500
  },
  "dataset_mng": {
    "title": "Dataset Management",
    "url": "/dataset_mng/index.html",
    "width": 800,
    "height": 600
  }
}

const appWindow = new Window('main');
const compList = document.getElementById('autocompleteList');
const txtInput = document.getElementById('textInput');
const helpBtn = document.getElementById('helpButton');
const reloadBtn = document.getElementById('reloadButton');
const settingsBtn = document.getElementById('settingsButton');
const datasetBtn = document.getElementById('datasetButton');

let childnum = -1;

txtInput.focus()

listen('show_window', (event) => {
  appWindow.show(); // Show the window when the event is received
  const setFocus = async () => {
    if (await appWindow.isMinimized()) {
      await appWindow.unminimize(); // Unminimize the window if it is minimized
    }
    await appWindow.setFocus()
    txtInput.focus();
  };
  setFocus();
});

function createWindow(label) {
  const config = window_config[label];
  const theme = document.documentElement.getAttribute('color-theme') || 'light';
  if (!config) {
    throw new Error(`Window configuration for ${label} not found`);
  }
  config["theme"] = theme; // Add theme to the config
  const newWindow = new WebviewWindow(label, config);

  newWindow.once('tauri://created', () => {
    console.log(`${label} Window successfully created!`);
  });

  newWindow.once('tauri://error', (e) => {
    console.error(`Failed to create window for ${label}:`, e);
  });
  return newWindow;
}


async function find_matches(text) {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let matches = await invoke("find_matches", { input: text, cnt: 5 });
  //console.log(matches)
  compList.innerHTML = ''; // Clear previous results
  matches.forEach(match => {
    const item = document.createElement('span');
    item.innerHTML = `<strong>${match.matchstr.slice(0, match.matchlen)}</strong>${match.matchstr.slice(match.matchlen, match.matchstr.length)} (<span class="character-span">${match.value}</span>)`;
    item.className = 'autocomplete-item';
    item.dataset.alias = match.matchstr;
    item.addEventListener('click', async () => {
      await clear_and_hide(); // Clear input and hide the window
      await select_alias(match.matchstr);
    });
    compList.appendChild(item);
  });
  if (matches.length > 0) {
    compList.children[0].classList.add('selected'); // Select the first item by default
    childnum = 0; // Reset childnum to the first item
  } else {
    childnum = -1; // No matches found
  }
}

async function select_alias(alias) {
  return await invoke("select_alias", { alias });
}

async function clear_and_hide() {
  txtInput.value = ''; // Clear the input field
  compList.innerHTML = ''; // Clear the list
  await appWindow.hide(); // Hide the window
}

window.onload = async () => {
  try {
    await invoke("load_dataset");
  }
  catch (e) {
    error(`Error loading dataset on window load: ${e}`);
  }
}

txtInput.addEventListener("input", (event) => {
  if (event.target.value.length === 0) {
    compList.innerHTML = ''; // Clear results if input is less than 3 characters
    return;
  }
  find_matches(event.target.value);
  childnum = -1; // Reset childnum when input changes
});

reloadBtn.addEventListener('click', async () => {
  try {
    await invoke("load_dataset");
  }
  catch (e) {
    error(`Error reloading dataset: ${e}`);
  }
});

window.addEventListener('keydown', (e) => {
  if (e.key === "Escape") {
    e.preventDefault();
    // Use either close or hide
    // await appWindow.close(); // Quits the window
    clear_and_hide(); // Clear input and hide the window
    return;
  }

  if (e.key === 'Enter' && childnum >= 0 && childnum < compList.children.length) {
    e.preventDefault();
    //close the window and send api
    let alias = compList.children[childnum].dataset.alias;
    clear_and_hide().then(() => {
      select_alias(alias)
    });
  }
  if (e.key === "Tab") {
    txtInput.focus(); // Ensure the input field is focused
  }

  if (e.key === 'Tab' || e.key === 'ArrowDown') {
    e.preventDefault(); // Prevent default tab behavior
    if (compList.children.length === 0)
      return; // No items to select

    // if (childnum >= 0) {
    compList.children[childnum].classList.remove('selected'); // Remove selection from current item
    // }

    childnum = (childnum + 1) % compList.children.length; // Cycle through items
    console.log(`Selected item index: ${childnum}`);
    compList.children[childnum].classList.add('selected');
  }
  else if (e.key === 'ArrowUp') {
    e.preventDefault(); // Prevent default tab behavior
    if (compList.children.length === 0)
      return; // No items to select

    // if (childnum >= 0) {
    compList.children[childnum].classList.remove('selected'); // Remove selection from current item
    // }

    childnum = (childnum - 1 + compList.children.length) % compList.children.length; // Cycle through items
    console.log(`Selected item index: ${childnum}`);
    compList.children[childnum].classList.add('selected');
  }
}
);

// Replace the existing help button event listener
helpBtn.addEventListener('click', () => {
  createWindow('help'); // Show the help window
});

settingsBtn.addEventListener('click', () => {
  createWindow('settings'); // Show the settings window  
});

datasetBtn.addEventListener('click', () => {
  createWindow('dataset_mng'); // Show the dataset management window
});

const theme = localStorage.getItem('color-theme');
document.documentElement.setAttribute('color-theme', theme || 'light');

listen('theme-changed', (event) => {
  document.documentElement.setAttribute('color-theme', event.payload);
})