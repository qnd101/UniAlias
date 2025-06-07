import { Window } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';

const { invoke } = window.__TAURI__.core;

const compList = document.getElementById('autocompleteList');
const txtInput = document.getElementById('textInput');
const helpBtn = document.getElementById('helpButton');
const reloadBtn = document.getElementById('reloadButton');
const appWindow = new Window('main');
const helpWindow = new Window('help');

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

async function select_alias(alias){
  return await invoke("select_alias", { alias});
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
  catch (error) {
    console.error("Error loading dataset on window load:", error);
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
  catch (error) {
    console.error("Error reloading dataset:", error);
  }
});

//TODO arrow key UP Down
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
    let alias = compList.children[childnum].dataset.alias ;
    clear_and_hide().then(() => {    
      select_alias(alias)
    });
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
document.getElementById('helpButton').addEventListener('click', () => {
  helpWindow.show(); // Show the help window
});
