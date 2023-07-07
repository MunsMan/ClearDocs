use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let close: CustomMenuItem = CustomMenuItem::new("close".to_string(), "Close");
    let new = CustomMenuItem::new("new".to_string(), "New");
    let submenu: Submenu = Submenu::new(
        "File",
        Menu::new().add_item(quit).add_item(close).add_item(new),
    );
    return Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
}

pub fn listen_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            event.window().close().unwrap();
        }
        "new" => {
            println!("Open new Window");
        }
        _ => {}
    }
}
