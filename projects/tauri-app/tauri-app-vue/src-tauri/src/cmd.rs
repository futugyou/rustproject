use serde::Deserialize;
use tauri::{command, Manager};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestBody {
    id: i32,
    name: String,
}

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
pub async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[command]
#[allow(unused)]
pub async fn open_docs(handle: tauri::AppHandle) {
    let _docs_window = tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}

#[command]
pub fn log_operation(event: String, payload: Option<String>) {
    println!("{} {:?}", event, payload);
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> String {
    println!("{} {:?}", endpoint, body);
    "message response".into()
}
