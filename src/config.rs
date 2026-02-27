pub struct Config {
    pub screen_width : i32,
    pub screen_height : i32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            screen_width : 1280,
            screen_height : 720,
        }
    }
}