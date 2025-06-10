use enigo::{Enigo, Keyboard, Settings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::RwLock;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, Emitter, Manager};
use trie::{Trie, TrieNodeContent};

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
    //log::info!("{:?}", result);
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
                    log::info!("Failed to input character: {}", ch);
                    false
                }
            } else {
                false
            }
        }
        Err(e) => {
            log::info!("Error finding alias: {}", e);
            false
        }
    }
}

// Loads all datasets under the "dataset" directory in the app data directory
// It expects each dataset to be in CSV format
// Returns an error if the dataset cannot be loaded or parsed (error type is String)
#[tauri::command]
fn load_dataset(
    app_handle: tauri::AppHandle,
    appstate: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Create a new Trie instance
    let mut newtrie = Trie::new();
    let config_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to find appdata directory")?
        .join("dataset");
    // Parse all csv files under the path
    log::info!("Loading dataset from: {:?}...", config_path);
    for entry in std::fs::read_dir(config_path).map_err(|_| "Failed to open appdata directory")? {
        let entry = entry.map_err(|_| "Failed to read appdata entry")?;
        let path = entry.path();
        let path = path.as_path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            // Parse the unicode config file and append data to the trie
            parse_unicode_dataset(path, &mut newtrie)
                .map_err(|e| format!("Failed to parse dataset file {:?}: {}", path, e))?;
            log::info!("Loaded dataset from: {:?}", path.file_name().unwrap());
        } else {
            log::info!("Skipping non-csv file: {:?}", path);
        }
    }
    log::info!("Dataset loaded successfully.");

    // Print the trie if debug
    // #[cfg(debug_assertions)]
    // {
    //     log::info!("Current Trie: {}", &newtrie);
    // }

    // Swap the new trie into the ArcSwap
    let mut triemut = appstate.trie.write().unwrap();
    *triemut = newtrie;
    Ok(())
}

// //1) parse the unicode config file (a csv file of two colums. It contains comments starting with '#')
// //2) appends all the parsed data into the trie
fn parse_unicode_dataset(path: &Path, trie: &mut Trie) -> anyhow::Result<()> {
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
                log::info!("Warning: {}", e);
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

fn setup_hotkey(app: &mut App, hotkey: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_global_shortcut::{Builder, ShortcutState};
    app.handle().plugin(
        Builder::new()
            .with_shortcuts([hotkey])?
            .with_handler(move |_app, _, event| {
                if event.state() == ShortcutState::Pressed {
                    //log::info!("Shortcut Pressed: {:?}", shortcut);
                    if let Err(e) = _app
                        .get_webview_window("main")
                        .unwrap()
                        .emit("show_window", ())
                    {
                        log::info!("Error emitting event: {:?}", e);
                    }
                }
            })
            .build(),
    )?;
    Result::Ok(())
}

fn load_settings(app: &App) -> anyhow::Result<HashMap<String, String>> {
    let settings_path = app.path().app_data_dir()?.join("settings.json");
    let file = File::open(settings_path)?;
    let reader = io::BufReader::new(file);
    let dict: HashMap<String, String> = serde_json::from_reader(reader)?;
    return Ok(dict);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            trie: RwLock::new(Trie::new()),
        })
        .setup(|app| {
            // Open settings
            let settings_dict = match load_settings(app) {
                Ok(dict) => dict,
                Err(e) => {
                    log::error!("Error loading settings: {}", e);
                    HashMap::new()
                }
            };
            // Register the tray icon
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

            let hotkey: &str = settings_dict
                .get("hotkey")
                .map(|s| s.as_str())
                .unwrap_or_else(|| "alt+shift+u");
            // Setup the hotkey
            if let Err(e) = setup_hotkey(app, hotkey) {
                log::error!("Error setting up hotkey: {}", e);
            }
            Result::Ok(())
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
                log::info!("Exit...");
                app.exit(0);
            }
            _ => {
                log::info!("menu item {:?} not handled", event.id);
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            find_matches,
            select_alias,
            load_dataset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
