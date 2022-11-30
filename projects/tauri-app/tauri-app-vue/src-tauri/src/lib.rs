#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use tauri::{
    window::WindowBuilder, App, AppHandle, CustomMenuItem, Menu, MenuEntry, MenuItem, RunEvent,
    Submenu, WindowUrl,
};

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;
pub type OnEvent = Box<dyn FnMut(&AppHandle, RunEvent)>;
#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
    on_event: Option<OnEvent>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_event<F>(mut self, on_event: F) -> Self
    where
        F: Fn(&AppHandle, RunEvent) + 'static,
    {
        self.on_event.replace(Box::new(on_event));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        let mut on_event = self.on_event;

        let quit = CustomMenuItem::new("quit".to_string(), "Quit");
        let close = CustomMenuItem::new("close".to_string(), "Close");
        let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
        let submenu2 =  Submenu::new(
            "File",
            Menu::with_items([
                MenuItem::CloseWindow.into(),
                #[cfg(targer_os = "macos")]
                CustomMenuItem::new("hello", "Hello").into(),
            ]),
        );
        let menu = Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_item(CustomMenuItem::new("hide", "Hide"))
            .add_submenu(submenu)
            .add_submenu(submenu2)
            ;

        #[allow(unused_mut)]
        let mut builder = tauri::Builder::default().setup(move |app| {
            if let Some(setup) = setup {
                (setup)(app)?;
            }

            let mut window_builder = WindowBuilder::new(app, "main", WindowUrl::default())
                .user_agent("tauri api")
                .title("tauri api validation")
                .inner_size(1000., 1000.)
                .min_inner_size(600., 400.);

            #[cfg(target_os = "windows")]
            {
                window_builder = window_builder.transparent(true);
                window_builder = window_builder.decorations(false);
            }

            let window = window_builder.build().unwrap();

            #[cfg(target_os = "windows")]
            {
                let _ = window_shadows::set_shadow(&window, true);
                let _ = window_vibrancy::apply_blur(&window, Some((0, 0, 0, 0)));
            }

            // #[cfg(debug_assertions)]
            // window.open_devtools();

            // skip this for now

            Ok(())
        });

        #[allow(unused_mut)]
        let mut app = builder
            .menu(Menu::with_items([MenuEntry::Submenu(Submenu::new(
                "File",
                Menu::with_items([
                    MenuItem::CloseWindow.into(),
                    #[cfg(targer_os = "macos")]
                    CustomMenuItem::new("hello", "Hello").into(),
                ]),
            ))]))
            .menu(menu)
            .invoke_handler(tauri::generate_handler![
                cmd::greet,
                cmd::close_splashscreen,
            ])
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
            .build(tauri::generate_context!())
            .expect("error while building tauri application");

        app.run(move |app_handler, e| {
            if let RunEvent::ExitRequested { api, .. } = &e {
                api.prevent_exit();
            }
            if let Some(on_event) = &mut on_event {
                (on_event)(app_handler, e);
            }
        })
    }

    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }
}