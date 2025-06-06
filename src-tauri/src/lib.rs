use anyhow::Result;
use enigo::{Enigo, Keyboard, Settings};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager};
use trie::{Trie, TrieNodeContent};
use std::sync::RwLock;

pub mod trie;

#[derive(Serialize, Deserialize, Debug)]
struct MatchData {
    matchstr: String,
    matchlen: usize,
    value: char,
}

/// Finds (cnt)-top matches for a given input string
/// if the input is empty or not ASCII it returns an empty list
#[tauri::command]
fn find_matches(input: String, cnt: usize, appstate: tauri::State<'_, AppState>) -> Vec<MatchData> {
    let trie = appstate.trie.read().unwrap();
    let mut result = Vec::with_capacity(cnt);
    let mut counter = cnt;
    if cnt == 0 || input.len() == 0 || !input.is_ascii() {
        return result;
    }

    let (midx, mlen) = trie.find_max_match(input.as_bytes());
    if midx == 0 {
        //root
        return result;
    }
    for (idx, _) in trie.iter(&midx) {
        let node = &(*trie).nodes[idx];
        if let TrieNodeContent::Leaf { data } = node.content {
            result.push(MatchData {
                matchstr: node.value_str().to_owned(),
                matchlen: mlen,
                value: data,
            });
            counter -= 1;
            if counter == 0 {
                break;
            }
        }
    }
    //println!("{:?}", result);
    result
}

//gets the alias, searches it within the trie, and input the match through keyboard
//window hiding is handled by frontend
//not recives the value directly (more safe)
#[tauri::command]
fn select_alias(alias: String, appstate: tauri::State<'_, AppState>) -> bool {
    let trie = appstate.trie.read().unwrap();
    match trie.find_value(&alias) {
        Ok(ch) => {
            if let Ok(mut en) = Enigo::new(&Settings::default()) {
                if let Ok(_) = en.text(&ch.to_string()) {
                    true
                } else {
                    println!("Failed to input character: {}", ch);
                    false
                }
            } else {
                false
            }
        }
        Err(e) => {
            println!("Error finding alias: {}", e);
            false
        }
    }
}

// Loads all datasets under the "dataset" directory in the app data directory
// It expects each dataset to be in CSV format
// Returns an error if the dataset cannot be loaded or parsed (error type is String)
#[tauri::command]
fn load_dataset(app_handle: tauri::AppHandle, appstate: tauri::State<'_, AppState>) -> Result<(), String> {
    // Create a new Trie instance
    let mut newtrie = Trie::new();
    let config_path = app_handle.path().app_data_dir().map_err(|_| "Failed to find appdata directory")?.join("dataset");
    // Parse all csv files under the path
    println!("Loading dataset from: {:?}...", config_path);
    for entry in std::fs::read_dir(config_path).map_err(|_| "Failed to open appdata directory")? {
        let entry = entry.map_err(|_| "Failed to read appdata entry")?;
        let path = entry.path();
        let path = path.as_path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            // Parse the unicode config file and append data to the trie
            parse_unicode_dataset(path, &mut newtrie).map_err(|e| {
                format!("Failed to parse dataset file {:?}: {}", path, e)
            })?;
            println!("Loaded dataset from: {:?}", path.file_name().unwrap());
        } else {
            println!("Skipping non-csv file: {:?}", path);
        }
    }
    println!("Dataset loaded successfully.");

    // Print the trie if debug
    #[cfg(debug_assertions)]
    {
        println!("Current Trie: {}", &newtrie);
    }

    // Swap the new trie into the ArcSwap
    let mut triemut = appstate.trie.write().unwrap();
    *triemut = newtrie;
    Ok(())
}

// //1) parse the unicode config file (a csv file of two colums. It contains comments starting with '#')
// //2) appends all the parsed data into the trie
fn parse_unicode_dataset(path : &Path,  trie: &mut Trie) -> Result<()> {
    let file = File::open(path).map_err(|e| anyhow::anyhow!("Failed to open file: {}", e))?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| anyhow::anyhow!("Failed to read line: {}", e))?;
        if line.len() > 0 && line.starts_with('#') {
            // Skip comment lines
            continue;
        }
        // Process each line as needed
        if let Some(idx) = line.find(',') {
            // Split the line into alias and character
            let alias = line[..idx].trim().to_string();
            // Check alias validity (non-empty, ASCII)
            if alias.is_empty() {
                return Err(anyhow::anyhow!(
                    "Invalid line format. Alias string is empty: {}",
                    line
                ));
            }
            if !alias.is_ascii() {
                return Err(anyhow::anyhow!(
                    "Invalid line format. Alias string is not ASCII: {}",
                    line
                ));
            }

            let ch = line[idx + 1..].trim().chars().next().ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid line format. No character found after space in line: {}",
                    line
                )
            })?;
            if let Result::Err(e) = trie.append_leaf(alias, ch) {
                println!("Warning: {}", e);
            }
        } else {
            return Err(anyhow::anyhow!(
                "Invalid line format. Space separation not found: {}",
                line
            ));
        }
    }
    Ok(())
}

struct AppState {
    trie: RwLock<Trie>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState{trie: RwLock::new(Trie::new())})
        .setup(|app| {
            let exit_i = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&exit_i])?;
            let _ = TrayIconBuilder::new()
                .menu(&menu)
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            use tauri_plugin_global_shortcut::{Builder, ShortcutState};
            app.handle().plugin(
                Builder::new()
                    .with_shortcuts(["alt+shift+u"])?
                    .with_handler(move |_app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        //println!("Shortcut Pressed: {:?}", shortcut);
                        if let Err(e) = _app.get_webview_window("main").unwrap().emit("show_window", ()) {
                            println!("Error emitting event: {:?}", e);
                        }
                    }
                })
                .build(),
            )?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Prevent the app from quitting
                api.prevent_close();
                // Hide the window instead
                let _ = window.hide();
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "exit" => {
                println!("Exit...");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![find_matches, select_alias, load_dataset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
