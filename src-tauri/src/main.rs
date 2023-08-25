// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler, App, UserAttentionType};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn open_window(url: String, handle: tauri::AppHandle) {
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "external",
        tauri::WindowUrl::External(url.parse().unwrap()),
    )
    .build()
    .unwrap();

    std::thread::spawn(move || {
        docs_window
            .show()
            .expect("alternative window could not be launched");
        docs_window
            .request_user_attention(Some(UserAttentionType::Critical))
            .expect("alternate window could not request attention");
    });
}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
    std::thread::spawn(move || {
        docs_window.show().expect("alternative window could not be launched");
        docs_window
            .request_user_attention(Some(UserAttentionType::Critical))
            .expect("alternate window could not request attention");
    });
}


fn main() {
    let app: App = tauri::Builder::default()
        .invoke_handler(generate_handler![greet, open_window, open_docs])
        .build(generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}
