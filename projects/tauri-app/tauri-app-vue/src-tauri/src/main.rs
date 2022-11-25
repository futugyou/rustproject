#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        // .menu(Menu::with_items([MenuEntry::Submenu(Submenu::new(
        //     "File",
        //     Menu::with_items([
        //         MenuItem::CloseWindow.into(),
        //         #[cfg(targer_os = "macos")]
        //         CustomMenuItem::new("hello", "Hello").into(),
        //     ]),
        // ))]))
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => event.window().close().unwrap(),
            _ => {}
        })
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
        // .setup(|app| {
        //     WindowBuilder::new(
        //         app,
        //         "main-window".to_string(),
        //         tauri::WindowUrl::App("index.html".into()),
        //     )
        //     .menu(menu)
        //     .build()?;
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
