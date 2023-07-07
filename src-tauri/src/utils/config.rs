use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use tauri::api::path::home_dir;
use tauri::{TitleBarStyle, WindowBuilder};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationFile {
    window: Window,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Window {
    theme: String,
    transparent: bool,
    hidden_title: bool,
    decorations: bool,
    title_bar_style: TitleBarStyle,
}

pub fn load_base_config() -> ConfigurationFile {
    let config_dir = format!(
        "{}/.config/clear_docs/config.yaml",
        home_dir().unwrap().to_str().unwrap()
    );
    println!("The current directory is {:?}", config_dir);
    let file = File::open(config_dir).unwrap();
    let config = serde_yaml::from_reader::<&File, ConfigurationFile>(&file).unwrap();
    println!("{:?}", config);
    return config;
    // loading file
    // parsing yaml
    // checking fields
    // return config
}

pub fn configured_window<'a>(
    config: &ConfigurationFile,
) -> impl Fn(WindowBuilder<'a>) -> WindowBuilder<'a> {
    let window_config = config.window.clone();
    move |window| {
        window
            .transparent(window_config.transparent)
            .hidden_title(window_config.hidden_title)
            .title_bar_style(window_config.title_bar_style.clone())
            .decorations(window_config.decorations)
    }
}
