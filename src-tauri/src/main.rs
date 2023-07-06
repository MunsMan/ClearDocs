use std::env;
use std::path::Path;
use tauri::{AppHandle, Manager};

mod utils;

struct AppState {}

#[tauri::command]
fn pdf_exists(filepath: String, app_handle: AppHandle) -> bool {
    if Path::new(&filepath).exists() {
        app_handle.fs_scope().allow_file(filepath).unwrap();
        return true;
    }
    return false;
}

fn main() {
    for arg in env::args_os().into_iter() {
        println!("{arg:#?}");
    }
    tauri::Builder::default()
        .manage(AppState {})
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            // ToDo: Load file from config and update app config.
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![pdf_exists])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
