use tauri::{path::BaseDirectory, Manager};

pub mod cam;
pub mod server;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, server])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn server(handle: tauri::AppHandle) {
    let resource_path = handle
        .path()
        .resolve(
            "resources/haarcascade_frontalface_default.xml",
            BaseDirectory::Resource,
        )
        .unwrap();
    server::run(&resource_path);
}
