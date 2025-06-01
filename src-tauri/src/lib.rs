use anyhow::Result;
use enigo::{Enigo, Keyboard, Settings};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager};
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
fn find_matches(input: String, cnt: usize, trie: tauri::State<'_, Arc<Trie>>) -> Vec<MatchData> {
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
fn select_alias(alias: String, trie: tauri::State<'_, Arc<Trie>>) -> bool {
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

// //1) parse the unicode config file
// //2) create a tree from the parsed data
fn parse_unicode_config(path: &str) -> Result<Trie> {
    let file = File::open(path).map_err(|e| anyhow::anyhow!("Failed to open file: {}", e))?;
    let reader = io::BufReader::new(file);

    let mut trie = Trie::new();

    for line in reader.lines() {
        let line = line.map_err(|e| anyhow::anyhow!("Failed to read line: {}", e))?;
        // Process each line as needed
        if let Some(idx) = line.find(' ') {
            // Split the line into alias and character
            let alias = line[..idx].to_string();
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
    Ok(trie)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let trie = parse_unicode_config("C:\\Users\\leeyw\\Projects\\Rust\\unicode-manager\\src-tauri\\target\\debug\\unicode_config.txt").unwrap();
    print!("{}", trie);

    tauri::Builder::default()
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
        .manage(Arc::new(trie))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![find_matches, select_alias])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
