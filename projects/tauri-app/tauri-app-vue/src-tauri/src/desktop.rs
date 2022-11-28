pub fn main() {
    tauri_app_vue::AppBuilder::new()
    .setup(|app|{
        Ok(())
    }).run()

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
