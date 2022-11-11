#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

#[tauri::command]
fn backend_paths() {
    let paths = fs::read_dir("/").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

#[tauri::command]
fn backend_add(num: i32) -> i32 {
    // Note: these commands block the main thread and hang the UI until they return.
    // If you need to run a long-running task, use async command instead.
    println!("Backend was called with an argument: {}", num);
    num + 2
}

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![backend_paths, backend_add])
        .menu(
            tauri::Menu::os_default("Rexplorer").add_submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new("GitHub", "GitHub").into()]),
            )),
        )
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "GitHub" => {
                    let url = "https://github.com/Shinigami92/rexplorer".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
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
        .run(ctx)
        .expect("error while running tauri application");
}
