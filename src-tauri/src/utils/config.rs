pub struct Config {
    window: Window,
    shortcuts: Vec<Shortcut>,
}

struct Shortcut {}
struct Window {
    theme: String,
    transparent: bool,
}

fn load_base_config() -> Config {
    todo!()
    // loading file
    // parsing yaml
    // checking fields
    // return config
}
