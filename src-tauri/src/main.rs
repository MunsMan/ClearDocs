use std::path::Path;
use std::{env, sync::Mutex};
use tauri::{AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window};
use utils::config::{configured_window, load_base_config, ConfigurationFile};

mod utils;

struct AppState {
    app_handle: AppHandle,
    configuration: ConfigurationFile,
}

impl AppState {}

fn update_url(window: &Window, url: &str) {
    window
        .eval(&format!(
            "window.location.replace('http://localhost:{}')",
            url
        ))
        .unwrap();
}

#[tauri::command]
fn pdf_exists(filepath: String, app_handle: AppHandle) -> bool {
    if Path::new(&filepath).exists() {
        app_handle.fs_scope().allow_file(filepath).unwrap();
        return true;
    }
    return false;
}

fn create_window(
    window_name: String,
    app_handle: &AppHandle,
    config_file: &ConfigurationFile,
) -> tauri::Window {
    return configured_window(config_file)(tauri::WindowBuilder::new(
        app_handle,
        window_name,
        tauri::WindowUrl::App("index.html".into()),
    ))
    .build()
    .unwrap();
}

fn generade_window_id(app_handle: &AppHandle) -> u32 {
    let mut id = 0;
    let windows = app_handle.windows();
    loop {
        if !(windows.contains_key(&id.to_string())) {
            return id;
        }
        id += 1;
    }
}

fn main() {
    for arg in env::args_os().into_iter() {
        println!("{arg:#?}");
    }
    tauri::Builder::default()
        .menu(
            Menu::new()
                .add_native_item(MenuItem::Copy)
                .add_item(CustomMenuItem::new("hide", "Hide"))
                .add_submenu(Submenu::new(
                    "File",
                    Menu::new()
                        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
                        .add_item(CustomMenuItem::new("close".to_string(), "Close"))
                        .add_item(CustomMenuItem::new("open".to_string(), "Open"))
                        .add_item(CustomMenuItem::new("new".to_string(), "New")),
                )),
        )
        .invoke_handler(tauri::generate_handler![pdf_exists])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            let configuration = load_base_config();
            let store = Mutex::new(AppState {
                app_handle: app.handle(),
                configuration,
            });

            app.manage(store);
            // ToDo: Load file from config and update app config.
            Ok(())
        })
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            "open" => {
                println!("{}", event.window().url());
                event
                    .window()
                    .get_focused_window()
                    .or(Some(create_window(
                        generade_window_id(&event.window().app_handle()).to_string(),
                        &event.window().app_handle(),
                        &event
                            .window()
                            .app_handle()
                            .state::<Mutex<AppState>>()
                            .lock()
                            .unwrap()
                            .configuration
                            .clone(),
                    )))
                    .unwrap()
                    .emit("open", {})
                    .unwrap();
                update_url(&event.window(), "/");
                println!("{}", event.window().url());
            }
            "new" => {
                println!("Open new Window");
                let app_handle = event.window().app_handle();
                create_window(
                    generade_window_id(&app_handle).to_string(),
                    &app_handle,
                    &app_handle
                        .state::<Mutex<AppState>>()
                        .lock()
                        .unwrap()
                        .configuration
                        .clone(),
                );
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
