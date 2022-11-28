use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl,
};

pub fn main() {
    tauri_app_vue::AppBuilder::new()
        .setup(|app| {
            create_item(app)?;
            Ok(())
        })
        .run()

    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let close = CustomMenuItem::new("close".to_string(), "Close");
    // let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    // let menu = Menu::new()
    //     .add_native_item(MenuItem::Copy)
    //     .add_item(CustomMenuItem::new("hide", "Hide"))
    //     .add_submenu(submenu);

    // tauri::Builder::default()
    //     // .menu(Menu::with_items([MenuEntry::Submenu(Submenu::new(
    //     //     "File",
    //     //     Menu::with_items([
    //     //         MenuItem::CloseWindow.into(),
    //     //         #[cfg(targer_os = "macos")]
    //     //         CustomMenuItem::new("hello", "Hello").into(),
    //     //     ]),
    //     // ))]))
    //     .menu(menu)
    //     .on_menu_event(|event| match event.menu_item_id() {
    //         "quit" => {
    //             std::process::exit(0);
    //         }
    //         "close" => event.window().close().unwrap(),
    //         _ => {}
    //     })
    //     .on_window_event(|event| match event.event() {
    //         tauri::WindowEvent::CloseRequested { api, .. } => {
    //             api.prevent_close();
    //             let window = event.window().clone();

    //             tauri::api::dialog::confirm(
    //                 Some(&event.window()),
    //                 "close app",
    //                 "are you sure?",
    //                 move |answer| {
    //                     if answer {
    //                         let _result = window.close();
    //                     }
    //                 },
    //             )
    //         }
    //         _ => {}
    //     })
    //     // .setup(|app| {
    //     //     let docs_window = WindowBuilder::new(
    //     //         app,
    //     //         "main-window".to_string(),
    //     //         tauri::WindowUrl::App("index.html".into()),
    //     //     )
    //     //     //.menu(menu)
    //     //     .build();
    //     //     let local_window = tauri::WindowBuilder::new(
    //     //         app,
    //     //         "external",
    //     //         tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    //     //     )
    //     //     .build();
    //     //     Ok(())
    //     // })
    //     .invoke_handler(tauri::generate_handler![
    //         cmd::greet,
    //         cmd::close_splashscreen
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

fn create_item(app: &tauri::App) -> tauri::Result<()> {
    let mut tray_menu1 = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle", "Toggle"))
        .add_item(CustomMenuItem::new("new", "New window"))
        .add_item(CustomMenuItem::new("icon_1", "Tray Icon 1"))
        .add_item(CustomMenuItem::new("icon_2", "Tray Icon 2"));

    #[cfg(target_os = "macos")]
    {
        tray_menu1 = tray_menu1.add_item(CustomMenuItem::new("set_title", "Set Title"));
    }

    tray_menu1 = tray_menu1
        .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
        .add_item(CustomMenuItem::new("exit_app", "Quit"))
        .add_item(CustomMenuItem::new("destroy", "Destroy"));

    let tray_menu2 = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle", "Toggle"))
        .add_item(CustomMenuItem::new("new", "New window"))
        .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
        .add_item(CustomMenuItem::new("exit_app", "Quit"))
        .add_item(CustomMenuItem::new("destroy", "Destroy"));
    let is_menu1 = AtomicBool::new(true);

    let handle = app.handle();
    let tray_id = "my-tray".to_string();
    SystemTray::new()
        .with_id(&tray_id)
        .with_menu(tray_menu1.clone())
        .on_event(move |event| {
            let tray_handle = handle.tray_handle_by_id(&tray_id).unwrap();
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = handle.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let item_handle = tray_handle.get_item(&id);
                    match id.as_str() {
                        "exit_app" => {
                            // exit the app
                            handle.exit(0);
                        }
                        "destroy" => {
                            tray_handle.destroy().unwrap();
                        }
                        "toggle" => {
                            let window = handle.get_window("main").unwrap();
                            let new_title = if window.is_visible().unwrap() {
                                window.hide().unwrap();
                                "Show"
                            } else {
                                window.show().unwrap();
                                "Hide"
                            };
                            item_handle.set_title(new_title).unwrap();
                        }
                        "new" => {
                            WindowBuilder::new(&handle, "new", WindowUrl::App("index.html".into()))
                                .title("Tauri")
                                .build()
                                .unwrap();
                        }
                        "set_title" => {
                            #[cfg(target_os = "macos")]
                            tray_handle.set_title("Tauri").unwrap();
                        }
                        //   "icon_1" => {
                        //     #[cfg(target_os = "macos")]
                        //     tray_handle.set_icon_as_template(true).unwrap();

                        //     tray_handle
                        //       .set_icon(tauri::Icon::Raw(
                        //         include_bytes!("../../../.icons/tray_icon_with_transparency.png").to_vec(),
                        //       ))
                        //       .unwrap();
                        //   }
                        //   "icon_2" => {
                        //     #[cfg(target_os = "macos")]
                        //     tray_handle.set_icon_as_template(true).unwrap();

                        //     tray_handle
                        //       .set_icon(tauri::Icon::Raw(
                        //         include_bytes!("../../../.icons/icon.ico").to_vec(),
                        //       ))
                        //       .unwrap();
                        //   }
                        "switch_menu" => {
                            let flag = is_menu1.load(Ordering::Relaxed);
                            tray_handle
                                .set_menu(if flag {
                                    tray_menu2.clone()
                                } else {
                                    tray_menu1.clone()
                                })
                                .unwrap();
                            is_menu1.store(!flag, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .build(app)
        .map(|_| ())
}
