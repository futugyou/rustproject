#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Menu, MenuEntry, Submenu, MenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .menu(Menu::with_items([MenuEntry::Submenu(Submenu::new(
            "File",
            Menu::with_items([
                MenuItem::CloseWindow.into(),
                #[cfg(targer_os = "macos")]
                CustomMenuItem::new("hello", "Hello").into(),
            ]),
        ))]))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let window = event.window().clone();

                tauri::api::dialog::confirm(
                    Some(&event.window()),
                    "close app",
                    "are you sure?",
                    move |answer| {
                        if answer {
                            let _result = window.close();
                        }
                    },
                )
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
