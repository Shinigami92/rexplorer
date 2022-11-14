#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

#[derive(serde::Serialize)]
struct PathResponse {
    path: String,
    name: String,
    size: u64,
    is_dir: bool,
}

#[tauri::command]
fn get_paths(path: Option<String>) -> Result<Vec<PathResponse>, String> {
    let path = path.unwrap_or_else(|| std::env::var("HOME").unwrap());
    let paths = fs::read_dir(path).map_err(|e| e.to_string())?;

    let mut paths_vec = Vec::new();
    for path in paths {
        let p = path.unwrap().path();
        if p.is_symlink() {
            continue;
        }
        paths_vec.push(PathResponse {
            path: p.to_str().unwrap().to_string(),
            name: p.file_name().unwrap().to_str().unwrap().to_string(),
            size: p.metadata().unwrap().len(),
            is_dir: p.is_dir(),
        });
    }
    Ok(paths_vec)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_paths])
        .menu(
            tauri::Menu::os_default("Rexplorer").add_submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new("GitHub", "GitHub").into()]),
            )),
        )
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            if event_name == "GitHub" {
                let url = "https://github.com/Shinigami92/rexplorer".to_string();
                shell::open(&event.window().shell_scope(), url, None).unwrap();
            }
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
