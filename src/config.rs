pub struct Config {
    pub screen_width : i32,
    pub screen_height : i32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            screen_width : 1920,
            screen_height : 1080,
        }
    }
    pub fn update(&mut self, new_screen_width : i32, new_screen_height : i32,){
        self.screen_width = new_screen_width;
        self.screen_height = new_screen_height;
    }
}