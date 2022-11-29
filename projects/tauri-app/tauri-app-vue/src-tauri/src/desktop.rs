use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl,
};

pub fn main() {
    tauri_app_vue::AppBuilder::new()
        .setup(|app| {
            create_tray(app)?;
            Ok(())
        })
        .run()
}

fn create_tray(app: &tauri::App) -> tauri::Result<()> {
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
                        "icon_1" => {
                            #[cfg(target_os = "macos")]
                            tray_handle.set_icon_as_template(true).unwrap();

                            tray_handle
                                .set_icon(tauri::Icon::Raw(
                                    include_bytes!("../icons/tray_icon_with_transparency.ico")
                                        .to_vec(),
                                ))
                                .unwrap();
                        }
                        "icon_2" => {
                            #[cfg(target_os = "macos")]
                            tray_handle.set_icon_as_template(true).unwrap();

                            tray_handle
                                .set_icon(tauri::Icon::Raw(
                                    include_bytes!("../icons/icon.ico").to_vec(),
                                ))
                                .unwrap();
                        }
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
